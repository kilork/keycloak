use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tok: Value = reqwest::Client::new()
        .post("http://localhost:9080/auth/realms/master/protocol/openid-connect/token")
        .form(&json!({
            "username": "admin",
            "password": "password",
            "client_id": "admin-cli",
            "grant_type": "password"
        }))
        .send()
        .await?
        .json()
        .await?;
    eprintln!("{:?}", tok);
    Ok(())
}
