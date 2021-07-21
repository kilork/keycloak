use crate::{types::*, KeycloakError};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod rest;

pub struct KeycloakAdmin {
    url: String,
    client: reqwest::Client,
    admin_token: KeycloakAdminToken,
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

impl KeycloakAdminToken {
    pub async fn get(&self, _url: &str) -> Result<String, KeycloakError> {
        Ok(self.access_token.clone())
    }

    pub async fn acquire(
        url: &str,
        username: &str,
        password: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakAdminToken, KeycloakError> {
        let response = client
            .post(&format!(
                "{}/auth/realms/master/protocol/openid-connect/token",
                url
            ))
            .form(&json!({
                "username": username,
                "password": password,
                "client_id": "admin-cli",
                "grant_type": "password"
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
