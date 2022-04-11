use async_trait::async_trait;

use crate::{types::*, KeycloakError};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod rest;

pub struct KeycloakAdmin<TS: KeycloakTokenSupplier = KeycloakAdminToken> {
    url: String,
    client: reqwest::Client,
    admin_token: TS,
}

#[async_trait]
pub trait KeycloakTokenSupplier {
    async fn get(&self, url: &str) -> Result<String, KeycloakError>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeycloakServiceAccountAdminToken {
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
impl KeycloakTokenSupplier for KeycloakServiceAccountAdminToken {
    async fn get(&self, _url: &str) -> Result<String, KeycloakError> {
        Ok(self.access_token.clone())
    }
}

impl KeycloakServiceAccountAdminToken {
    /// Acquire a token for a [service account](https://www.keycloak.org/docs/latest/server_development/#authenticate-with-a-service-account)
    /// * `url` - Protocol, hostname, and port of Keycloak such as `http://localhost:8080`
    /// * `client_id` - The client id of a client with the following characteristics:
    ///                  1. Exists in the **master** realm
    ///                  2. `confidential` access type
    ///                  3. `Service Accounts` option is enabled
    /// * `client_secret` - The secret credential assigned to the given `client_id`
    /// * `client` - A reqwest Client to perform the token retrieval call
    pub async fn acquire(
        url: &str,
        client_id: &str,
        client_secret: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakAdminToken, KeycloakError> {
        let response = client
            .post(&format!(
                "{}/auth/realms/master/protocol/openid-connect/token",
                url,
            ))
            .form(&json!({
                "client_id": client_id,
                "client_secret": client_secret,
                "grant_type": "client_credentials"
            }))
            .send()
            .await?;
        Ok(error_check(response).await?.json().await?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
            .post(&format!(
                "{}/auth/realms/{}/protocol/openid-connect/token",
                url, realm
            ))
            .form(&json!({
                "username": username,
                "password": password,
                "client_id": client_id,
                "grant_type": grant_type
            }))
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

impl KeycloakAdmin {
    pub fn new(url: &str, admin_token: KeycloakAdminToken, client: reqwest::Client) -> Self {
        Self {
            url: url.into(),
            client,
            admin_token,
        }
    }
}
