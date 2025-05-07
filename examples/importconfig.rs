#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "multipart")]
    {
        use keycloak::{KeycloakAdmin, KeycloakAdminToken};

        let url = std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://localhost:8080".into());
        let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "admin".into());
        let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "password".into());

        let client = reqwest::Client::new();
        let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;

        eprintln!("{:?}", admin_token);

        let admin = KeycloakAdmin::new(&url, admin_token, client);

        let config = admin
            .realm_identity_provider_import_config_post_form(
                "master",
                "saml".to_string(),
                include_bytes!("metadata.xml").to_vec(),
            )
            .await?;

        eprintln!("{:?}", config);
    }
    Ok(())
}
