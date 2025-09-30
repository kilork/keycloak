use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
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
    pub async fn realm_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/client-scopes", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_client_scopes_post(
        &self,
        realm: &str,
        body: ClientScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/client-scopes", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    pub async fn realm_client_scopes_with_client_scope_id_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<ClientScopeRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: ClientScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    pub async fn realm_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    pub async fn realm_client_templates_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_client_templates_post(
        &self,
        realm: &str,
        body: ClientScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-templates",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    pub async fn realm_client_templates_with_client_scope_id_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<ClientScopeRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_client_templates_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: ClientScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    pub async fn realm_client_templates_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }
}
// not all paths processed
// left 243
