# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements Keycloak Admin REST API version 10.

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "10"
```

```rust
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use keycloak::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "http://localhost:9080";
    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(url, "admin", "password", &client).await?;

    eprintln!("{:?}", admin_token);

    let admin = KeycloakAdmin::new(url, admin_token, client);

    admin
        .users_post(
            "test",
            UserRepresentation {
                username: Some("user".into()),
                ..Default::default()
            },
        )
        .await?;

    let users = admin
        .users_get("test", None, None, None, None, None, None, None, None)
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

    admin.users_delete("test", id.as_str()).await?;

    Ok(())
}
```
