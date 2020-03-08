use keycloak::types::*;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use serde_json::{json, Value};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:9080";
    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(url, "admin", "password", &client).await?;

    eprintln!("{:?}", admin_token);

    let admin = KeycloakAdmin::new(url, admin_token, client);

    let user = admin
        .users_create(
            "test",
            UserRepresentation {
                ..Default::default()
            },
        )
        .await?;

    eprintln!("{:?}", user);

    Ok(())
}
