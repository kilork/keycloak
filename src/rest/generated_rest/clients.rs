use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Clients</h4>

    /// Get clients belonging to the realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_id`: filter by clientId
    /// - `first`: the first result
    /// - `max`: the max results to return
    /// - `q`
    /// - `search`: whether this is a search query or a getClientById query
    /// - `viewable_only`: filter clients that cannot be viewed in full by admin
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclients>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_get(
        &self,
        realm: &str,
        client_id: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<bool>,
        viewable_only: Option<bool>,
    ) -> Result<TypeVec<ClientRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/clients", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("clientId", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = viewable_only {
            builder = builder.query(&[("viewableOnly", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new client Client’s client_id must be unique!
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclients>
    pub async fn realm_clients_post(
        &self,
        realm: &str,
        body: ClientRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/clients", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get representation of the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}`
    pub async fn realm_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}`
    pub async fn realm_clients_with_client_uuid_put(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ClientRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}`
    pub async fn realm_clients_with_client_uuid_delete(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the client secret
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/client-secret`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidclient_secret>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/client-secret`
    pub async fn realm_clients_with_client_uuid_client_secret_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/client-secret",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new secret for the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/client-secret`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidclient_secret>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/client-secret`
    pub async fn realm_clients_with_client_uuid_client_secret_post(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/client-secret",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the rotated client secret
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/client-secret/rotated`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidclient_secretrotated>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/client-secret/rotated`
    pub async fn realm_clients_with_client_uuid_client_secret_rotated_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/client-secret/rotated",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Invalidate the rotated secret for the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/client-secret/rotated`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidclient_secretrotated>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/client-secret/rotated`
    pub async fn realm_clients_with_client_uuid_client_secret_rotated_delete(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/client-secret/rotated",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get default client scopes.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuiddefault_client_scopes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/default-client-scopes`
    pub async fn realm_clients_with_client_uuid_default_client_scopes_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/default-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuiddefault_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/default-client-scopes/{clientScopeId}`
    pub async fn realm_clients_with_client_uuid_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/default-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuiddefault_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/default-client-scopes/{clientScopeId}`
    pub async fn realm_clients_with_client_uuid_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/default-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Create JSON with payload of example access token
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `audience`
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/generate-example-access-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesgenerate_example_access_token>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/generate-example-access-token`
    pub async fn realm_clients_with_client_uuid_evaluate_scopes_generate_example_access_token_get(
        &self,
        realm: &str,
        client_uuid: &str,
        audience: Option<String>,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<AccessToken, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/generate-example-access-token",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = audience {
            builder = builder.query(&[("audience", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = user_id {
            builder = builder.query(&[("userId", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create JSON with payload of example id token
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `audience`
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/generate-example-id-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesgenerate_example_id_token>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/generate-example-id-token`
    pub async fn realm_clients_with_client_uuid_evaluate_scopes_generate_example_id_token_get(
        &self,
        realm: &str,
        client_uuid: &str,
        audience: Option<String>,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<IDToken, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/generate-example-id-token",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = audience {
            builder = builder.query(&[("audience", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = user_id {
            builder = builder.query(&[("userId", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create JSON with payload of example user info
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/generate-example-userinfo`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesgenerate_example_userinfo>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/generate-example-userinfo`
    pub async fn realm_clients_with_client_uuid_evaluate_scopes_generate_example_userinfo_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<Value, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/generate-example-userinfo",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = user_id {
            builder = builder.query(&[("userId", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return list of all protocol mappers, which will be used when generating tokens issued for particular client.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/protocol-mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesprotocol_mappers>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/protocol-mappers`
    pub async fn realm_clients_with_client_uuid_evaluate_scopes_protocol_mappers_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope: Option<String>,
    ) -> Result<TypeVec<ProtocolMapperEvaluationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/protocol-mappers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective scope mapping of all roles of particular role container, which this client is defacto allowed to have in the accessToken issued for him.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/scope-mappings/{role_container_id}/granted`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesscope_mappingsrolecontaineridgranted>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/scope-mappings/{roleContainerId}/granted`
    pub async fn realm_clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_container_id = p(role_container_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/scope-mappings/{role_container_id}/granted",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get roles, which this client doesn't have scope for and can't have them in the accessToken issued for him.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/scope-mappings/{role_container_id}/not-granted`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesscope_mappingsrolecontaineridnot_granted>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/scope-mappings/{roleContainerId}/not-granted`
    pub async fn realm_clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_container_id = p(role_container_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/evaluate-scopes/scope-mappings/{role_container_id}/not-granted",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `provider_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/installation/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidinstallationprovidersproviderid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/installation/providers/{providerId}`
    pub async fn realm_clients_with_client_uuid_installation_providers_with_provider_id_get(
        &self,
        realm: &str,
        client_uuid: &str,
        provider_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let provider_id = p(provider_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/installation/providers/{provider_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/management/permissions`
    pub async fn realm_clients_with_client_uuid_management_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/management/permissions`
    pub async fn realm_clients_with_client_uuid_management_permissions_put(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a cluster node with the client Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle by sending registration request to Keycloak
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/nodes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidnodes>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/nodes`
    pub async fn realm_clients_with_client_uuid_nodes_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/nodes",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Unregister a cluster node from the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `node`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/nodes/{node}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidnodesnode>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/nodes/{node}`
    pub async fn realm_clients_with_client_uuid_nodes_with_node_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        node: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let node = p(node);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/nodes/{node}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get application offline session count Returns a number of offline user sessions associated with this client { "count": number }
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/offline-session-count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidoffline_session_count>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/offline-session-count`
    pub async fn realm_clients_with_client_uuid_offline_session_count_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/offline-session-count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get offline sessions for client Returns a list of offline user sessions associated with this client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/offline-sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidoffline_sessions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/offline-sessions`
    pub async fn realm_clients_with_client_uuid_offline_sessions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserSessionRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/offline-sessions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get optional client scopes.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidoptional_client_scopes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes`
    pub async fn realm_clients_with_client_uuid_optional_client_scopes_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/optional-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidoptional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes/{clientScopeId}`
    pub async fn realm_clients_with_client_uuid_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidoptional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes/{clientScopeId}`
    pub async fn realm_clients_with_client_uuid_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Push the client's revocation policy to its admin URL If the client has an admin URL, push revocation policy to it.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidpush_revocation>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/push-revocation`
    pub async fn realm_clients_with_client_uuid_push_revocation_post(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/push-revocation",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new registration access token for the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/registration-access-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidregistration_access_token>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/registration-access-token`
    pub async fn realm_clients_with_client_uuid_registration_access_token_post(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/registration-access-token",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get a user dedicated to the service account
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/service-account-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidservice_account_user>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/service-account-user`
    pub async fn realm_clients_with_client_uuid_service_account_user_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<UserRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/service-account-user",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get application session count Returns a number of user sessions associated with this client { "count": number }
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/session-count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidsession_count>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/session-count`
    pub async fn realm_clients_with_client_uuid_session_count_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/session-count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Test if registered cluster nodes are available Tests availability by sending 'ping' request to all cluster nodes.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/test-nodes-available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidtest_nodes_available>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/test-nodes-available`
    pub async fn realm_clients_with_client_uuid_test_nodes_available_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/test-nodes-available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get user sessions for client Returns a list of user sessions associated with this client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/user-sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuiduser_sessions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/user-sessions`
    pub async fn realm_clients_with_client_uuid_user_sessions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserSessionRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/user-sessions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 221
