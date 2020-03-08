use crate::{types::*, KeycloakError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;

mod rest;

pub struct KeycloakAdmin<'a> {
    url: String,
    client: reqwest::Client,
    admin_token: KeycloakAdminToken<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeycloakAdminToken<'a> {
    access_token: Cow<'a, str>,
    expires_in: usize,
    #[serde(rename = "not-before-policy")]
    not_before_policy: Option<usize>,
    refresh_expires_in: Option<usize>,
    refresh_token: Option<Cow<'a, str>>,
    scope: Cow<'a, str>,
    session_state: Option<Cow<'a, str>>,
    token_type: Cow<'a, str>,
}

impl<'a> KeycloakAdminToken<'a> {
    pub async fn get(&self, _url: &str) -> Result<Cow<'_, str>, KeycloakError> {
        Ok(self.access_token.clone())
    }

    pub async fn acquire(
        url: &str,
        username: &str,
        password: &str,
        client: &reqwest::Client,
    ) -> Result<KeycloakAdminToken<'a>, KeycloakError> {
        Ok(client
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
            .await?
            .json()
            .await?)
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

impl<'a> KeycloakAdmin<'a> {
    pub fn new(url: &str, admin_token: KeycloakAdminToken<'a>, client: reqwest::Client) -> Self {
        Self {
            url: url.into(),
            client,
            admin_token,
        }
    }
}
