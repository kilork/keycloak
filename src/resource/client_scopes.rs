use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Scopes</h4>
    /// Get client scopes belonging to the realm Returns a list of client scopes belonging to the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /admin/realms/{realm}/client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopes>
    pub fn client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_scopes_get(self.realm)
    }

    /// Create a new client scope Client Scope’s name must be unique!
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Scopes`
    ///
    /// `POST /admin/realms/{realm}/client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopes>
    pub fn client_scopes_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_scopes_post(self.realm, body)
    }

    /// Get representation of the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}`
    pub fn client_scopes_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_get(self.realm, client_scope_id)
    }

    /// Update the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Scopes`
    ///
    /// `PUT /admin/realms/{realm}/client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclient_scopesclient_scope_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-scopes/{client-scope-id}`
    pub fn client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_put(self.realm, client_scope_id, body)
    }

    /// Delete the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Scopes`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}`
    pub fn client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_delete(self.realm, client_scope_id)
    }

    /// Get client scopes belonging to the realm Returns a list of client scopes belonging to the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /admin/realms/{realm}/client-templates`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templates>
    pub fn client_templates_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_templates_get(self.realm)
    }

    /// Create a new client scope Client Scope’s name must be unique!
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Scopes`
    ///
    /// `POST /admin/realms/{realm}/client-templates`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templates>
    pub fn client_templates_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_templates_post(self.realm, body)
    }

    /// Get representation of the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}`
    pub fn client_templates_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_get(self.realm, client_scope_id)
    }

    /// Update the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Scopes`
    ///
    /// `PUT /admin/realms/{realm}/client-templates/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclient_templatesclient_scope_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-templates/{client-scope-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Scopes`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}`
    pub fn client_templates_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_delete(self.realm, client_scope_id)
    }
}
