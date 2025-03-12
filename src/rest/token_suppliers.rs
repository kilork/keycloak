use std::time::Instant;

use async_trait::async_trait;
use derivative::Derivative;

use crate::{error::TokenRequestError, types::TypeString, KeycloakError};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait KeycloakTokenSupplier {
    async fn get(&mut self, url: &str) -> Result<String, KeycloakError>;
}

#[derive(Clone)]
pub struct KeycloakServiceAccountAdminTokenRetriever {
    client_id: String,
    client_secret: String,
    realm: String,
    reqwest_client: reqwest::Client,
}

#[async_trait]
impl KeycloakTokenSupplier for KeycloakServiceAccountAdminTokenRetriever {
    async fn get(&mut self, url: &str) -> Result<String, KeycloakError> {
        let admin_token = self.acquire(url).await?;
        Ok(admin_token.access_token)
    }
}

impl KeycloakServiceAccountAdminTokenRetriever {
    /// Creates a token retriever for a [service account](https://www.keycloak.org/docs/latest/server_development/#authenticating-with-a-service-account)
    /// * `client_id` - The client id of a client with the following characteristics:
    ///                  1. Exists in the **master** realm
    ///                  2. `confidential` access type
    ///                  3. `Service Accounts` option is enabled
    /// * `client_secret` - The secret credential assigned to the given `client_id`
    /// * `client` - A reqwest Client to perform the token retrieval call
    pub fn create(client_id: &str, client_secret: &str, client: reqwest::Client) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            realm: "master".into(),
            reqwest_client: client,
        }
    }

    pub fn create_with_custom_realm(
        client_id: &str,
        client_secret: &str,
        realm: &str,
        client: reqwest::Client,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            realm: realm.into(),
            reqwest_client: client,
        }
    }

    pub async fn acquire(&self, url: &str) -> Result<KeycloakAdminToken, KeycloakError> {
        let realm = &self.realm;
        let response = self
            .reqwest_client
            .post(format!(
                "{url}/realms/{realm}/protocol/openid-connect/token",
            ))
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await?;
        Ok(error_check(response).await?.json().await?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeycloakAdminToken {
    access_token: String,
    expires_in: usize,
    #[serde(rename = "not-before-policy")]
    not_before_policy: Option<usize>,
    refresh_expires_in: Option<usize>,
    refresh_token: Option<String>,
    scope: String,
    session_state: Option<String>,
    token_type: String,
}

#[async_trait]
impl KeycloakTokenSupplier for KeycloakAdminToken {
    async fn get(&mut self, _url: &str) -> Result<String, KeycloakError> {
        Ok(self.access_token.clone())
    }
}

impl KeycloakAdminToken {
    pub async fn acquire(
        url: &str,
        username: &str,
        password: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakAdminToken, KeycloakError> {
        Self::acquire_custom_realm(
            url,
            username,
            password,
            "master",
            "admin-cli",
            "password",
            client,
        )
        .await
    }

    pub async fn acquire_custom_realm(
        url: &str,
        username: &str,
        password: &str,
        realm: &str,
        client_id: &str,
        grant_type: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakAdminToken, KeycloakError> {
        let response = client
            .post(format!(
                "{url}/realms/{realm}/protocol/openid-connect/token",
            ))
            .form(&[
                ("username", username),
                ("password", password),
                ("client_id", client_id),
                ("grant_type", grant_type),
            ])
            .send()
            .await?;
        Ok(error_check(response).await?.json().await?)
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq)]
pub struct KeycloakTokenWithRefresh {
    pub(crate) token_data: KeycloakAdminToken,

    #[derivative(PartialEq = "ignore")]
    pub(crate) url: String,

    #[derivative(PartialEq = "ignore")]
    pub(crate) realm: String,

    #[derivative(PartialEq = "ignore")]
    pub(crate) username: String,

    #[derivative(PartialEq = "ignore")]
    pub(crate) password: String,

    #[derivative(PartialEq = "ignore")]
    pub(crate) client: reqwest::Client,

    #[derivative(PartialEq = "ignore")]
    pub(crate) acquired_at: Instant,
}

#[async_trait]
impl KeycloakTokenSupplier for KeycloakTokenWithRefresh {
    async fn get(&mut self, _url: &str) -> Result<String, KeycloakError> {
        if self.is_access_token_valid() {
            return Ok(self.token_data.access_token.clone());
        }

        if self.is_refresh_token_valid() {
            self.refresh_with_token().await?;
            return Ok(self.token_data.access_token.clone());
        }

        self.refresh_with_auth().await?;
        Ok(self.token_data.access_token.clone())
    }
}

impl KeycloakTokenWithRefresh {
    pub fn is_access_token_valid(&self) -> bool {
        let access_expires_in = self.token_data.expires_in;
        let time_since_access_token_generated = (Instant::now() - self.acquired_at).as_secs();

        if let Ok(val) = usize::try_from(time_since_access_token_generated) {
            val < access_expires_in
        } else {
            false
        }
    }

    pub fn is_refresh_token_valid(&self) -> bool {
        let refresh_expires_in = match self
            .token_data
            .refresh_expires_in
            .as_ref()
            .ok_or(TokenRequestError::get_illegal_state(
            "Called KeycloakAdminToken::refresh_with_token without configured refresh_expires_in",
        )) {
            Ok(refresh_expires_in) => refresh_expires_in,
            Err(_) => return false,
        };

        let time_since_request_token_generated = (Instant::now() - self.acquired_at).as_secs();

        if let Ok(val) = usize::try_from(time_since_request_token_generated) {
            val < *refresh_expires_in
        } else {
            false
        }
    }

    pub async fn refresh_with_token(&mut self) -> Result<(), KeycloakError> {
        let refresh_token =
            self.token_data
                .refresh_token
                .as_ref()
                .ok_or(TokenRequestError::get_illegal_state(
                "Called KeycloakAdminToken::refresh_with_token without configured refresh_token",
            ))?;

        if !self.is_refresh_token_valid() {
            return Err(TokenRequestError::RefreshTokenInvalid.into());
        }

        let response = self
            .client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                &self.url, &self.realm
            ))
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", "admin-cli"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await
            .map_err(KeycloakError::from)?;

        let responded_token: KeycloakAdminToken = error_check(response)
            .await?
            .json()
            .await
            .map_err(KeycloakError::from)?;

        self.token_data = responded_token;
        self.acquired_at = Instant::now();

        Ok(())
    }

    pub async fn refresh_with_auth(&mut self) -> Result<(), KeycloakError> {
        let responded_token: KeycloakAdminToken = KeycloakAdminToken::acquire_custom_realm(
            &self.url,
            &self.username,
            &self.password,
            &self.realm,
            "admin-cli",
            "password",
            &self.client,
        )
        .await?;

        self.token_data = responded_token;
        self.acquired_at = Instant::now();

        Ok(())
    }

    pub async fn acquire(
        url: &str,
        username: &str,
        password: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakTokenWithRefresh, KeycloakError> {
        Self::acquire_custom_realm(
            url,
            username,
            password,
            "master",
            "admin-cli",
            "password",
            client,
        )
        .await
    }

    pub async fn acquire_custom_realm(
        url: &str,
        username: &str,
        password: &str,
        realm: &str,
        client_id: &str,
        grant_type: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakTokenWithRefresh, KeycloakError> {
        let token_data: KeycloakAdminToken = KeycloakAdminToken::acquire_custom_realm(
            url, username, password, realm, client_id, grant_type, client,
        )
        .await?;

        Ok(Self {
            token_data,
            url: url.to_string(),
            realm: realm.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            client: client.clone(),
            acquired_at: Instant::now(),
        })
    }
}

pub(crate) async fn error_check(
    response: reqwest::Response,
) -> Result<reqwest::Response, KeycloakError> {
    if !response.status().is_success() {
        let status = response.status().into();
        let text = response.text().await?;
        return Err(KeycloakError::HttpFailure {
            status,
            body: serde_json::from_str(&text).ok(),
            text,
        });
    }

    Ok(response)
}

pub(crate) fn to_id(response: reqwest::Response) -> Option<TypeString> {
    response
        .headers()
        .get(reqwest::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split('/').last())
        .map(From::from)
}
