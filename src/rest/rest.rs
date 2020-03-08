use super::*;

impl<'a> KeycloakAdmin<'a> {
    pub async fn users_create(
        &self,
        realm: &str,
        rep: UserRepresentation<'_>,
    ) -> Result<String, KeycloakError> {
        let response = self
            .client
            .post(&format!("{}/auth/admin/realms/{}/users", self.url, realm))
            .bearer_auth(self.admin_token.access_token.to_string())
            .json(&rep)
            .send()
            .await?;
        Ok(error_check(response).await?.text().await?)
    }
}
