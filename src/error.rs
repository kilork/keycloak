use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum TokenRequestError {
    #[error("Illegal State: {msg}")]
    IllegalState { msg: String },

    #[error("Tried to refresh the jwt with an invalid refresh token")]
    RefreshTokenInvalid,
}

impl TokenRequestError {
    pub(crate) fn get_illegal_state(msg: impl Into<String>) -> Self {
        Self::IllegalState { msg: msg.into() }
    }
}

#[derive(ThisError, Debug)]
pub enum KeycloakError {
    #[error("keycloak token request error: {0}")]
    TokenRequestFailure(#[from] TokenRequestError),

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
