/*!
# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements [Keycloak Admin REST API version 26.3.1](https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html).

### Feature flags

Default flags: `tags-all`.

- `rc`: use `Arc` for deserialization.
- `schemars`: add [schemars](https://crates.io/crates/schemars) support.
- `multipart`: add multipart support to reqwest, enabling extra methods in API.
- `tags-all`: activate all tags (resource groups) in REST API, it is default behavior. Disable default features and use individual `tag-xxx` features to activate only required resource groups. For a full list reference the [Cargo.toml](Cargo.toml).

## Usage

Requires Rust version >= `1.84.0`.

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "~26.3"
```

```rust, no_run
use keycloak::{
    types::*,
    {KeycloakAdmin, KeycloakAdminToken},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://localhost:8080".into());
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

    let response = admin
        .realm_users_post(
            "test",
            UserRepresentation {
                username: Some("user".into()),
                ..Default::default()
            },
        )
        .await?;

    eprintln!("{:?}", response.to_id());

    let users = admin
        .realm_users_get(
            "test", None, None, None, None, None, None, None, None, None, None, None, None, None,
            None,
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
        .realm_users_with_user_id_delete("test", id.as_str())
        .await?;

    admin.realm_delete("test").await?;

    Ok(())
}
```

## Version agreement

If we have `x.y.z` version of `keycloak`, our package version would be `x.y.(z * 100 + v)` there v is a minor
fix version to official `x.y.z` version.

Example: official version `13.0.1` is `13.0.100` for crate version. `13.0.102` means keycloak version `13.0.1` and minor fix version `2`.

## Update

To update current version use provided [update.ts](./update.ts) `deno` script:

```sh
deno run --allow-env=KEYCLOAK_RUST_VERSION,KEYCLOAK_VERSION,KEYCLOAK_RUST_MAJOR_VERSION --allow-read=Cargo.toml --allow-write=Cargo.toml,api/openapi.json,src/types.rs,src/rest/generated_rest.rs --allow-net=keycloak.org,www.keycloak.org --allow-run=cargo,gh,git,handlebars-magic update.ts
```

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
