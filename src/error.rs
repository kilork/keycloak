use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum KeycloakError {
    #[error("keycloak error (network): {0}")]
    ReqwestFailure(#[from] reqwest::Error),

    #[error("keycloak error (rest): {status} {body:?} {text}")]
    HttpFailure {
        status: u16,
        body: Option<KeycloakHttpError>,
        text: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeycloakHttpError {
    pub error: Option<String>,
    pub error_description: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}
