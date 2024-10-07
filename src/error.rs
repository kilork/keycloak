use std::{borrow::Cow, error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum KeycloakError {
    ReqwestFailure(reqwest::Error),
    HttpFailure {
        status: u16,
        body: Option<KeycloakHttpError>,
        text: String,
    },
}

impl From<reqwest::Error> for KeycloakError {
    fn from(value: reqwest::Error) -> Self {
        KeycloakError::ReqwestFailure(value)
    }
}

impl Error for KeycloakError {}

impl Display for KeycloakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeycloakError::ReqwestFailure(e) => write!(f, "keycloak error (network): {e}"),
            KeycloakError::HttpFailure { status, body, text } => write!(
                f,
                "keycloak error (rest): {status} {}",
                body.as_ref()
                    .and_then(|e| e.message())
                    .unwrap_or_else(|| Cow::from(text))
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeycloakHttpError {
    pub error: Option<String>,
    pub error_description: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

impl KeycloakHttpError {
    pub fn message(&self) -> Option<Cow<'_, str>> {
        self.error_message
            .as_deref()
            .map(Cow::from)
            .or_else(|| {
                self.error
                    .as_deref()
                    .map(|error| {
                        format!(
                            "{} [{error}]",
                            self.error_description.as_deref().unwrap_or("null")
                        )
                    })
                    .map(Cow::from)
            })
            .or_else(|| self.error_description.as_deref().map(Cow::from))
    }
}
