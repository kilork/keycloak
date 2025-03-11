use std::sync::Arc;

use crate::{types::*, KeycloakError};

mod generated_rest;
mod token_suppliers;
mod url_enc;

pub use token_suppliers::*;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct KeycloakAdmin<TS: KeycloakTokenSupplier = KeycloakAdminToken> {
    pub(crate) url: String,
    pub(crate) client: reqwest::Client,
    pub(crate) token_supplier: Arc<Mutex<TS>>,
}

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    pub fn new(url: &str, token_supplier: TS, client: reqwest::Client) -> Self {
        Self {
            url: url.into(),
            client,
            token_supplier: Arc::new(Mutex::new(token_supplier)),
        }
    }
}
