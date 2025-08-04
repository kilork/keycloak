#[cfg(all(feature = "builder", feature = "resource"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use keycloak::{types::*, KeycloakAdmin, KeycloakAdminToken};

    const REALM: &str = "resource";

    let url = std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://localhost:8080".into());
    let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "admin".into());
    let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "password".into());

    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;

    eprintln!("{admin_token:?}");

    let admin = KeycloakAdmin::new(&url, admin_token, client);

    admin
        .post(RealmRepresentation {
            realm: Some(REALM.into()),
            ..Default::default()
        })
        .await?;

    let realm = admin.realm(REALM);

    let response = realm
        .users_post(UserRepresentation {
            username: Some("user".into()),
            ..Default::default()
        })
        .await?;

    eprintln!("{:?}", response.to_id());

    let users = realm.users_get().username("user".to_string()).await?;

    eprintln!("{users:?}");

    let id = users
        .iter()
        .find(|u| u.username == Some("user".into()))
        .unwrap()
        .id
        .as_ref()
        .unwrap()
        .to_string();

    realm.users_with_user_id_delete(id.as_str()).await?;

    realm.delete().await?;

    Ok(())
}
