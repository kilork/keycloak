/*!
# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements [Keycloak Admin REST API version 17.0](https://www.keycloak.org/docs-api/17.0/rest-api/index.html).

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "17.0"
```

```rust#ignore
use keycloak::{
    types::*,
    {KeycloakAdmin, KeycloakAdminToken},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://localhost:9080".into());
    let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "admin".into());
    let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "password".into());
    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;

    eprintln!("{:?}", admin_token);

    let admin = KeycloakAdmin::new(&url, admin_token, client);

    admin
        .post(RealmRepresentation {
            realm: Some("test".into()),
            ..Default::default()
        })
        .await?;

    admin
        .realm_users_post(
            "test",
            UserRepresentation {
                username: Some("user".into()),
                ..Default::default()
            },
        )
        .await?;

    let users = admin
        .realm_users_get(
            "test", None, None, None, None, None, None, None, None, None, None, None, None, None,
        )
        .await?;

    eprintln!("{:?}", users);

    let id = users
        .iter()
        .find(|u| u.username == Some("user".into()))
        .unwrap()
        .id
        .as_ref()
        .unwrap()
        .to_string();

    admin
        .realm_users_with_id_delete("test", id.as_str())
        .await?;

    admin.realm_delete("test").await?;

    Ok(())
}
```

## Version agreement

If we have `x.y.z` version of `keycloak`, our package version would be `x.y.(z * 100 + v)` there v is a minor
fix version to official `x.y.z` version.

Example: official version `13.0.1` is `13.0.100` for crate version. `13.0.102` means keycloak version `13.0.1` and minor fix version `2`.
*/

pub mod types;

mod error;
mod rest;

pub use error::KeycloakError;
pub use rest::{KeycloakAdmin, KeycloakAdminToken, KeycloakTokenSupplier, KeycloakServiceAccountAdminTokenRetriever};
