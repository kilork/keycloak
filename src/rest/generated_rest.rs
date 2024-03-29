use reqwest::header::CONTENT_LENGTH;
use serde_json::Value;

use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Attack Detection</h4>

    /// Clear any user login failures for all users This can release temporary disabled users
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /admin/realms/{realm}/attack-detection/brute-force/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmattack_detectionbrute_forceusers>
    #[cfg(feature = "tag-attack-detection")]
    pub async fn realm_attack_detection_brute_force_users_delete(
        &self,
        realm: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmattack_detectionbrute_forceusersuserid>
    ///
    /// REST method: `GET /admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    #[cfg(feature = "tag-attack-detection")]
    pub async fn realm_attack_detection_brute_force_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /admin/realms/{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmattack_detectionbrute_forceusersuserid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    #[cfg(feature = "tag-attack-detection")]
    pub async fn realm_attack_detection_brute_force_users_with_user_id_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users/{user_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationauthenticator_providers>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationclient_authenticator_providers>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_client_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationconfig>
    #[cfg(feature = "tag-authentication-management")]
    #[deprecated]
    pub async fn realm_authentication_config_post(
        &self,
        realm: &str,
        body: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationconfig_descriptionproviderid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/config-description/{providerId}`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_config_description_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<AuthenticatorConfigInfoRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationconfigid>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_config_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticatorConfigRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmauthenticationconfigid>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_config_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/config/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete authenticator configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmauthenticationconfigid>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_config_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/authentication/config/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new authentication execution
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutions>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_executions_post(
        &self,
        realm: &str,
        body: AuthenticationExecutionRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/executions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationexecutionsexecutionid>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/executions/{executionId}`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_executions_with_execution_id_get(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete execution
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/executions/{execution_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmauthenticationexecutionsexecutionid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/authentication/executions/{executionId}`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_executions_with_execution_id_delete(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update execution with new configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    /// - `body`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions/{execution_id}/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidconfig>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/config`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_executions_with_execution_id_config_post(
        &self,
        realm: &str,
        execution_id: &str,
        body: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationexecutionsexecutionidconfigid>
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
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions/{execution_id}/lower-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidlower_priority>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/lower-priority`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_executions_with_execution_id_lower_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/lower-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Raise execution's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/executions/{execution_id}/raise-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationexecutionsexecutionidraise_priority>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/executions/{executionId}/raise-priority`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_executions_with_execution_id_raise_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/raise-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationflows>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_get(
        &self,
        realm: &str,
    ) -> Result<AuthenticationFlowRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationflows>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_post(
        &self,
        realm: &str,
        body: AuthenticationFlowRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Copy existing authentication flow under a new name The new name is given as 'newName' attribute of the passed JSON object
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: name of the existing authentication flow
    /// - `body`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows/{flow_alias}/copy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliascopy>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/copy`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_flow_alias_copy_post(
        &self,
        realm: &str,
        flow_alias: &str,
        body: TypeMap<String, String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/copy",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationflowsflowaliasexecutions>
    ///
    /// REST method: `GET /admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_flow_alias_executions_get(
        &self,
        realm: &str,
        flow_alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update authentication executions of a Flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Flow alias
    /// - `body`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/flows/{flow_alias}/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmauthenticationflowsflowaliasexecutions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_flow_alias_executions_put(
        &self,
        realm: &str,
        flow_alias: &str,
        body: AuthenticationExecutionInfoRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new authentication execution to a flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Alias of parent flow
    /// - `body`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows/{flow_alias}/executions/execution`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliasexecutionsexecution>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/execution`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_flow_alias_executions_execution_post(
        &self,
        realm: &str,
        flow_alias: &str,
        body: TypeMap<String, String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions/execution",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new flow with new execution to existing flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Alias of parent authentication flow
    /// - `body`
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/flows/{flow_alias}/executions/flow`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationflowsflowaliasexecutionsflow>
    ///
    /// REST method: `POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/flow`
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_flow_alias_executions_flow_post(
        &self,
        realm: &str,
        flow_alias: &str,
        body: TypeMap<String, String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions/flow",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationflowsid>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticationFlowRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmauthenticationflowsid>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: AuthenticationFlowRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete an authentication flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Flow id
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmauthenticationflowsid>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_flows_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationform_action_providers>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_form_action_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationform_providers>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_form_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationper_client_config_description>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_per_client_config_description_get(
        &self,
        realm: &str,
    ) -> Result<TypeMap<String, TypeVec<ConfigPropertyRepresentation>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/register-required-action`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationregister_required_action>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_register_required_action_post(
        &self,
        realm: &str,
        body: RequiredActionProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/register-required-action",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actions>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationrequired_actionsalias>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_required_actions_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<RequiredActionProviderRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Authentication Management`
    ///
    /// `PUT /admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmauthenticationrequired_actionsalias>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_required_actions_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        body: RequiredActionProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete required action
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmauthenticationrequired_actionsalias>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_required_actions_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Lower required action's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/required-actions/{alias}/lower-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationrequired_actionsaliaslower_priority>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_required_actions_with_alias_lower_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/lower-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Raise required action's priority
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /admin/realms/{realm}/authentication/required-actions/{alias}/raise-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmauthenticationrequired_actionsaliasraise_priority>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_required_actions_with_alias_raise_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}/raise-priority",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmauthenticationunregistered_required_actions>
    #[cfg(feature = "tag-authentication-management")]
    pub async fn realm_authentication_unregistered_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// - `id`
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/certificates/{attr}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidcertificatesattr>
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_id_certificates_with_attr_get(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}",
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
    /// - `id`
    /// - `attr`
    /// - `body`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/certificates/{attr}/download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidcertificatesattrdownload>
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_id_certificates_with_attr_download_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        body: KeyStoreConfig,
    ) -> Result<TypeString, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/download",
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
    /// - `id`
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/certificates/{attr}/generate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidcertificatesattrgenerate>
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_id_certificates_with_attr_generate_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/generate",
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
    /// - `id`
    /// - `attr`
    /// - `body`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/certificates/{attr}/generate-and-download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidcertificatesattrgenerate_and_download>
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_id_certificates_with_attr_generate_and_download_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        body: KeyStoreConfig,
    ) -> Result<TypeString, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/generate-and-download",
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
    /// - `id`
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/certificates/{attr}/upload`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidcertificatesattrupload>
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_id_certificates_with_attr_upload_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/upload",
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
    /// - `id`
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/certificates/{attr}/upload-certificate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidcertificatesattrupload_certificate>
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub async fn realm_clients_with_id_certificates_with_attr_upload_certificate_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/upload-certificate",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclients_initial_access>
    #[cfg(feature = "tag-client-initial-access")]
    pub async fn realm_clients_initial_access_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientInitialAccessPresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclients_initial_access>
    #[cfg(feature = "tag-client-initial-access")]
    pub async fn realm_clients_initial_access_post(
        &self,
        realm: &str,
        body: ClientInitialAccessCreatePresentation,
    ) -> Result<ClientInitialAccessPresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
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
    /// Resource: `Client Initial Access`
    ///
    /// `DELETE /admin/realms/{realm}/clients-initial-access/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclients_initial_accessid>
    #[cfg(feature = "tag-client-initial-access")]
    pub async fn realm_clients_initial_access_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients-initial-access/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_registration_policyproviders>
    #[cfg(feature = "tag-client-registration-policy")]
    pub async fn realm_client_registration_policy_providers_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-registration-policy/providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Client Role Mappings</h4>

    /// Get client-level role mappings for the user, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappingsclientsclient>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmgroupsidrole_mappingsclientsclient>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete client-level roles from user role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmgroupsidrole_mappingsclientsclient>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappingsclientsclientavailable>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}/available",
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
    /// - `id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappingsclientsclientcomposite>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client-level role mappings for the user, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappingsclientsclient>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_id_role_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidrole_mappingsclientsclient>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_id_role_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete client-level roles from user role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersidrole_mappingsclientsclient>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_id_role_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappingsclientsclientavailable>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_id_role_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}/available",
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
    /// - `id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappingsclientsclientcomposite>
    #[cfg(feature = "tag-client-role-mappings")]
    pub async fn realm_users_with_id_role_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}/composite",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopes>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/client-scopes", self.url))
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
    /// Resource: `Client Scopes`
    ///
    /// `POST /admin/realms/{realm}/client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_scopes>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_scopes_post(
        &self,
        realm: &str,
        body: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/client-scopes", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get representation of the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesid>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_scopes_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientScopeRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `PUT /admin/realms/{realm}/client-scopes/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclient_scopesid>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_scopes_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_scopesid>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_scopes_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templates>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_templates_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientInitialAccessPresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Client Scopes`
    ///
    /// `POST /admin/realms/{realm}/client-templates`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_templates>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_templates_post(
        &self,
        realm: &str,
        body: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-templates",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get representation of the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesid>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_templates_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientScopeRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `PUT /admin/realms/{realm}/client-templates/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclient_templatesid>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_templates_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_templatesid>
    #[cfg(feature = "tag-client-scopes")]
    pub async fn realm_client_templates_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclients>
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
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/clients", self.url))
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
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclients>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_post(
        &self,
        realm: &str,
        body: ClientRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/clients", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get representation of the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsid>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/clients/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsid>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ClientRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/clients/{id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsid>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}/clients/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the client secret
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/client-secret`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidclient_secret>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_client_secret_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/client-secret",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/client-secret`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidclient_secret>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_client_secret_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/client-secret",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/client-secret/rotated`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidclient_secretrotated>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_client_secret_rotated_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/client-secret/rotated",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/client-secret/rotated`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidclient_secretrotated>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_client_secret_rotated_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/client-secret/rotated",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get default client scopes.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsiddefault_client_scopes>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_default_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/default-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_scope_id`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}/default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsiddefault_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{id}/default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/default-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_scope_id`
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsiddefault_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{id}/default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/default-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create JSON with payload of example access token
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-access-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidevaluate_scopesgenerate_example_access_token>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_evaluate_scopes_generate_example_access_token_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<AccessToken, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-access-token",
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

    /// Create JSON with payload of example id token
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-id-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidevaluate_scopesgenerate_example_id_token>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_evaluate_scopes_generate_example_id_token_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<IDToken, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-id-token",
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

    /// Create JSON with payload of example user info
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-userinfo`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidevaluate_scopesgenerate_example_userinfo>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_evaluate_scopes_generate_example_userinfo_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-userinfo",
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
    /// - `id`
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/protocol-mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidevaluate_scopesprotocol_mappers>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_evaluate_scopes_protocol_mappers_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/protocol-mappers",
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
    /// - `id`
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/granted`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidevaluate_scopesscope_mappingsrolecontaineridgranted>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/granted`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/granted",
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
    /// - `id`
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/not-granted`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidevaluate_scopesscope_mappingsrolecontaineridnot_granted>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/not-granted`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/not-granted",
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
    /// - `id`
    /// - `provider_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/installation/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidinstallationprovidersproviderid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/installation/providers/{providerId}`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_installation_providers_with_provider_id_get(
        &self,
        realm: &str,
        id: &str,
        provider_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/installation/providers/{provider_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidmanagementpermissions>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/management/permissions",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsidmanagementpermissions>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/management/permissions",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/nodes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidnodes>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_nodes_post(
        &self,
        realm: &str,
        id: &str,
        body: TypeMap<String, String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/nodes",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Unregister a cluster node from the client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `node`
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/nodes/{node}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidnodesnode>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_nodes_with_node_delete(
        &self,
        realm: &str,
        id: &str,
        node: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/nodes/{node}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get application offline session count Returns a number of offline user sessions associated with this client { "count": number }
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/offline-session-count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidoffline_session_count>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_offline_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/offline-session-count",
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
    /// - `id`
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/offline-sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidoffline_sessions>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_offline_sessions_get(
        &self,
        realm: &str,
        id: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/offline-sessions",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidoptional_client_scopes>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_optional_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/optional-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_scope_id`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}/optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsidoptional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{id}/optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_scope_id`
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidoptional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{id}/optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Push the client's revocation policy to its admin URL If the client has an admin URL, push revocation policy to it.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidpush_revocation>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_push_revocation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/push-revocation",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/registration-access-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidregistration_access_token>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_registration_access_token_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/registration-access-token",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/service-account-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidservice_account_user>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_service_account_user_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<UserRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/service-account-user",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/session-count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidsession_count>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/session-count",
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
    /// - `id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/test-nodes-available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidtest_nodes_available>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_test_nodes_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/test-nodes-available",
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
    /// - `id`
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/user-sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsiduser_sessions>
    #[cfg(feature = "tag-clients")]
    pub async fn realm_clients_with_id_user_sessions_get(
        &self,
        realm: &str,
        id: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/user-sessions",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmcomponents>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_get(
        &self,
        realm: &str,
        name: Option<String>,
        parent: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<ComponentRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/components", self.url))
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
    /// Resource: `Component`
    ///
    /// `POST /admin/realms/{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmcomponents>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_post(
        &self,
        realm: &str,
        body: ComponentRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/components", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmcomponentsid>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ComponentRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/components/{id}",
                self.url
            ))
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
    /// Resource: `Component`
    ///
    /// `PUT /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmcomponentsid>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ComponentRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/components/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Component`
    ///
    /// `DELETE /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmcomponentsid>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/components/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmcomponentsidsub_component_types>
    #[cfg(feature = "tag-component")]
    pub async fn realm_components_with_id_sub_component_types_get(
        &self,
        realm: &str,
        id: &str,
        type_: Option<String>,
    ) -> Result<TypeVec<ComponentRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
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

    /// Get group hierarchy.  Only name and ids are returned.
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
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroups>
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
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/groups", self.url))
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
    /// Resource: `Groups`
    ///
    /// `POST /admin/realms/{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmgroups>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_post(
        &self,
        realm: &str,
        body: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/groups", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupscount>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_count_get(
        &self,
        realm: &str,
        search: Option<String>,
        top: Option<bool>,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/groups/count", self.url))
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
    /// - `id`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsid>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/groups/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update group, ignores subgroups.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /admin/realms/{realm}/groups/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmgroupsid>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/groups/{id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Groups`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmgroupsid>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}/groups/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Set or create child.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Groups`
    ///
    /// `POST /admin/realms/{realm}/groups/{id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmgroupsidchildren>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_children_post(
        &self,
        realm: &str,
        id: &str,
        body: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/groups/{id}/children",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidmanagementpermissions>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/management/permissions",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /admin/realms/{realm}/groups/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmgroupsidmanagementpermissions>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/groups/{id}/management/permissions",
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
    /// - `id`
    /// - `brief_representation`: Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidmembers>
    #[cfg(feature = "tag-groups")]
    pub async fn realm_groups_with_id_members_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/members",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmidentity_providerimport_config>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_import_config_post(
        &self,
        realm: &str,
        body: TypeMap<String, Value>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/identity-provider/import-config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get identity providers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstances>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<IdentityProviderRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
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
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstances>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_post(
        &self,
        realm: &str,
        body: IdentityProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesalias>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesalias>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        body: IdentityProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesalias>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Export public broker configuration for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `format`: Format to use
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasexport>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_export_get(
        &self,
        realm: &str,
        alias: &str,
        format: Option<String>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/export",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = format {
            builder = builder.query(&[("format", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_put(
        &self,
        realm: &str,
        alias: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmapper_types>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mapper_types_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<TypeMap<String, IdentityProviderMapperTypeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappers>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstancesaliasmappers>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_post(
        &self,
        realm: &str,
        alias: &str,
        body: IdentityProviderMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_get(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<IdentityProviderMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_put(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
        body: IdentityProviderMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a mapper for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_delete(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get identity providers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `provider_id`: Provider id
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmidentity_providerprovidersprovider_id>
    #[cfg(feature = "tag-identity-providers")]
    pub async fn realm_identity_provider_providers_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/identity-provider/providers/{provider_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmkeys>
    #[cfg(feature = "tag-key")]
    pub async fn realm_keys_get(
        &self,
        realm: &str,
    ) -> Result<KeysMetadataRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/keys", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    // <h4>Protocol Mappers</h4>

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_scopesidprotocol_mappersadd_models>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidprotocol_mappersmodels>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_scopesidprotocol_mappersmodels>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}",
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
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclient_scopesidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_scopesidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidprotocol_mappersprotocolprotocol>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_scopes_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/protocol/{protocol}",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_templatesidprotocol_mappersadd_models>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidprotocol_mappersmodels>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/models",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_templatesidprotocol_mappersmodels>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/models/{id}",
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
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/client-templates/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclient_templatesidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_templatesidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidprotocol_mappersprotocolprotocol>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_client_templates_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/protocol-mappers/protocol/{protocol}",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidprotocol_mappersadd_models>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidprotocol_mappersmodels>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidprotocol_mappersmodels>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}",
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
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidprotocol_mappersmodelsid>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidprotocol_mappersprotocolprotocol>
    #[cfg(feature = "tag-protocol-mappers")]
    pub async fn realm_clients_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/protocol/{protocol}",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealms>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn get(
        &self,
        brief_representation: Option<bool>,
    ) -> Result<RealmRepresentation, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms", self.url))
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
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealms>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn post(&self, body: RealmRepresentation) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealm>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_get(&self, realm: &str) -> Result<RealmRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}", self.url))
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
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealm>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_put(
        &self,
        realm: &str,
        body: RealmRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealm>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// - `date_from`
    /// - `date_to`
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmadmin_events>
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
        first: Option<i32>,
        max: Option<i32>,
        operation_types: Option<Vec<String>>,
        resource_path: Option<String>,
        resource_types: Option<Vec<String>>,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/admin-events", self.url))
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
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = operation_types {
            builder = builder.query(&[("operationTypes", v)]);
        }
        if let Some(v) = resource_path {
            builder = builder.query(&[("resourcePath", v)]);
        }
        if let Some(v) = resource_types {
            builder = builder.query(&[("resourceTypes", v)]);
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
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmadmin_events>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_admin_events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}/admin-events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_description_converter>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_description_converter_post(
        &self,
        realm: &str,
        body: String,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
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
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_policiespolicies>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_policies_get(
        &self,
        realm: &str,
    ) -> Result<ClientPoliciesRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-policies/policies",
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
    /// `PUT /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclient_policiespolicies>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_policies_put(
        &self,
        realm: &str,
        body: ClientPoliciesRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-policies/policies",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_policiesprofiles>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_profiles_get(
        &self,
        realm: &str,
        include_global_profiles: Option<bool>,
    ) -> Result<ClientProfilesRepresentation, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
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
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclient_policiesprofiles>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_policies_profiles_put(
        &self,
        realm: &str,
        body: ClientProfilesRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-policies/profiles",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_session_stats>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_client_session_stats_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-session-stats",
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
    /// `GET /admin/realms/{realm}/credential-registrators`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmcredential_registrators>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_credential_registrators_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeString>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/credential-registrators",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm default client scopes.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmdefault_default_client_scopes>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_default_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/default-default-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/default-default-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmdefault_groups>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_groups_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/default-groups", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-groups/{groupId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_groups_with_group_id_put(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/default-groups/{group_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-groups/{groupId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_groups_with_group_id_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/default-groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm optional client scopes.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmdefault_optional_client_scopes>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_optional_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get events Returns all events, or filters them based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client`: App or oauth client name
    /// - `date_from`: From date
    /// - `date_to`: To date
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmevents>
    #[cfg(feature = "tag-realms-admin")]
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_events_get(
        &self,
        realm: &str,
        client: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        first: Option<i32>,
        ip_address: Option<String>,
        max: Option<i32>,
        type_: Option<Vec<String>>,
        user: Option<String>,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/events", self.url))
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
            builder = builder.query(&[("type", v)]);
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
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmevents>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}/events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmeventsconfig>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_events_config_get(
        &self,
        realm: &str,
    ) -> Result<RealmEventsConfigRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/events/config", self.url))
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
    /// `PUT /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmeventsconfig>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_events_config_put(
        &self,
        realm: &str,
        body: RealmEventsConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/events/config", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroup_by_pathpath>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_group_by_path_with_path_get(
        &self,
        realm: &str,
        path: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmlocalization>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, String>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/localization", self.url))
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmlocalizationlocale>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_get(
        &self,
        realm: &str,
        locale: &str,
        use_realm_default_locale_fallback: Option<bool>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
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
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmlocalizationlocale>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_post(
        &self,
        realm: &str,
        locale: &str,
        body: TypeMap<String, String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocale>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_delete(
        &self,
        realm: &str,
        locale: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmlocalizationlocalekey>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_with_key_get(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
    ) -> Result<TypeString, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmlocalizationlocalekey>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_with_key_put(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
        body: String,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .body(body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocalekey>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_localization_with_locale_with_key_delete(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmlogout_all>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_logout_all_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/logout-all", self.url))
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmpartial_export>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_partial_export_post(
        &self,
        realm: &str,
        export_clients: Option<bool>,
        export_groups_and_roles: Option<bool>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/partial-export", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = export_clients {
            builder = builder.query(&[("exportClients", v)]);
        }
        if let Some(v) = export_groups_and_roles {
            builder = builder.query(&[("exportGroupsAndRoles", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmpartialimport>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_partial_import_post(
        &self,
        realm: &str,
        body: String,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/partialImport", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmpush_revocation>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_push_revocation_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/push-revocation",
                self.url
            ))
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
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/sessions/{session}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmsessionssession>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_sessions_with_session_delete(
        &self,
        realm: &str,
        session: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/sessions/{session}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Test SMTP connection with current logged in user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/testSMTPConnection`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmtestsmtpconnection>
    #[cfg(feature = "tag-realms-admin")]
    #[deprecated]
    pub async fn realm_test_smtp_connection_post(
        &self,
        realm: &str,
        body: TypeMap<String, String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/testSMTPConnection",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusers_management_permissions>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_users_management_permissions_get(
        &self,
        realm: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusers_management_permissions>
    #[cfg(feature = "tag-realms-admin")]
    pub async fn realm_users_management_permissions_put(
        &self,
        realm: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
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
    /// - `id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappings>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_id_role_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings",
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
    /// - `id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappingsrealm>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_id_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/groups/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmgroupsidrole_mappingsrealm>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_id_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmgroupsidrole_mappingsrealm>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappingsrealmavailable>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm/available",
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
    /// - `id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmgroupsidrole_mappingsrealmcomposite>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_groups_with_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm/composite",
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
    /// - `id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappings>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_id_role_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings",
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
    /// - `id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappingsrealm>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_id_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidrole_mappingsrealm>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_id_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersidrole_mappingsrealm>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappingsrealmavailable>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm/available",
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
    /// - `id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidrole_mappingsrealmcomposite>
    #[cfg(feature = "tag-role-mapper")]
    pub async fn realm_users_with_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm/composite",
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
    /// - `id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidroles>
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidroles>
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_post(
        &self,
        realm: &str,
        id: &str,
        body: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}",
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
    /// - `id`
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsidrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{id}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_put(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        body: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{id}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_delete(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get composites of the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites",
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
    /// - `id`
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{id}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{id}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_namecompositesclientsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/composites/clients/{clientUuid}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        id: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites/clients/{client_uuid}",
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
    /// - `id`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/composites/realm`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites/realm",
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
    /// - `id`
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/groups`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/groups",
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
    /// - `id`
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/management/permissions",
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
    /// - `id`
    /// - `role_name`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{id}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmclientsidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{id}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/management/permissions",
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
    /// - `id`
    /// - `role_name`: the role name.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/users`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_clients_with_id_roles_with_role_name_users_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/users",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmroles>
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/roles", self.url))
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
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmroles>
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_post(
        &self,
        realm: &str,
        body: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/roles", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_put(
        &self,
        realm: &str,
        role_name: &str,
        body: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_delete(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}/composites`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesclientsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/clients/{clientUuid}`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/realm`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_namegroups>
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
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}/management/permissions`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        role_name: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
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
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/users`
    #[cfg(feature = "tag-roles")]
    pub async fn realm_roles_with_role_name_users_get(
        &self,
        realm: &str,
        role_name: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}/users",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles-by-id/{role-id}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_put(
        &self,
        realm: &str,
        role_id: &str,
        body: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles-by-id/{role-id}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_delete(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcomposites>
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
        let mut builder = self
            .client
            .get(&format!(
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
    /// Resource: `Roles (by ID)`
    ///
    /// `POST /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_post(
        &self,
        realm: &str,
        role_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a set of roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: Role id
    /// - `body`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_delete(
        &self,
        realm: &str,
        role_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcompositesclientsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites/clients/{clientUuid}`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites/realm`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_composites_realm_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_management_permissions_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmroles_by_idrole_idmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    #[cfg(feature = "tag-roles-by-id")]
    pub async fn realm_roles_by_id_with_role_id_management_permissions_put(
        &self,
        realm: &str,
        role_id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
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
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappings>
    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub async fn realm_client_scopes_with_id_scope_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings",
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
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}",
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
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_scopesidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove client-level roles from the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_scopesidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappingsclientsclientavailable>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}/available",
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
    /// - `id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappingsclientsclientcomposite>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite",
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
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_scopesidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a set of realm-level roles from the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_scopesidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappingsrealmavailable>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm/available",
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
    /// - `id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_scopesidscope_mappingsrealmcomposite>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm/composite",
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
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappings>
    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub async fn realm_client_templates_with_id_scope_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings",
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
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}",
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
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_templatesidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove client-level roles from the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_templatesidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappingsclientsclientavailable>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}/available",
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
    /// - `id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappingsclientsclientcomposite>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/clients/{client}/composite",
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
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/realm",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclient_templatesidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a set of realm-level roles from the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclient_templatesidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappingsrealmavailable>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/realm/available",
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
    /// - `id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclient_templatesidscope_mappingsrealmcomposite>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_client_templates_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/client-templates/{id}/scope-mappings/realm/composite",
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
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappings>
    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub async fn realm_clients_with_id_scope_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings",
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
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}",
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
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove client-level roles from the client's scope.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidscope_mappingsclientsclient>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappingsclientsclientavailable>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}/available",
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
    /// - `id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappingsclientsclientcomposite>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}/composite",
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
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /admin/realms/{realm}/clients/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmclientsidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a set of realm-level roles from the client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmclientsidscope_mappingsrealm>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client's scope
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappingsrealmavailable>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm/available",
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
    /// - `id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /admin/realms/{realm}/clients/{id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmclientsidscope_mappingsrealmcomposite>
    #[cfg(feature = "tag-scope-mappings")]
    pub async fn realm_clients_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm/composite",
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusers>
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
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/users", self.url))
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
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusers>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_post(
        &self,
        realm: &str,
        body: UserRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/users", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmuserscount>
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
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/users/count", self.url))
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersprofile>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_profile_get(&self, realm: &str) -> Result<TypeString, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/users/profile", self.url))
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
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersprofile>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_profile_put(
        &self,
        realm: &str,
        body: String,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/users/profile", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get representation of the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `user_profile_metadata`: Indicates if the user profile metadata should be added to the response
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersid>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_get(
        &self,
        realm: &str,
        id: &str,
        user_profile_metadata: Option<bool>,
    ) -> Result<UserRepresentation, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/users/{id}", self.url))
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersid>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: UserRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/users/{id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersid>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}/users/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/configured-user-storage-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidconfigured_user_storage_credential_types>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_configured_user_storage_credential_types_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/configured-user-storage-credential-types",
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
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/consents`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidconsents>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_consents_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/consents",
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
    /// - `id`
    /// - `client`: Client id
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}/consents/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersidconsentsclient>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_consents_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/consents/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/credentials`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidcredentials>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_credentials_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/credentials",
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
    /// - `id`
    /// - `credential_id`
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}/credentials/{credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersidcredentialscredentialid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{id}/credentials/{credentialId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_credentials_with_credential_id_delete(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/credentials/{credential_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a position behind another credential
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `credential_id`: The credential to move
    /// - `new_previous_credential_id`: The credential that will be the previous element in the list. If set to null, the moved credential will be the first element in the list.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidcredentialscredentialidmoveafternewpreviouscredentialid>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
        new_previous_credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a first position in the credentials list of the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `credential_id`: The credential to move
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/credentials/{credential_id}/moveToFirst`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidcredentialscredentialidmovetofirst>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{id}/credentials/{credentialId}/moveToFirst`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_credentials_with_credential_id_move_to_first_post(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/credentials/{credential_id}/moveToFirst",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update a credential label for a user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `credential_id`
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/credentials/{credential_id}/userLabel`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersidcredentialscredentialiduserlabel>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{id}/credentials/{credentialId}/userLabel`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_credentials_with_credential_id_user_label_put(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
        body: String,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/credentials/{credential_id}/userLabel",
                self.url
            ))
            .body(body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Disable all credentials for a user of a specific type
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/disable-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersiddisable_credential_types>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_disable_credential_types_put(
        &self,
        realm: &str,
        id: &str,
        body: Vec<String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/disable-credential-types",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Send an email to the user with a link they can click to execute particular actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/execute-actions-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersidexecute_actions_email>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_execute_actions_email_put(
        &self,
        realm: &str,
        id: &str,
        client_id: Option<String>,
        lifespan: Option<i32>,
        redirect_uri: Option<String>,
        body: Vec<String>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/execute-actions-email",
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
        error_check(response).await?;
        Ok(())
    }

    /// Get social logins associated with the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/federated-identity`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidfederated_identity>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_federated_identity_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/federated-identity",
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
    /// - `id`
    /// - `provider`: Social login provider id
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidfederated_identityprovider>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_federated_identity_with_provider_post(
        &self,
        realm: &str,
        id: &str,
        provider: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/federated-identity/{provider}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a social login provider from user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `provider`: Social login provider id
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersidfederated_identityprovider>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_federated_identity_with_provider_delete(
        &self,
        realm: &str,
        id: &str,
        provider: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/federated-identity/{provider}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidgroups>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_groups_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<Value, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/groups",
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
    /// - `id`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidgroupscount>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_groups_count_get(
        &self,
        realm: &str,
        id: &str,
        search: Option<String>,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/groups/count",
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
    /// - `id`
    /// - `group_id`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersidgroupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{id}/groups/{groupId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_groups_with_group_id_put(
        &self,
        realm: &str,
        id: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/groups/{group_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `group_id`
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_delete_adminrealmsrealmusersidgroupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{id}/groups/{groupId}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_groups_with_group_id_delete(
        &self,
        realm: &str,
        id: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Impersonate the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/impersonation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidimpersonation>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_impersonation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/impersonation",
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
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{id}/logout`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_post_adminrealmsrealmusersidlogout>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_logout_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/logout",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get offline sessions associated with the user and client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_uuid`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/offline-sessions/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidoffline_sessionsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{id}/offline-sessions/{clientUuid}`
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_offline_sessions_with_client_uuid_get(
        &self,
        realm: &str,
        id: &str,
        client_uuid: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/offline-sessions/{client_uuid}",
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
    /// - `id`
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/reset-password`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersidreset_password>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_reset_password_put(
        &self,
        realm: &str,
        id: &str,
        body: CredentialRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/reset-password",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Send an email to the user with a link they can click to reset their password.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_id`: client id
    /// - `redirect_uri`: redirect uri
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/reset-password-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersidreset_password_email>
    #[cfg(feature = "tag-users")]
    #[deprecated]
    pub async fn realm_users_with_id_reset_password_email_put(
        &self,
        realm: &str,
        id: &str,
        client_id: Option<String>,
        redirect_uri: Option<String>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/reset-password-email",
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
        error_check(response).await?;
        Ok(())
    }

    /// Send an email-verification email to the user An email contains a link the user can click to verify their email address.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `client_id`: Client id
    /// - `redirect_uri`: Redirect uri
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{id}/send-verify-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_put_adminrealmsrealmusersidsend_verify_email>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_send_verify_email_put(
        &self,
        realm: &str,
        id: &str,
        client_id: Option<String>,
        redirect_uri: Option<String>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/send-verify-email",
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
        error_check(response).await?;
        Ok(())
    }

    /// Get sessions associated with the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{id}/sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/22.0.3/rest-api/index.html#_get_adminrealmsrealmusersidsessions>
    #[cfg(feature = "tag-users")]
    pub async fn realm_users_with_id_sessions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Value, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/users/{id}/sessions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
