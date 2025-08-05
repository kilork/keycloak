use reqwest::header::CONTENT_LENGTH;
use serde_json::Value;

use super::{url_enc::encode_url_param as p, *};

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Attack Detection</h4>

    /// Clear any user login failures for all users This can release temporary disabled users
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /admin/realms/{realm}/attack-detection/brute-force/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmattack_detectionbrute_forceusers>
    #[cfg(feature = "tag-attack-detection")]
    pub async fn realm_attack_detection_brute_force_users_delete(
        &self,
        realm: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get status of a username in brute force detection
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Attack Detection`
    ///
    /// `GET /admin/realms/{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmattack_detectionbrute_forceusersuserid>
    ///
    /// REST method: `GET /admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    #[cfg(feature = "tag-attack-detection")]
    pub async fn realm_attack_detection_brute_force_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users/{user_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Clear any user login failures for the user This can release temporary disabled user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /admin/realms/{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmattack_detectionbrute_forceusersuserid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    #[cfg(feature = "tag-attack-detection")]
    pub async fn realm_attack_detection_brute_force_users_with_user_id_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users/{user_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationauthenticator_providers>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationclient_authenticator_providers>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationconfig>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationconfig_descriptionproviderid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/config-description/{providerId}`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationconfigid>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmauthenticationconfigid>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmauthenticationconfigid>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutions>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationexecutionsexecutionid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/executions/{executionId}`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmauthenticationexecutionsexecutionid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/authentication/executions/{executionId}`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidconfig>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/config`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationexecutionsexecutionidconfigid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/executions/{executionId}/config/{id}`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidlower_priority>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/lower-priority`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidraise_priority>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/raise-priority`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationflows>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationflows>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliascopy>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/copy`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationflowsflowaliasexecutions>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmauthenticationflowsflowaliasexecutions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliasexecutionsexecution>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/execution`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliasexecutionsflow>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/flow`
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationflowsid>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmauthenticationflowsid>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmauthenticationflowsid>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationform_action_providers>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationform_providers>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationper_client_config_description>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationregister_required_action>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actions>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsalias>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmauthenticationrequired_actionsalias>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmauthenticationrequired_actionsalias>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsaliasconfig>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmauthenticationrequired_actionsaliasconfig>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmauthenticationrequired_actionsaliasconfig>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsaliasconfig_description>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationrequired_actionsaliaslower_priority>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmauthenticationrequired_actionsaliasraise_priority>
    #[cfg(feature = "tag-authentication-management")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmauthenticationunregistered_required_actions>
    #[cfg(feature = "tag-authentication-management")]
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

    // <h4>Client Attribute Certificate</h4>

    /// Get key info
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidcertificatesattr>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}`
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_get(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get a keystore file for the client, containing private key and public certificate
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    /// - `body`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrdownload>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/download`
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_download_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
        body: KeyStoreConfig,
    ) -> Result<TypeString, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/download",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.text().await.map(From::from)?)
    }

    /// Generate a new certificate with new key pair
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrgenerate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/generate`
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_generate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new keypair and certificate, and get the private key file
    ///
    /// Generates a keypair and certificate and serves the private key in a specified keystore format.
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    /// - `body`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate-and-download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrgenerate_and_download>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/generate-and-download`
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_generate_and_download_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
        body: KeyStoreConfig,
    ) -> Result<TypeString, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate-and-download",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.text().await.map(From::from)?)
    }

    /// Upload certificate and eventually private key
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrupload>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/upload`
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_upload_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Upload only certificate, not private key
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload-certificate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrupload_certificate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/upload-certificate`
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_upload_certificate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload-certificate",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

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
    #[cfg(feature = "tag-client-initial-access")]
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
    #[cfg(feature = "tag-client-initial-access")]
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
    #[cfg(feature = "tag-client-initial-access")]
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

    // <h4>Client Registration Policy</h4>

    /// Base path for retrieve providers with the configProperties properly filled
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Registration Policy`
    ///
    /// `GET /admin/realms/{realm}/client-registration-policy/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_registration_policyproviders>
    #[cfg(feature = "tag-client-registration-policy")]
    pub async fn realm_client_registration_policy_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ComponentTypeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-registration-policy/providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Client Role Mappings</h4>

    /// Get client-level role mappings for the user or group, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_get(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_post(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete client-level roles from user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_delete(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get available client-level roles that can be mapped to the user or group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_idavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}/available`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_available_get(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings This recurses any composite roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_idcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}/composite`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_composite_get(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client-level role mappings for the user or group, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_get(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_post(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete client-level roles from user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_delete(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get available client-level roles that can be mapped to the user or group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_idavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}/available`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_available_get(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings This recurses any composite roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_idcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}/composite`
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_composite_get(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopes>
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_scopes>
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}`
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_scopesclient_scope_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-scopes/{client-scope-id}`
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}`
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templates>
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_templates>
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}`
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_templatesclient_scope_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-templates/{client-scope-id}`
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}`
    #[cfg(feature = "tag-client-scopes")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclients>
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclients>
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidclient_secret>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/client-secret`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidclient_secret>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/client-secret`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidclient_secretrotated>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/client-secret/rotated`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidclient_secretrotated>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/client-secret/rotated`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuiddefault_client_scopes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/default-client-scopes`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuiddefault_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuiddefault_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesgenerate_example_access_token>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/generate-example-access-token`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesgenerate_example_id_token>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/generate-example-id-token`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesgenerate_example_userinfo>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/generate-example-userinfo`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesprotocol_mappers>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/protocol-mappers`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesscope_mappingsrolecontaineridgranted>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/scope-mappings/{roleContainerId}/granted`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidevaluate_scopesscope_mappingsrolecontaineridnot_granted>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/scope-mappings/{roleContainerId}/not-granted`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidinstallationprovidersproviderid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/installation/providers/{providerId}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/management/permissions`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/management/permissions`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidnodes>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/nodes`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidnodesnode>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/nodes/{node}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidoffline_session_count>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/offline-session-count`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidoffline_sessions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/offline-sessions`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidoptional_client_scopes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidoptional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidoptional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidpush_revocation>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/push-revocation`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidregistration_access_token>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/registration-access-token`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidservice_account_user>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/service-account-user`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidsession_count>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/session-count`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidtest_nodes_available>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/test-nodes-available`
    #[cfg(feature = "tag-clients")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuiduser_sessions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/user-sessions`
    #[cfg(feature = "tag-clients")]
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

    // <h4>Component</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `name`
    /// - `parent`
    /// - `type_`
    ///
    /// Resource: `Component`
    ///
    /// `GET /admin/realms/{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmcomponents>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_get(
        &self,
        realm: &str,
        name: Option<String>,
        parent: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<ComponentRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/components", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = parent {
            builder = builder.query(&[("parent", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Component`
    ///
    /// `POST /admin/realms/{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmcomponents>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_post(
        &self,
        realm: &str,
        body: ComponentRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/components", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Component`
    ///
    /// `GET /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmcomponentsid>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ComponentRepresentation, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/components/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Component`
    ///
    /// `PUT /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmcomponentsid>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ComponentRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/components/{id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Component`
    ///
    /// `DELETE /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmcomponentsid>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/components/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `type_`
    ///
    /// Resource: `Component`
    ///
    /// `GET /admin/realms/{realm}/components/{id}/sub-component-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmcomponentsidsub_component_types>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_sub_component_types_get(
        &self,
        realm: &str,
        id: &str,
        type_: Option<String>,
    ) -> Result<TypeVec<ComponentTypeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/components/{id}/sub-component-types",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Groups</h4>

    /// Get group hierarchy.  Only `name` and `id` are returned.  `subGroups` are only returned when using the `search` or `q` parameter. If none of these parameters is provided, the top-level groups are returned without `subGroups` being filled.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `exact`
    /// - `first`
    /// - `max`
    /// - `populate_hierarchy`
    /// - `q`
    /// - `search`
    /// - `sub_groups_count`: Boolean which defines whether to return the count of subgroups for each group (default: true
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroups>
    #[cfg(feature = "tag-groups")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_groups_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        populate_hierarchy: Option<bool>,
        q: Option<String>,
        search: Option<String>,
        sub_groups_count: Option<bool>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/groups", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = populate_hierarchy {
            builder = builder.query(&[("populateHierarchy", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = sub_groups_count {
            builder = builder.query(&[("subGroupsCount", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// create or add a top level realm groupSet or create child.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `POST /admin/realms/{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroups>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_post(
        &self,
        realm: &str,
        body: GroupRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/groups", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the groups counts.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `search`
    /// - `top`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupscount>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_count_get(
        &self,
        realm: &str,
        search: Option<String>,
        top: Option<bool>,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/groups/count", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = top {
            builder = builder.query(&[("top", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update group, ignores subgroups.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /admin/realms/{realm}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmgroupsgroup_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/groups/{group-id}`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_put(
        &self,
        realm: &str,
        group_id: &str,
        body: GroupRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/groups/{group_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return a paginated list of subgroups that have a parent group corresponding to the group on the URL
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: Boolean which defines whether brief groups representations are returned or not (default: false)
    /// - `exact`: Boolean which defines whether the params "search" must match exactly or not
    /// - `first`: The position of the first result to be returned (pagination offset).
    /// - `max`: The maximum number of results that are to be returned. Defaults to 10
    /// - `search`: A String representing either an exact group name or a partial name
    /// - `sub_groups_count`: Boolean which defines whether to return the count of subgroups for each subgroup of this group (default: true
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idchildren>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/children`
    #[cfg(feature = "tag-groups")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_groups_with_group_id_children_get(
        &self,
        realm: &str,
        group_id: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
        sub_groups_count: Option<bool>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/children",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = sub_groups_count {
            builder = builder.query(&[("subGroupsCount", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Set or create child.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idchildren>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/children`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_children_post(
        &self,
        realm: &str,
        group_id: &str,
        body: GroupRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/children",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/management/permissions`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_management_permissions_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/management/permissions",
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
    /// - `group_id`
    /// - `body`
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /admin/realms/{realm}/groups/{group_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmgroupsgroup_idmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/groups/{group-id}/management/permissions`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_management_permissions_put(
        &self,
        realm: &str,
        group_id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get users Returns a stream of users, filtered according to query parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/members`
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_group_id_members_get(
        &self,
        realm: &str,
        group_id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/members",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Identity Providers</h4>

    /// Import identity provider from JSON body
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/import-config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmidentity_providerimport_config>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_import_config_post(
        &self,
        realm: &str,
        body: TypeMap<String, Value>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/import-config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// List identity providers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    /// - `realm_only`: Boolean which defines if only realm-level IDPs (not associated with orgs) should be returned (default: false)
    /// - `search`: Filter specific providers by name. Search can be prefix (name*), contains (*name*) or exact ("name"). Default prefixed.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstances>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        realm_only: Option<bool>,
        search: Option<String>,
    ) -> Result<TypeVec<IdentityProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = realm_only {
            builder = builder.query(&[("realmOnly", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstances>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_post(
        &self,
        realm: &str,
        body: IdentityProviderRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesalias>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesalias>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        body: IdentityProviderRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesalias>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Export public broker configuration for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `format`: Format to use
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasexport>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_export_get(
        &self,
        realm: &str,
        alias: &str,
        format: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/export",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = format {
            builder = builder.query(&[("format", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions",
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
    /// - `alias`
    /// - `body`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_put(
        &self,
        realm: &str,
        alias: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper types for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmapper_types>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mapper_types_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<TypeMap<String, IdentityProviderMapperTypeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mappers for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappers>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a mapper to identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstancesaliasmappers>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_post(
        &self,
        realm: &str,
        alias: &str,
        body: IdentityProviderMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mapper by id for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_get(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<IdentityProviderMapperRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a mapper for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_put(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
        body: IdentityProviderMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete a mapper for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_delete(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Reaload keys for the identity provider if the provider supports it, "true" is returned if reload was performed, "false" if not.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/reload-keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasreload_keys>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_reload_keys_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<bool, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/reload-keys",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the identity provider factory for that provider id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `provider_id`: The provider id to get the factory
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmidentity_providerprovidersprovider_id>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_providers_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let provider_id = p(provider_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/providers/{provider_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Key</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Key`
    ///
    /// `GET /admin/realms/{realm}/keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmkeys>
    #[cfg(feature = "tag-key")]
    pub async fn realm_keys_get(
        &self,
        realm: &str,
    ) -> Result<KeysMetadataRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/keys", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Organizations</h4>

    /// Returns a paginated list of organizations filtered according to the specified parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned - defaults to 10
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String representing either an organization name or domain
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizations>
    #[cfg(feature = "tag-organizations")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_organizations_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<String>,
    ) -> Result<TypeVec<OrganizationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/organizations", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
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
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Creates a new organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizations>
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_post(
        &self,
        realm: &str,
        body: OrganizationRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/organizations", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the organizations counts.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String representing either an organization name or domain
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationscount>
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_count_get(
        &self,
        realm: &str,
        exact: Option<bool>,
        q: Option<String>,
        search: Option<String>,
    ) -> Result<i64, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns the organizations associated with the user that has the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `member_id`
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/members/{member_id}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/members/{member-id}/organizations`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_members_with_member_id_organizations_get(
        &self,
        realm: &str,
        member_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<OrganizationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let member_id = p(member_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/members/{member_id}/organizations",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns the organization representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_get(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<OrganizationRepresentation, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Updates the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `PUT /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/organizations/{org-id}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_put(
        &self,
        realm: &str,
        org_id: &str,
        body: OrganizationRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Deletes the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_delete(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns all identity providers associated with the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_identity_providers_get(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<TypeVec<IdentityProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Adds the identity provider with the specified id to the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/identity-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_identity_providers_post(
        &self,
        realm: &str,
        org_id: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the identity provider associated with the organization that has the specified alias
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_identity_providers_with_alias_get(
        &self,
        realm: &str,
        org_id: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Removes the identity provider with the specified alias from the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_identity_providers_with_alias_delete(
        &self,
        realm: &str,
        org_id: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let alias = p(alias);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns a paginated list of organization members filtered according to the specified parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned. Defaults to 10
    /// - `membership_type`: The membership type
    /// - `search`: A String representing either a member's username, e-mail, first name, or last name.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members`
    #[cfg(feature = "tag-organizations")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_organizations_with_org_id_members_get(
        &self,
        realm: &str,
        org_id: &str,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        membership_type: Option<String>,
        search: Option<String>,
    ) -> Result<TypeVec<MemberRepresentation>, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = membership_type {
            builder = builder.query(&[("membershipType", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Adds the user with the specified id as a member of the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_post(
        &self,
        realm: &str,
        org_id: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns number of members in the organization.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmemberscount>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/count`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_count_get(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<i64, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Invites an existing user to the organization, using the specified user id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members/invite-existing-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_existing_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-existing-user`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_invite_existing_user_post(
        &self,
        realm: &str,
        org_id: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/invite-existing-user",
                self.url
            ))
            .form(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Invites an existing user or sends a registration link to a new user, based on the provided e-mail address.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members/invite-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-user`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_invite_user_post(
        &self,
        realm: &str,
        org_id: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/invite-user",
                self.url
            ))
            .form(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the member of the organization with the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_with_member_id_get(
        &self,
        realm: &str,
        org_id: &str,
        member_id: &str,
    ) -> Result<MemberRepresentation, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let member_id = p(member_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/{member_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Removes the user with the specified id from the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/members/{member_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_with_member_id_delete(
        &self,
        realm: &str,
        org_id: &str,
        member_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let member_id = p(member_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/{member_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the organizations associated with the user that has the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}/organizations`
    #[cfg(feature = "tag-organizations")]
    pub async fn realm_organizations_with_org_id_members_with_member_id_organizations_get(
        &self,
        realm: &str,
        org_id: &str,
        member_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<OrganizationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let member_id = p(member_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/{member_id}/organizations",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/add-models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/protocol/{protocol}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/add-models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/protocol/{protocol}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/add-models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    #[cfg(feature = "tag-protocol-mappers")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/protocol/{protocol}`
    #[cfg(feature = "tag-protocol-mappers")]
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

    // <h4>Realms Admin</h4>

    /// Get accessible realms Returns a list of accessible realms. The list is filtered based on what realms the caller is allowed to view.
    ///
    /// Parameters:
    ///
    /// - `brief_representation`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealms>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn get(
        &self,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RealmRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(format!("{}/admin/realms", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Import a realm. Imports a realm from a full representation of that realm.
    ///
    /// Parameters:
    ///
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealms>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn post(&self, body: RealmRepresentation) -> Result<DefaultResponse, KeycloakError> {
        let builder = self
            .client
            .post(format!("{}/admin/realms", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the top-level representation of the realm It will not include nested information like User and Client representations.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealm>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_get(&self, realm: &str) -> Result<RealmRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the top-level information of the realm Any user, roles or client information in the representation will be ignored.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealm>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_put(
        &self,
        realm: &str,
        body: RealmRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealm>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_delete(&self, realm: &str) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get admin events Returns all admin events, or filters events based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `auth_client`
    /// - `auth_ip_address`
    /// - `auth_realm`
    /// - `auth_user`: user id
    /// - `date_from`: From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `date_to`: To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `direction`: The direction to sort events by (asc or desc)
    /// - `first`
    /// - `max`: Maximum results size (defaults to 100)
    /// - `operation_types`
    /// - `resource_path`
    /// - `resource_types`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmadmin_events>
    #[cfg(feature = "tag-realms-admin")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_admin_events_get(
        &self,
        realm: &str,
        auth_client: Option<String>,
        auth_ip_address: Option<String>,
        auth_realm: Option<String>,
        auth_user: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        direction: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        operation_types: Option<Vec<String>>,
        resource_path: Option<String>,
        resource_types: Option<Vec<String>>,
    ) -> Result<TypeVec<AdminEventRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/admin-events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = auth_client {
            builder = builder.query(&[("authClient", v)]);
        }
        if let Some(v) = auth_ip_address {
            builder = builder.query(&[("authIpAddress", v)]);
        }
        if let Some(v) = auth_realm {
            builder = builder.query(&[("authRealm", v)]);
        }
        if let Some(v) = auth_user {
            builder = builder.query(&[("authUser", v)]);
        }
        if let Some(v) = date_from {
            builder = builder.query(&[("dateFrom", v)]);
        }
        if let Some(v) = date_to {
            builder = builder.query(&[("dateTo", v)]);
        }
        if let Some(v) = direction {
            builder = builder.query(&[("direction", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = operation_types {
            builder = builder.query(
                &v.into_iter()
                    .map(|e| ("operationTypes", e))
                    .collect::<Vec<_>>(),
            );
        }
        if let Some(v) = resource_path {
            builder = builder.query(&[("resourcePath", v)]);
        }
        if let Some(v) = resource_types {
            builder = builder.query(
                &v.into_iter()
                    .map(|e| ("resourceTypes", e))
                    .collect::<Vec<_>>(),
            );
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete all admin events
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmadmin_events>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_admin_events_delete(
        &self,
        realm: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/admin-events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Base path for importing clients under this realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/client-description-converter`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_description_converter>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_description_converter_post(
        &self,
        realm: &str,
        body: String,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-description-converter",
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
    /// - `include_global_policies`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_policiespolicies>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_policies_get(
        &self,
        realm: &str,
        include_global_policies: Option<bool>,
    ) -> Result<ClientPoliciesRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-policies/policies",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = include_global_policies {
            builder = builder.query(&[("include-global-policies", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_policiespolicies>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_policies_put(
        &self,
        realm: &str,
        body: ClientPoliciesRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-policies/policies",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `include_global_profiles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_policiesprofiles>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_profiles_get(
        &self,
        realm: &str,
        include_global_profiles: Option<bool>,
    ) -> Result<ClientProfilesRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-policies/profiles",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = include_global_profiles {
            builder = builder.query(&[("include-global-profiles", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_policiesprofiles>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_profiles_put(
        &self,
        realm: &str,
        body: ClientProfilesRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-policies/profiles",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client session stats Returns a JSON map.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-session-stats`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_session_stats>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_session_stats_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, String>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-session-stats",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// List all client types available in the current realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_types>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_types_get(
        &self,
        realm: &str,
    ) -> Result<ClientTypesRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/client-types", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a client type
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_types>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_types_put(
        &self,
        realm: &str,
        body: ClientTypesRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/client-types", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/credential-registrators`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmcredential_registrators>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_credential_registrators_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<String>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/credential-registrators",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm default client scopes. Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_default_client_scopes>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_default_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/default-default-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/default-default-client-scopes/{client_scope_id}",
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
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/default-default-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get group hierarchy.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_groups>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_groups_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/default-groups", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-groups/{groupId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_groups_with_group_id_put(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/default-groups/{group_id}",
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
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-groups/{groupId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_groups_with_group_id_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/default-groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm optional client scopes. Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_optional_client_scopes>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_optional_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}",
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
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get events Returns all events, or filters them based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client`: App or oauth client name
    /// - `date_from`: From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `date_to`: To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `direction`: The direction to sort events by (asc or desc)
    /// - `first`: Paging offset
    /// - `ip_address`: IP Address
    /// - `max`: Maximum results size (defaults to 100)
    /// - `type_`: The types of events to return
    /// - `user`: User id
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmevents>
    #[cfg(feature = "tag-realms-admin")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_events_get(
        &self,
        realm: &str,
        client: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        direction: Option<String>,
        first: Option<i32>,
        ip_address: Option<String>,
        max: Option<i32>,
        type_: Option<Vec<String>>,
        user: Option<String>,
    ) -> Result<TypeVec<EventRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client {
            builder = builder.query(&[("client", v)]);
        }
        if let Some(v) = date_from {
            builder = builder.query(&[("dateFrom", v)]);
        }
        if let Some(v) = date_to {
            builder = builder.query(&[("dateTo", v)]);
        }
        if let Some(v) = direction {
            builder = builder.query(&[("direction", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = ip_address {
            builder = builder.query(&[("ipAddress", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&v.into_iter().map(|e| ("type", e)).collect::<Vec<_>>());
        }
        if let Some(v) = user {
            builder = builder.query(&[("user", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete all events
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmevents>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_events_delete(&self, realm: &str) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the events provider configuration Returns JSON object with events provider configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmeventsconfig>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_events_config_get(
        &self,
        realm: &str,
    ) -> Result<RealmEventsConfigRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/events/config", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmeventsconfig>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_events_config_put(
        &self,
        realm: &str,
        body: RealmEventsConfigRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/events/config", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `path`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/group-by-path/{path}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroup_by_pathpath>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_group_by_path_with_path_get(
        &self,
        realm: &str,
        path: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let realm = p(realm);
        let path = p(path);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/group-by-path/{path}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalization>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<String>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/localization", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `use_realm_default_locale_fallback`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalizationlocale>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_get(
        &self,
        realm: &str,
        locale: &str,
        use_realm_default_locale_fallback: Option<bool>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        let realm = p(realm);
        let locale = p(locale);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = use_realm_default_locale_fallback {
            builder = builder.query(&[("useRealmDefaultLocaleFallback", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Import localization from uploaded JSON file
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmlocalizationlocale>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_post(
        &self,
        realm: &str,
        locale: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let locale = p(locale);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocale>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_delete(
        &self,
        realm: &str,
        locale: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let locale = p(locale);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalizationlocalekey>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_with_key_get(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
    ) -> Result<TypeString, KeycloakError> {
        let realm = p(realm);
        let key = p(key);
        let locale = p(locale);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.text().await.map(From::from)?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmlocalizationlocalekey>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_with_key_put(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let key = p(key);
        let locale = p(locale);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .body(body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocalekey>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_with_key_delete(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let key = p(key);
        let locale = p(locale);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Removes all user sessions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/logout-all`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmlogout_all>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_logout_all_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/logout-all", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Partial export of existing realm into a JSON file.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `export_clients`
    /// - `export_groups_and_roles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/partial-export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpartial_export>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_partial_export_post(
        &self,
        realm: &str,
        export_clients: Option<bool>,
        export_groups_and_roles: Option<bool>,
    ) -> Result<RealmRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/partial-export", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = export_clients {
            builder = builder.query(&[("exportClients", v)]);
        }
        if let Some(v) = export_groups_and_roles {
            builder = builder.query(&[("exportGroupsAndRoles", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Partial import from a JSON file to an existing realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/partialImport`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpartialimport>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_partial_import_post(
        &self,
        realm: &str,
        body: RealmRepresentation,
    ) -> Result<Value, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/partialImport", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Push the realm's revocation policy to any client that has an admin url associated with it.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpush_revocation>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_push_revocation_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/push-revocation", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a specific user session.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `session`
    /// - `is_offline`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/sessions/{session}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmsessionssession>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_sessions_with_session_delete(
        &self,
        realm: &str,
        session: &str,
        is_offline: Option<bool>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let session = p(session);
        let mut builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/sessions/{session}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = is_offline {
            builder = builder.query(&[("isOffline", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Test SMTP connection with current logged in user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/testSMTPConnection`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmtestsmtpconnection>
    #[cfg(feature = "tag-realms-admin")]
    #[deprecated]
    pub async fn realm_test_smtp_connection_post(
        &self,
        realm: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/testSMTPConnection",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusers_management_permissions>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_users_management_permissions_get(
        &self,
        realm: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users-management-permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusers_management_permissions>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_users_management_permissions_put(
        &self,
        realm: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users-management-permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Role Mapper</h4>

    /// Get role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_group_id_role_mappings_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_group_id_role_mappings_realm_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_group_id_role_mappings_realm_post(
        &self,
        realm: &str,
        group_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_group_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        group_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm/available`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_group_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm/composite`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_group_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        group_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_user_id_role_mappings_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_user_id_role_mappings_realm_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_user_id_role_mappings_realm_post(
        &self,
        realm: &str,
        user_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_user_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        user_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm/available`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_user_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm/composite`
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_user_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        user_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Roles</h4>

    /// Get all roles for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidroles>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_get(
        &self,
        realm: &str,
        client_uuid: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidroles>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/roles`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_put(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get composites of the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client-level roles for the client that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecompositesclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites/clients/{client-uuid}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites/realm`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of groups that have the specified role name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/groups`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/groups",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/users`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_users_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/users",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get all roles for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles>
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/roles", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmroles>
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_post(
        &self,
        realm: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/roles", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_put(
        &self,
        realm: &str,
        role_name: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_delete(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get composites of the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client-level roles for the client that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/clients/{client-uuid}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/realm`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of groups that have the specified role name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/groups`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/groups",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        role_name: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/users`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_users_get(
        &self,
        realm: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/users",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Roles (by ID)</h4>

    /// Get a specific role's representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles-by-id/{role-id}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_put(
        &self,
        realm: &str,
        role_id: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles-by-id/{role-id}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_delete(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get role's children Returns a set of role's children provided the role is a composite.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_get(
        &self,
        realm: &str,
        role_id: &str,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Make the role a composite role by associating some child roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `POST /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_post(
        &self,
        realm: &str,
        role_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a set of roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: Role id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_delete(
        &self,
        realm: &str,
        role_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client-level roles for the client that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcompositesclientsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites/clients/{clientUuid}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites/realm`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_realm_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_management_permissions_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `body`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /admin/realms/{realm}/roles-by-id/{role_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmroles_by_idrole_idmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_management_permissions_put(
        &self,
        realm: &str,
        role_id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclientavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}/available`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsclientsclientcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/clients/{client}/composite`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm/available`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idscope_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/realm/composite`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclientavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}/available`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsclientsclientcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/clients/{client}/composite`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm/available`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idscope_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/scope-mappings/realm/composite`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclient>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclient>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclientavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}/available`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsclientsclientcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/{client}/composite`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidscope_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidscope_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm/available`
    #[cfg(feature = "tag-scope-mappings")]
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidscope_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm/composite`
    #[cfg(feature = "tag-scope-mappings")]
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

    // <h4>Users</h4>

    /// Get users Returns a stream of users, filtered according to query parameters.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `email`: A String contained in email, or the complete email, if param "exact" is true
    /// - `email_verified`: whether the email has been verified
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `exact`: Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
    /// - `first`: Pagination offset
    /// - `first_name`: A String contained in firstName, or the complete firstName, if param "exact" is true
    /// - `idp_alias`: The alias of an Identity Provider linked to the user
    /// - `idp_user_id`: The userId at an Identity Provider linked to the user
    /// - `last_name`: A String contained in lastName, or the complete lastName, if param "exact" is true
    /// - `max`: Maximum results size (defaults to 100)
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    /// - `username`: A String contained in username, or the complete username, if param "exact" is true
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusers>
    #[cfg(feature = "tag-users")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_users_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        email: Option<String>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        first_name: Option<String>,
        idp_alias: Option<String>,
        idp_user_id: Option<String>,
        last_name: Option<String>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<String>,
        username: Option<String>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
        }
        if let Some(v) = email_verified {
            builder = builder.query(&[("emailVerified", v)]);
        }
        if let Some(v) = enabled {
            builder = builder.query(&[("enabled", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = first_name {
            builder = builder.query(&[("firstName", v)]);
        }
        if let Some(v) = idp_alias {
            builder = builder.query(&[("idpAlias", v)]);
        }
        if let Some(v) = idp_user_id {
            builder = builder.query(&[("idpUserId", v)]);
        }
        if let Some(v) = last_name {
            builder = builder.query(&[("lastName", v)]);
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
        if let Some(v) = username {
            builder = builder.query(&[("username", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new user Username must be unique.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusers>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_post(
        &self,
        realm: &str,
        body: UserRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/users", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the number of users that match the given criteria.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `email`: email filter
    /// - `email_verified`
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `first_name`: first name filter
    /// - `last_name`: last name filter
    /// - `q`
    /// - `search`: arbitrary search string for all the fields below. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    /// - `username`: username filter
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmuserscount>
    #[cfg(feature = "tag-users")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_users_count_get(
        &self,
        realm: &str,
        email: Option<String>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        first_name: Option<String>,
        last_name: Option<String>,
        q: Option<String>,
        search: Option<String>,
        username: Option<String>,
    ) -> Result<i32, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users/count", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
        }
        if let Some(v) = email_verified {
            builder = builder.query(&[("emailVerified", v)]);
        }
        if let Some(v) = enabled {
            builder = builder.query(&[("enabled", v)]);
        }
        if let Some(v) = first_name {
            builder = builder.query(&[("firstName", v)]);
        }
        if let Some(v) = last_name {
            builder = builder.query(&[("lastName", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = username {
            builder = builder.query(&[("username", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/profile`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersprofile>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_profile_get(&self, realm: &str) -> Result<UPConfig, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users/profile", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/profile`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersprofile>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_profile_put(
        &self,
        realm: &str,
        body: UPConfig,
    ) -> Result<UPConfig, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/users/profile", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/profile/metadata`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersprofilemetadata>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_profile_metadata_get(
        &self,
        realm: &str,
    ) -> Result<UserProfileMetadata, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/profile/metadata",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get representation of the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `user_profile_metadata`: Indicates if the user profile metadata should be added to the response
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
        user_profile_metadata: Option<bool>,
    ) -> Result<UserRepresentation, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users/{user_id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = user_profile_metadata {
            builder = builder.query(&[("userProfileMetadata", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_put(
        &self,
        realm: &str,
        user_id: &str,
        body: UserRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/users/{user_id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/users/{user_id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/configured-user-storage-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idconfigured_user_storage_credential_types>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/configured-user-storage-credential-types`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_configured_user_storage_credential_types_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<String>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/configured-user-storage-credential-types",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get consents granted by the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/consents`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idconsents>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/consents`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_consents_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/consents",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Revoke consent and offline tokens for particular client from user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`: Client id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/consents/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idconsentsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/consents/{client}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_consents_with_client_delete(
        &self,
        realm: &str,
        user_id: &str,
        client: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client = p(client);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/consents/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/credentials`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idcredentials>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/credentials`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_credentials_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<CredentialRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a credential for a user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idcredentialscredentialid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_credentials_with_credential_id_delete(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Move a credential to a position behind another credential
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`: The credential to move
    /// - `new_previous_credential_id`: The credential that will be the previous element in the list. If set to null, the moved credential will be the first element in the list.
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idcredentialscredentialidmoveafternewpreviouscredentialid>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
        new_previous_credential_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let new_previous_credential_id = p(new_previous_credential_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Move a credential to a first position in the credentials list of the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`: The credential to move
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveToFirst`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idcredentialscredentialidmovetofirst>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/moveToFirst`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_credentials_with_credential_id_move_to_first_post(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveToFirst",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Update a credential label for a user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/userLabel`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_idcredentialscredentialiduserlabel>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/userLabel`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_credentials_with_credential_id_user_label_put(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/userLabel",
                self.url
            ))
            .body(body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Disable all credentials for a user of a specific type
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/disable-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_iddisable_credential_types>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/disable-credential-types`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_disable_credential_types_put(
        &self,
        realm: &str,
        user_id: &str,
        body: Vec<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/disable-credential-types",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Send an email to the user with a link they can click to execute particular actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/execute-actions-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_idexecute_actions_email>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/execute-actions-email`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_execute_actions_email_put(
        &self,
        realm: &str,
        user_id: &str,
        client_id: Option<String>,
        lifespan: Option<i32>,
        redirect_uri: Option<String>,
        body: Vec<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/execute-actions-email",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("client_id", v)]);
        }
        if let Some(v) = lifespan {
            builder = builder.query(&[("lifespan", v)]);
        }
        if let Some(v) = redirect_uri {
            builder = builder.query(&[("redirect_uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get social logins associated with the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/federated-identity`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idfederated_identity>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/federated-identity`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_federated_identity_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<FederatedIdentityRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/federated-identity",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a social login provider to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `provider`: Social login provider id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idfederated_identityprovider>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/federated-identity/{provider}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_federated_identity_with_provider_post(
        &self,
        realm: &str,
        user_id: &str,
        provider: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let provider = p(provider);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/federated-identity/{provider}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a social login provider from user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `provider`: Social login provider id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idfederated_identityprovider>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/federated-identity/{provider}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_federated_identity_with_provider_delete(
        &self,
        realm: &str,
        user_id: &str,
        provider: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let provider = p(provider);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/federated-identity/{provider}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idgroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/groups`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_groups_get(
        &self,
        realm: &str,
        user_id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idgroupscount>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/groups/count`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_groups_count_get(
        &self,
        realm: &str,
        user_id: &str,
        search: Option<String>,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups/count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_idgroupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/groups/{groupId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_groups_with_group_id_put(
        &self,
        realm: &str,
        user_id: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups/{group_id}",
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
    /// - `user_id`
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idgroupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/groups/{groupId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_groups_with_group_id_delete(
        &self,
        realm: &str,
        user_id: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Impersonate the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/impersonation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idimpersonation>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/impersonation`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_impersonation_post(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/impersonation",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/logout`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idlogout>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/logout`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_logout_post(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/logout",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get offline sessions associated with the user and client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_uuid`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/offline-sessions/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idoffline_sessionsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/offline-sessions/{clientUuid}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_offline_sessions_with_client_uuid_get(
        &self,
        realm: &str,
        user_id: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<UserSessionRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/offline-sessions/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Set up a new password for the user.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/reset-password`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_idreset_password>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/reset-password`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_reset_password_put(
        &self,
        realm: &str,
        user_id: &str,
        body: CredentialRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/reset-password",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Send an email to the user with a link they can click to reset their password.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id
    /// - `redirect_uri`: redirect uri
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/reset-password-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_idreset_password_email>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/reset-password-email`
    #[cfg(feature = "tag-users")]
    #[deprecated]
    pub async fn realm_users_with_user_id_reset_password_email_put(
        &self,
        realm: &str,
        user_id: &str,
        client_id: Option<String>,
        redirect_uri: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/reset-password-email",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("client_id", v)]);
        }
        if let Some(v) = redirect_uri {
            builder = builder.query(&[("redirect_uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Send an email-verification email to the user An email contains a link the user can click to verify their email address.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/send-verify-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusersuser_idsend_verify_email>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/send-verify-email`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_send_verify_email_put(
        &self,
        realm: &str,
        user_id: &str,
        client_id: Option<String>,
        lifespan: Option<i32>,
        redirect_uri: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/send-verify-email",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("client_id", v)]);
        }
        if let Some(v) = lifespan {
            builder = builder.query(&[("lifespan", v)]);
        }
        if let Some(v) = redirect_uri {
            builder = builder.query(&[("redirect_uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get sessions associated with the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idsessions>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/sessions`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_sessions_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<UserSessionRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/sessions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/unmanagedAttributes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idunmanagedattributes>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/unmanagedAttributes`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_user_id_unmanaged_attributes_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, TypeVec<String>>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/unmanagedAttributes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>default</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_server>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ResourceServerRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server",
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
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidauthzresource_server>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_put(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ResourceServerRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/import`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverimport>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/import`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_import_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ResourceServerRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/import",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `permission`
    /// - `policy_id`
    /// - `resource`
    /// - `resource_type`
    /// - `scope`
    /// - `type_`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpermission>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        permission: Option<bool>,
        policy_id: Option<String>,
        resource: Option<String>,
        resource_type: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<AbstractPolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = permission {
            builder = builder.query(&[("permission", v)]);
        }
        if let Some(v) = policy_id {
            builder = builder.query(&[("policyId", v)]);
        }
        if let Some(v) = resource {
            builder = builder.query(&[("resource", v)]);
        }
        if let Some(v) = resource_type {
            builder = builder.query(&[("resourceType", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpermission>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/evaluate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpermissionevaluate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission/evaluate`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_evaluate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: PolicyEvaluationRequest,
    ) -> Result<PolicyEvaluationResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/evaluate",
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
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpermissionproviders>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission/providers`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_providers_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<PolicyProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/providers",
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
    /// - `fields`
    /// - `name`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpermissionsearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission/search`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        name: Option<String>,
    ) -> Result<AbstractPolicyRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `permission`
    /// - `policy_id`
    /// - `resource`
    /// - `resource_type`
    /// - `scope`
    /// - `type_`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicy>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        permission: Option<bool>,
        policy_id: Option<String>,
        resource: Option<String>,
        resource_type: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<AbstractPolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = permission {
            builder = builder.query(&[("permission", v)]);
        }
        if let Some(v) = policy_id {
            builder = builder.query(&[("policyId", v)]);
        }
        if let Some(v) = resource {
            builder = builder.query(&[("resource", v)]);
        }
        if let Some(v) = resource_type {
            builder = builder.query(&[("resourceType", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicy>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/evaluate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicyevaluate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy/evaluate`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_evaluate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: PolicyEvaluationRequest,
    ) -> Result<PolicyEvaluationResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/evaluate",
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
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicyproviders>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy/providers`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_providers_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<PolicyProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/providers",
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
    /// - `fields`
    /// - `name`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicysearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy/search`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        name: Option<String>,
    ) -> Result<AbstractPolicyRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresource>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
    ) -> Result<TypeVec<ResourceRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `body`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverresource>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_post(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        body: ResourceRepresentation,
    ) -> Result<ResourceRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourcesearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/search`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
    ) -> Result<ResourceRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<ResourceRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
        body: ResourceRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// Returns response for future processing.
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// Returns response for future processing.
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/attributes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_idattributes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}/attributes`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_attributes_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/attributes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_idpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}/permissions`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<TypeVec<PolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_idscopes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}/scopes`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_scopes_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<TypeVec<ScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscope>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_get(
        &self,
        realm: &str,
        client_uuid: &str,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<String>,
        scope_id: Option<String>,
    ) -> Result<TypeVec<ScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = scope_id {
            builder = builder.query(&[("scopeId", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverscope>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `name`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopesearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/search`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        name: Option<String>,
    ) -> Result<TypeVec<ScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<ScopeRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}",
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
    /// - `scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
        body: ScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_idpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}/permissions`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<TypeVec<PolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/permissions",
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
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/resources`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_idresources>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}/resources`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_resources_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<TypeVec<ResourceRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/resources",
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
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/settings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serversettings>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/settings`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_settings_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ResourceServerRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/settings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
