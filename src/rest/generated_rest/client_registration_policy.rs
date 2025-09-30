use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Client Registration Policy</h4>

    /// Base path for retrieve providers with the configProperties properly filled
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Registration Policy`
    ///
    /// `GET /admin/realms/{realm}/client-registration-policy/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_registration_policyproviders>
    pub async fn realm_client_registration_policy_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ComponentTypeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-registration-policy/providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 246
