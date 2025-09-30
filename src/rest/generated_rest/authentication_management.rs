use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Authentication Management</h4>

    /// Get authenticator providers Returns a stream of authenticator providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/authenticator-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationauthenticator_providers>
    pub async fn realm_authentication_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/authenticator-providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client authenticator providers Returns a stream of client authenticator providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/client-authenticator-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationclient_authenticator_providers>
    pub async fn realm_authentication_client_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/client-authenticator-providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create new authenticator configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationconfig>
    #[deprecated]
    pub async fn realm_authentication_config_post(
        &self,
        realm: &str,
        body: AuthenticatorConfigRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get authenticator provider's configuration description
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `provider_id`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/config-description/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationconfig_descriptionproviderid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/config-description/{providerId}`
    pub async fn realm_authentication_config_description_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<AuthenticatorConfigInfoRepresentation, KeycloakError> {
        let realm = p(realm);
        let provider_id = p(provider_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/config-description/{provider_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get authenticator configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationconfigid>
    pub async fn realm_authentication_config_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticatorConfigRepresentation, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/config/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update authenticator configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmauthenticationconfigid>
    pub async fn realm_authentication_config_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: AuthenticatorConfigRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/authentication/config/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete authenticator configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmauthenticationconfigid>
    pub async fn realm_authentication_config_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/authentication/config/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Add new authentication execution
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutions>
    pub async fn realm_authentication_executions_post(
        &self,
        realm: &str,
        body: AuthenticationExecutionRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/executions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get Single Execution
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/executions/{execution_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationexecutionsexecutionid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/executions/{executionId}`
    pub async fn realm_authentication_executions_with_execution_id_get(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<AuthenticationExecutionRepresentation, KeycloakError> {
        let realm = p(realm);
        let execution_id = p(execution_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete execution
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/executions/{execution_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmauthenticationexecutionsexecutionid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/authentication/executions/{executionId}`
    pub async fn realm_authentication_executions_with_execution_id_delete(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let execution_id = p(execution_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Update execution with new configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions/{execution_id}/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidconfig>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/config`
    pub async fn realm_authentication_executions_with_execution_id_config_post(
        &self,
        realm: &str,
        execution_id: &str,
        body: AuthenticatorConfigRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let execution_id = p(execution_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get execution's configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    /// - `id`: Configuration id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/executions/{execution_id}/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationexecutionsexecutionidconfigid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/executions/{executionId}/config/{id}`
    #[deprecated]
    pub async fn realm_authentication_executions_with_execution_id_config_with_id_get(
        &self,
        realm: &str,
        execution_id: &str,
        id: &str,
    ) -> Result<AuthenticatorConfigRepresentation, KeycloakError> {
        let realm = p(realm);
        let execution_id = p(execution_id);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/config/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Lower execution's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions/{execution_id}/lower-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidlower_priority>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/lower-priority`
    pub async fn realm_authentication_executions_with_execution_id_lower_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let execution_id = p(execution_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/lower-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Raise execution's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions/{execution_id}/raise-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidraise_priority>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/raise-priority`
    pub async fn realm_authentication_executions_with_execution_id_raise_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let execution_id = p(execution_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/raise-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get authentication flows Returns a stream of authentication flows.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/flows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationflows>
    pub async fn realm_authentication_flows_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<AuthenticationFlowRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/flows",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new authentication flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationflows>
    pub async fn realm_authentication_flows_post(
        &self,
        realm: &str,
        body: AuthenticationFlowRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/flows",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Copy existing authentication flow under a new name The new name is given as 'newName' attribute of the passed JSON object
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: name of the existing authentication flow
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows/{flow_alias}/copy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliascopy>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/copy`
    pub async fn realm_authentication_flows_with_flow_alias_copy_post(
        &self,
        realm: &str,
        flow_alias: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let flow_alias = p(flow_alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/copy",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get authentication executions for a flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Flow alias
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/flows/{flow_alias}/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationflowsflowaliasexecutions>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    pub async fn realm_authentication_flows_with_flow_alias_executions_get(
        &self,
        realm: &str,
        flow_alias: &str,
    ) -> Result<TypeVec<AuthenticationExecutionInfoRepresentation>, KeycloakError> {
        let realm = p(realm);
        let flow_alias = p(flow_alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update authentication executions of a Flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Flow alias
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/flows/{flow_alias}/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmauthenticationflowsflowaliasexecutions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    pub async fn realm_authentication_flows_with_flow_alias_executions_put(
        &self,
        realm: &str,
        flow_alias: &str,
        body: AuthenticationExecutionInfoRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let flow_alias = p(flow_alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Add new authentication execution to a flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Alias of parent flow
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows/{flow_alias}/executions/execution`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliasexecutionsexecution>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/execution`
    pub async fn realm_authentication_flows_with_flow_alias_executions_execution_post(
        &self,
        realm: &str,
        flow_alias: &str,
        body: TypeMap<String, Value>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let flow_alias = p(flow_alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions/execution",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Add new flow with new execution to existing flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Alias of parent authentication flow
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows/{flow_alias}/executions/flow`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliasexecutionsflow>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/flow`
    pub async fn realm_authentication_flows_with_flow_alias_executions_flow_post(
        &self,
        realm: &str,
        flow_alias: &str,
        body: TypeMap<String, Value>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let flow_alias = p(flow_alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions/flow",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get authentication flow for id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Flow id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationflowsid>
    pub async fn realm_authentication_flows_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticationFlowRepresentation, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/flows/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update an authentication flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmauthenticationflowsid>
    pub async fn realm_authentication_flows_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: AuthenticationFlowRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/authentication/flows/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete an authentication flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Flow id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmauthenticationflowsid>
    pub async fn realm_authentication_flows_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/authentication/flows/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get form action providers Returns a stream of form action providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/form-action-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationform_action_providers>
    pub async fn realm_authentication_form_action_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/form-action-providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get form providers Returns a stream of form providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/form-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationform_providers>
    pub async fn realm_authentication_form_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/form-providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get configuration descriptions for all clients
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/per-client-config-description`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationper_client_config_description>
    pub async fn realm_authentication_per_client_config_description_get(
        &self,
        realm: &str,
    ) -> Result<TypeMap<String, TypeVec<ConfigPropertyRepresentation>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/per-client-config-description",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a new required actions
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/register-required-action`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationregister_required_action>
    pub async fn realm_authentication_register_required_action_post(
        &self,
        realm: &str,
        body: RequiredActionProviderRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/register-required-action",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get required actions Returns a stream of required actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/required-actions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actions>
    pub async fn realm_authentication_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/required-actions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get required action for alias
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsalias>
    pub async fn realm_authentication_required_actions_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<RequiredActionProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update required action
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmauthenticationrequired_actionsalias>
    pub async fn realm_authentication_required_actions_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        body: RequiredActionProviderRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete required action
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmauthenticationrequired_actionsalias>
    pub async fn realm_authentication_required_actions_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get RequiredAction configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/required-actions/{alias}/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsaliasconfig>
    pub async fn realm_authentication_required_actions_with_alias_config_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<RequiredActionConfigRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/config",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update RequiredAction configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/required-actions/{alias}/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmauthenticationrequired_actionsaliasconfig>
    pub async fn realm_authentication_required_actions_with_alias_config_put(
        &self,
        realm: &str,
        alias: &str,
        body: RequiredActionConfigRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete RequiredAction configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/required-actions/{alias}/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmauthenticationrequired_actionsaliasconfig>
    pub async fn realm_authentication_required_actions_with_alias_config_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/config",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get RequiredAction provider configuration description
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/required-actions/{alias}/config-description`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsaliasconfig_description>
    pub async fn realm_authentication_required_actions_with_alias_config_description_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<RequiredActionConfigInfoRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/config-description",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Lower required action's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/required-actions/{alias}/lower-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationrequired_actionsaliaslower_priority>
    pub async fn realm_authentication_required_actions_with_alias_lower_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/lower-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Raise required action's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/required-actions/{alias}/raise-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmauthenticationrequired_actionsaliasraise_priority>
    pub async fn realm_authentication_required_actions_with_alias_raise_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/raise-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get unregistered required actions Returns a stream of unregistered required actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /admin/realms/{realm}/authentication/unregistered-required-actions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmauthenticationunregistered_required_actions>
    pub async fn realm_authentication_unregistered_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/authentication/unregistered-required-actions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 219
