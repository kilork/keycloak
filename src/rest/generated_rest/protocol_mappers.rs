use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Protocol Mappers</h4>

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/add-models`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/protocol/{protocol}`
    pub async fn realm_client_scopes_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        protocol: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let protocol = p(protocol);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/protocol/{protocol}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/add-models`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        client_scope_id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/protocol/{protocol}`
    pub async fn realm_client_templates_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        client_scope_id: &str,
        protocol: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let protocol = p(protocol);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/protocol/{protocol}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/add-models`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/add-models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_models_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_models_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/protocol/{protocol}`
    pub async fn realm_clients_with_client_uuid_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        client_uuid: &str,
        protocol: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let protocol = p(protocol);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/protocol/{protocol}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 235
