use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum KeycloakError {
    ReqwestFailure(reqwest::Error),
    HttpFailure {
        status: u16,
        body: Option<KeycloakHttpError>,
        text: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeycloakHttpError {
    pub error: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

impl From<reqwest::Error> for KeycloakError {
    fn from(value: reqwest::Error) -> Self {
        KeycloakError::ReqwestFailure(value)
    }
}

impl Error for KeycloakError {
    fn description(&self) -> &str {
        "keycloak error"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Display for KeycloakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "keycloak error")
    }
}
