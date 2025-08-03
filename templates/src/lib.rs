/*!
{{ replace ( render ( read_to_str "templates/README.md" ) ) "```rust" "```rust, no_run" }}
*/

#[cfg(feature = "resource")]
pub mod resource;
pub mod types;

mod error;
mod rest;

pub use error::KeycloakError;
pub use rest::{
    DefaultResponse, KeycloakAdmin, KeycloakAdminToken, KeycloakRealmAdmin,
    KeycloakRealmAdminMethod, KeycloakServiceAccountAdminTokenRetriever, KeycloakTokenSupplier,
};
