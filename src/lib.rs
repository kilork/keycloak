/*!
# keycloak description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "0.1"
```

*/

pub mod types;

mod error;
mod rest;

pub use error::KeycloakError;
pub use rest::{KeycloakAdmin, KeycloakAdminToken};
