use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Initial Access</h4>
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `GET /admin/realms/{realm}/clients-initial-access`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclients_initial_access>
    pub fn clients_initial_access_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientInitialAccessPresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_clients_initial_access_get(self.realm)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclients_initial_access>
    pub fn clients_initial_access_post(
        &'a self,
        body: ClientInitialAccessCreatePresentation,
    ) -> impl Future<Output = Result<ClientInitialAccessCreatePresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_initial_access_post(self.realm, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclients_initial_accessid>
    pub fn clients_initial_access_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_initial_access_with_id_delete(self.realm, id)
    }
}
