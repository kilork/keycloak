/*!
{{ replace ( render ( read_to_str "templates/README.md" ) ) "```rust" "```rust, no_run" }}
*/

pub mod types;

mod error;
mod rest;

pub use error::KeycloakError;
pub use rest::{
    KeycloakAdmin, KeycloakAdminToken, KeycloakServiceAccountAdminTokenRetriever,
    KeycloakTokenSupplier,
};