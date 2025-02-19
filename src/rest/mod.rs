use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{types::*, KeycloakError};

mod default_response;
mod generated_rest;
mod url_enc;

pub use default_response::DefaultResponse;

pub struct KeycloakAdmin<TS: KeycloakTokenSupplier = KeycloakAdminToken> {
    url: String,
    client: reqwest::Client,
    token_supplier: TS,
}

#[async_trait]
pub trait KeycloakTokenSupplier {
    async fn get(&self, url: &str) -> Result<String, KeycloakError>;
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
    async fn get(&self, url: &str) -> Result<String, KeycloakError> {
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
    async fn get(&self, _url: &str) -> Result<String, KeycloakError> {
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

async fn error_check(response: reqwest::Response) -> Result<reqwest::Response, KeycloakError> {
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

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    pub fn new(url: &str, token_supplier: TS, client: reqwest::Client) -> Self {
        Self {
            url: url.into(),
            client,
            token_supplier,
        }
    }
}
