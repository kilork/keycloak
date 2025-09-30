use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Scope Mappings</h4>

    /// Get all scope mappings for the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings`
    #[deprecated]
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove client-level roles from the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclientavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}/available`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclientcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}/composite`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/clients/{client}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a set of realm-level roles to the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a set of realm-level roles from the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that are available to attach to this client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm/available`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm/composite`
    pub async fn realm_client_scopes_with_client_scope_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/scope-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get all scope mappings for the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings`
    #[deprecated]
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove client-level roles from the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclientavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}/available`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclientcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}/composite`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let client = p(client);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/clients/{client}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a set of realm-level roles to the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a set of realm-level roles from the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that are available to attach to this client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm/available`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm/composite`
    pub async fn realm_client_templates_with_client_scope_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/scope-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get all scope mappings for the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings`
    #[deprecated]
    pub async fn realm_clients_with_client_uuid_scope_mappings_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclient>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}`
    pub async fn realm_clients_with_client_uuid_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        client_uuid: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client = p(client);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclient>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}`
    pub async fn realm_clients_with_client_uuid_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        client_uuid: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client = p(client);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove client-level roles from the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}`
    pub async fn realm_clients_with_client_uuid_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client = p(client);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclientavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}/available`
    pub async fn realm_clients_with_client_uuid_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        client_uuid: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client = p(client);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclientcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}/composite`
    pub async fn realm_clients_with_client_uuid_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        client_uuid: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client = p(client);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/clients/{client}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    pub async fn realm_clients_with_client_uuid_scope_mappings_realm_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a set of realm-level roles to the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidscope_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    pub async fn realm_clients_with_client_uuid_scope_mappings_realm_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a set of realm-level roles from the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidscope_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    pub async fn realm_clients_with_client_uuid_scope_mappings_realm_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that are available to attach to this client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm/available`
    pub async fn realm_clients_with_client_uuid_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm/composite`
    pub async fn realm_clients_with_client_uuid_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        client_uuid: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/scope-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 226
