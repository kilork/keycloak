/*!
{{ replace ( render ( read_to_str "templates/README.md" ) ) "```rust" "```rust#ignore" }}
*/

pub mod types;

mod error;
mod rest;

pub use error::KeycloakError;
pub use rest::{KeycloakAdmin, KeycloakAdminToken, KeycloakTokenSupplier};
