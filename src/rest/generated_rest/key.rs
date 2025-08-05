use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Key</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Key`
    ///
    /// `GET /admin/realms/{realm}/keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmkeys>
    pub async fn realm_keys_get(
        &self,
        realm: &str,
    ) -> Result<KeysMetadataRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/keys", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 246
