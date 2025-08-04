use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Scopes</h4>
    /// Get client scopes belonging to the realm Returns a list of client scopes belonging to the realm
    pub fn client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_scopes_get(self.realm)
    }

    /// Create a new client scope Client Scope’s name must be unique!
    pub fn client_scopes_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_scopes_post(self.realm, body)
    }

    /// Get representation of the client scope
    pub fn client_scopes_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_get(self.realm, client_scope_id)
    }

    /// Update the client scope
    pub fn client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_put(self.realm, client_scope_id, body)
    }

    /// Delete the client scope
    pub fn client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_delete(self.realm, client_scope_id)
    }

    /// Get client scopes belonging to the realm Returns a list of client scopes belonging to the realm
    pub fn client_templates_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_templates_get(self.realm)
    }

    /// Create a new client scope Client Scope’s name must be unique!
    pub fn client_templates_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_templates_post(self.realm, body)
    }

    /// Get representation of the client scope
    pub fn client_templates_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_get(self.realm, client_scope_id)
    }

    /// Update the client scope
    pub fn client_templates_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_templates_with_client_scope_id_put(
            self.realm,
            client_scope_id,
            body,
        )
    }

    /// Delete the client scope
    pub fn client_templates_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_delete(self.realm, client_scope_id)
    }
}

// <h4>Client Scopes</h4>
#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Client Scopes</h4>
}
