use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Client Initial Access</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `GET /admin/realms/{realm}/clients-initial-access`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclients_initial_access>
    pub async fn realm_clients_initial_access_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientInitialAccessPresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients-initial-access",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new initial access token.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `POST /admin/realms/{realm}/clients-initial-access`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclients_initial_access>
    pub async fn realm_clients_initial_access_post(
        &self,
        realm: &str,
        body: ClientInitialAccessCreatePresentation,
    ) -> Result<ClientInitialAccessCreatePresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients-initial-access",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `DELETE /admin/realms/{realm}/clients-initial-access/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclients_initial_accessid>
    pub async fn realm_clients_initial_access_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients-initial-access/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }
}
// not all paths processed
// left 245
