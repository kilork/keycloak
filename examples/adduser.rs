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
        .realm_users_with_id_delete("test", id.as_str())
        .await?;

    admin.realm_delete("test").await?;

    Ok(())
}
