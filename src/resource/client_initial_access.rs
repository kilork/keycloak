use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Initial Access</h4>
    pub fn clients_initial_access_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientInitialAccessPresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_clients_initial_access_get(self.realm)
    }

    /// Create a new initial access token.
    pub fn clients_initial_access_post(
        &'a self,
        body: ClientInitialAccessCreatePresentation,
    ) -> impl Future<Output = Result<ClientInitialAccessCreatePresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_initial_access_post(self.realm, body)
    }

    pub fn clients_initial_access_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_initial_access_with_id_delete(self.realm, id)
    }
}

// <h4>Client Initial Access</h4>
