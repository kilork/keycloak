use serde_json::{json, Value};
use std::collections::HashMap;

use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    /// Clear any user login failures for all users   This can release temporary disabled users
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /{realm}/attack-detection/brute-force/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_clearallbruteforce>
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
    /// `GET /{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_bruteforceuserstatus>
    ///
    /// REST method: `GET /{realm}/attack-detection/brute-force/users/{userId}`
    pub async fn realm_attack_detection_brute_force_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// Clear any user login failures for the user   This can release temporary disabled user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_clearbruteforceforuser>
    ///
    /// REST method: `DELETE /{realm}/attack-detection/brute-force/users/{userId}`
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

    /// Get authenticator providers   Returns a stream of authenticator providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/authenticator-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getauthenticatorproviders>
    pub async fn realm_authentication_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Get client authenticator providers   Returns a stream of client authenticator providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/client-authenticator-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientauthenticatorproviders>
    pub async fn realm_authentication_client_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Get authenticator provider’s configuration description
    ///
    /// Parameters:
    ///
    /// - `provider_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/config-description/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getauthenticatorconfigdescription>
    ///
    /// REST method: `GET /{realm}/authentication/config-description/{providerId}`
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
    /// - `id`: Configuration id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getauthenticatorconfig>
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
    /// - `id`: Configuration id
    /// - `realm`: realm name (not id!)
    /// - `rep`: JSON describing new state of authenticator configuration
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updateauthenticatorconfig>
    pub async fn realm_authentication_config_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/config/{id}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete authenticator configuration
    ///
    /// Parameters:
    ///
    /// - `id`: Configuration id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /{realm}/authentication/config/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removeauthenticatorconfig>
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
    /// - `execution`: JSON model describing authentication execution
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addexecution>
    pub async fn realm_authentication_executions_post(
        &self,
        realm: &str,
        execution: AuthenticationExecutionRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/executions",
                self.url
            ))
            .json(&execution)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get Single Execution
    ///
    /// Parameters:
    ///
    /// - `execution_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/executions/{execution_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getexecution>
    ///
    /// REST method: `GET /{realm}/authentication/executions/{executionId}`
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
    /// - `execution_id`: Execution id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /{realm}/authentication/executions/{execution_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removeexecution>
    ///
    /// REST method: `DELETE /{realm}/authentication/executions/{executionId}`
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
    /// - `execution_id`: Execution id
    /// - `realm`: realm name (not id!)
    /// - `json`: JSON with new configuration
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/executions/{execution_id}/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_newexecutionconfig>
    ///
    /// REST method: `POST /{realm}/authentication/executions/{executionId}/config`
    pub async fn realm_authentication_executions_with_execution_id_config_post(
        &self,
        realm: &str,
        execution_id: &str,
        json: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/executions/{execution_id}/config",
                self.url
            ))
            .json(&json)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Lower execution’s priority
    ///
    /// Parameters:
    ///
    /// - `execution_id`: Execution id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/executions/{execution_id}/lower-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_lowerpriority>
    ///
    /// REST method: `POST /{realm}/authentication/executions/{executionId}/lower-priority`
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

    /// Raise execution’s priority
    ///
    /// Parameters:
    ///
    /// - `execution_id`: Execution id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/executions/{execution_id}/raise-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_raisepriority>
    ///
    /// REST method: `POST /{realm}/authentication/executions/{executionId}/raise-priority`
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

    /// Create a new authentication flow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `flow`: Authentication flow representation
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/flows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createflow>
    pub async fn realm_authentication_flows_post(
        &self,
        realm: &str,
        flow: AuthenticationFlowRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows",
                self.url
            ))
            .json(&flow)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication flows   Returns a stream of authentication flows.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/flows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getflows>
    pub async fn realm_authentication_flows_get(
        &self,
        realm: &str,
    ) -> Result<Vec<AuthenticationFlowRepresentation>, KeycloakError> {
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

    /// Copy existing authentication flow under a new name   The new name is given as 'newName' attribute of the passed JSON object
    ///
    /// Parameters:
    ///
    /// - `flow_alias`: Name of the existing authentication flow
    /// - `realm`: realm name (not id!)
    /// - `data`: JSON containing 'newName' attribute
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/flows/{flow_alias}/copy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_copy>
    ///
    /// REST method: `POST /{realm}/authentication/flows/{flowAlias}/copy`
    pub async fn realm_authentication_flows_with_flow_alias_copy_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/copy",
                self.url
            ))
            .json(&data)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication executions for a flow
    ///
    /// Parameters:
    ///
    /// - `flow_alias`: Flow alias
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/flows/{flow_alias}/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getexecutions>
    ///
    /// REST method: `GET /{realm}/authentication/flows/{flowAlias}/executions`
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
    /// - `flow_alias`: Flow alias
    /// - `realm`: realm name (not id!)
    /// - `rep`: AuthenticationExecutionInfoRepresentation
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /{realm}/authentication/flows/{flow_alias}/executions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updateexecutions>
    ///
    /// REST method: `PUT /{realm}/authentication/flows/{flowAlias}/executions`
    pub async fn realm_authentication_flows_with_flow_alias_executions_put(
        &self,
        realm: &str,
        flow_alias: &str,
        rep: AuthenticationExecutionInfoRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new authentication execution to a flow
    ///
    /// Parameters:
    ///
    /// - `flow_alias`: Alias of parent flow
    /// - `realm`: realm name (not id!)
    /// - `data`: New execution JSON data containing 'provider' attribute
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/flows/{flow_alias}/executions/execution`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addexecutiontoflow>
    ///
    /// REST method: `POST /{realm}/authentication/flows/{flowAlias}/executions/execution`
    pub async fn realm_authentication_flows_with_flow_alias_executions_execution_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions/execution",
                self.url
            ))
            .json(&data)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new flow with new execution to existing flow
    ///
    /// Parameters:
    ///
    /// - `flow_alias`: Alias of parent authentication flow
    /// - `realm`: realm name (not id!)
    /// - `data`: New authentication flow / execution JSON data containing 'alias', 'type', 'provider', and 'description' attributes
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/flows/{flow_alias}/executions/flow`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addexecutionflow>
    ///
    /// REST method: `POST /{realm}/authentication/flows/{flowAlias}/executions/flow`
    pub async fn realm_authentication_flows_with_flow_alias_executions_flow_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{flow_alias}/executions/flow",
                self.url
            ))
            .json(&data)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication flow for id
    ///
    /// Parameters:
    ///
    /// - `id`: Flow id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getflow>
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `flow`: Authentication flow representation
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updateflow>
    pub async fn realm_authentication_flows_with_id_put(
        &self,
        realm: &str,
        id: &str,
        flow: AuthenticationFlowRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/flows/{id}",
                self.url
            ))
            .json(&flow)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete an authentication flow
    ///
    /// Parameters:
    ///
    /// - `id`: Flow id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /{realm}/authentication/flows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteflow>
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

    /// Get form action providers   Returns a stream of form action providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/form-action-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getformactionproviders>
    pub async fn realm_authentication_form_action_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Get form providers   Returns a stream of form providers.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/form-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getformproviders>
    pub async fn realm_authentication_form_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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
    /// `GET /{realm}/authentication/per-client-config-description`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getperclientconfigdescription>
    pub async fn realm_authentication_per_client_config_description_get(
        &self,
        realm: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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
    /// - `data`: JSON containing 'providerId', and 'name' attributes.
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/register-required-action`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_registerrequiredaction>
    pub async fn realm_authentication_register_required_action_post(
        &self,
        realm: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/authentication/register-required-action",
                self.url
            ))
            .json(&data)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get required actions   Returns a stream of required actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/required-actions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrequiredactions>
    pub async fn realm_authentication_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<Vec<RequiredActionProviderRepresentation>, KeycloakError> {
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
    /// - `alias`: Alias of required action
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrequiredaction>
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
    /// - `alias`: Alias of required action
    /// - `realm`: realm name (not id!)
    /// - `rep`: JSON describing new state of required action
    ///
    /// Resource: `Authentication Management`
    ///
    /// `PUT /{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updaterequiredaction>
    pub async fn realm_authentication_required_actions_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        rep: RequiredActionProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/authentication/required-actions/{alias}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete required action
    ///
    /// Parameters:
    ///
    /// - `alias`: Alias of required action
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `DELETE /{realm}/authentication/required-actions/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removerequiredaction>
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

    /// Lower required action’s priority
    ///
    /// Parameters:
    ///
    /// - `alias`: Alias of required action
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/required-actions/{alias}/lower-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_lowerrequiredactionpriority>
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

    /// Raise required action’s priority
    ///
    /// Parameters:
    ///
    /// - `alias`: Alias of required action
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `POST /{realm}/authentication/required-actions/{alias}/raise-priority`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_raiserequiredactionpriority>
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

    /// Get unregistered required actions   Returns a stream of unregistered required actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Authentication Management`
    ///
    /// `GET /{realm}/authentication/unregistered-required-actions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getunregisteredrequiredactions>
    pub async fn realm_authentication_unregistered_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Get key info
    ///
    /// Parameters:
    ///
    /// - `attr`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `GET /{realm}/clients/{id}/certificates/{attr}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getkeyinfo>
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
    /// - `attr`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `config`: Keystore configuration as JSON
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /{realm}/clients/{id}/certificates/{attr}/download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getkeystore>
    pub async fn realm_clients_with_id_certificates_with_attr_download_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        config: KeyStoreConfig,
    ) -> Result<Vec<u8>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/download",
                self.url
            ))
            .json(&config)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.bytes().await?.to_vec())
    }

    /// Generate a new certificate with new key pair
    ///
    /// Parameters:
    ///
    /// - `attr`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /{realm}/clients/{id}/certificates/{attr}/generate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_generate>
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

    /// Generate a new keypair and certificate, and get the private key file   Generates a keypair and certificate and serves the private key in a specified keystore format.
    ///
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    ///
    /// Parameters:
    ///
    /// - `attr`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `config`: Keystore configuration as JSON
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /{realm}/clients/{id}/certificates/{attr}/generate-and-download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_generateandgetkeystore>
    pub async fn realm_clients_with_id_certificates_with_attr_generate_and_download_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        config: KeyStoreConfig,
    ) -> Result<Vec<u8>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/generate-and-download",
                self.url
            ))
            .json(&config)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.bytes().await?.to_vec())
    }

    /// Upload certificate and eventually private key
    ///
    /// Parameters:
    ///
    /// - `attr`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `input`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /{realm}/clients/{id}/certificates/{attr}/upload`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_uploadjks>
    pub async fn realm_clients_with_id_certificates_with_attr_upload_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        input: &[u8],
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/upload",
                self.url
            ))
            .form(&json!({
                "input": input,
            }))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Upload only certificate, not private key
    ///
    /// Parameters:
    ///
    /// - `attr`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `input`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /{realm}/clients/{id}/certificates/{attr}/upload-certificate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_uploadjkscertificate>
    pub async fn realm_clients_with_id_certificates_with_attr_upload_certificate_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        input: &[u8],
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/certificates/{attr}/upload-certificate",
                self.url
            ))
            .form(&json!({
                "input": input,
            }))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new initial access token.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `config`
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `POST /{realm}/clients-initial-access`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_create>
    pub async fn realm_clients_initial_access_post(
        &self,
        realm: &str,
        config: ClientInitialAccessCreatePresentation,
    ) -> Result<ClientInitialAccessPresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients-initial-access",
                self.url
            ))
            .json(&config)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `GET /{realm}/clients-initial-access`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_list>
    pub async fn realm_clients_initial_access_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientInitialAccessPresentation>, KeycloakError> {
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

    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Initial Access`
    ///
    /// `DELETE /{realm}/clients-initial-access/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_delete>
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

    /// Base path for retrieve providers with the configProperties properly filled
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Registration Policy`
    ///
    /// `GET /{realm}/client-registration-policy/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getproviders>
    pub async fn realm_client_registration_policy_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Add client-level roles to the user role mapping
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /{realm}/groups/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addclientrolemapping>
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level role mappings for the user, and the app
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientrolemappings>
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Delete client-level roles from user role mapping
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /{realm}/groups/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteclientrolemapping>
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailableclientrolemappings>
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective client-level role mappings   This recurses any composite roles
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositeclientrolemappings>
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Add client-level roles to the user role mapping
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /{realm}/users/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addclientrolemapping>
    pub async fn realm_users_with_id_role_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level role mappings for the user, and the app
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /{realm}/users/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientrolemappings>
    pub async fn realm_users_with_id_role_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Delete client-level roles from user role mapping
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /{realm}/users/{id}/role-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteclientrolemapping>
    pub async fn realm_users_with_id_role_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /{realm}/users/{id}/role-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailableclientrolemappings>
    pub async fn realm_users_with_id_role_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective client-level role mappings   This recurses any composite roles
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /{realm}/users/{id}/role-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositeclientrolemappings>
    pub async fn realm_users_with_id_role_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Create a new client scope   Client Scope’s name must be unique!
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `POST /{realm}/client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createclientscope>
    pub async fn realm_client_scopes_post(
        &self,
        realm: &str,
        rep: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/client-scopes", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client scopes belonging to the realm   Returns a list of client scopes belonging to the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /{realm}/client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientscopes>
    pub async fn realm_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/client-scopes", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get representation of the client scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Scopes`
    ///
    /// `GET /{realm}/client-scopes/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientscope>
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
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Client Scopes`
    ///
    /// `PUT /{realm}/client-scopes/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_client_scopes_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Scopes`
    ///
    /// `DELETE /{realm}/client-scopes/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteclientscope>
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

    /// Create a new client   Client’s client_id must be unique!
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Clients`
    ///
    /// `POST /{realm}/clients`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createclient>
    pub async fn realm_clients_post(
        &self,
        realm: &str,
        rep: ClientRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/clients", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get clients belonging to the realm.
    ///
    /// If a client can’t be retrieved from the storage due to a problem with the underlying storage,  it is silently removed from the returned list.  This ensures that concurrent modifications to the list don’t prevent callers from retrieving this list.
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
    /// `GET /{realm}/clients`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclients>
    pub async fn realm_clients_get(
        &self,
        realm: &str,
        client_id: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<bool>,
        viewable_only: Option<bool>,
    ) -> Result<Vec<ClientRepresentation>, KeycloakError> {
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

    /// Get representation of the client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclient>
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /{realm}/clients/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_clients_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ClientRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/clients/{id}", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /{realm}/clients/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteclient>
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

    /// Generate a new secret for the client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `POST /{realm}/clients/{id}/client-secret`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_regeneratesecret>
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

    /// Get the client secret
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/client-secret`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientsecret>
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

    /// Get the rotated client secret
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/client-secret/rotated`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientrotatedsecret>
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /{realm}/clients/{id}/client-secret/rotated`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_invalidaterotatedsecret>
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

    /// Get default client scopes.
    ///
    /// Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getdefaultclientscopes>
    pub async fn realm_clients_with_id_default_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
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

    /// PUT /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /{realm}/clients/{id}/default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_adddefaultclientscope>
    ///
    /// REST method: `PUT /{realm}/clients/{id}/default-client-scopes/{clientScopeId}`
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
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /{realm}/clients/{id}/default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removedefaultclientscope>
    ///
    /// REST method: `DELETE /{realm}/clients/{id}/default-client-scopes/{clientScopeId}`
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/evaluate-scopes/generate-example-access-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_generateexampleaccesstoken>
    pub async fn realm_clients_with_id_evaluate_scopes_generate_example_access_token_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<AccessToken, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/generate-example-access-token", self.url))
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/evaluate-scopes/generate-example-id-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_generateexampleidtoken>
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `scope`
    /// - `user_id`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/evaluate-scopes/generate-example-userinfo`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_generateexampleuserinfo>
    pub async fn realm_clients_with_id_evaluate_scopes_generate_example_userinfo_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
        user_id: Option<String>,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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
    /// This means  protocol mappers assigned to this client directly and protocol mappers assigned to all client scopes of this client.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/evaluate-scopes/protocol-mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgrantedprotocolmappers>
    pub async fn realm_clients_with_id_evaluate_scopes_protocol_mappers_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<String>,
    ) -> Result<Vec<ClientScopeEvaluateResourceProtocolMapperEvaluationRepresentation>, KeycloakError>
    {
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
    /// This contains scope mappings, which this client has directly, as well as scope mappings, which are granted to all client scopes,  which are linked with this client.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/granted`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgrantedscopemappings>
    ///
    /// REST method: `GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/granted`
    pub async fn realm_clients_with_id_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/granted", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get roles, which this client doesn’t have scope for and can’t have them in the accessToken issued for him.
    ///
    /// Defacto all the  other roles of particular role container, which are not in {@link #getGrantedScopeMappings()}
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/not-granted`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getnotgrantedscopemappings>
    ///
    /// REST method: `GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/not-granted`
    pub async fn realm_clients_with_id_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/clients/{id}/evaluate-scopes/scope-mappings/{role_container_id}/not-granted", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/clients/{id}/installation/providers/{providerId}
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `provider_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/installation/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getinstallationprovider>
    ///
    /// REST method: `GET /{realm}/clients/{id}/installation/providers/{providerId}`
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmanagementpermissions>
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `ref`
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /{realm}/clients/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setmanagementpermissionsenabled>
    pub async fn realm_clients_with_id_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/management/permissions",
                self.url
            ))
            .json(&ref_)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a cluster node with the client   Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle  by sending registration request to Keycloak
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `form_params`
    ///
    /// Resource: `Clients`
    ///
    /// `POST /{realm}/clients/{id}/nodes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_registernode>
    pub async fn realm_clients_with_id_nodes_post(
        &self,
        realm: &str,
        id: &str,
        form_params: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/nodes",
                self.url
            ))
            .json(&form_params)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Unregister a cluster node from the client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `node`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /{realm}/clients/{id}/nodes/{node}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_unregisternode>
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

    /// Get application offline session count   Returns a number of offline user sessions associated with this client   {      "count": number  }
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/offline-session-count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getofflinesessioncount>
    pub async fn realm_clients_with_id_offline_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// Get offline sessions for client   Returns a list of offline user sessions associated with this client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/offline-sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getofflineusersessions>
    pub async fn realm_clients_with_id_offline_sessions_get(
        &self,
        realm: &str,
        id: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Get optional client scopes.
    ///
    /// Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getoptionalclientscopes>
    pub async fn realm_clients_with_id_optional_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
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

    /// PUT /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `PUT /{realm}/clients/{id}/optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addoptionalclientscope>
    ///
    /// REST method: `PUT /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}`
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
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `DELETE /{realm}/clients/{id}/optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removeoptionalclientscope>
    ///
    /// REST method: `DELETE /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}`
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

    /// Push the client’s revocation policy to its admin URL   If the client has an admin URL, push revocation policy to it.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `POST /{realm}/clients/{id}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_pushrevocation>
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `POST /{realm}/clients/{id}/registration-access-token`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_regenerateregistrationaccesstoken>
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/service-account-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getserviceaccountuser>
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

    /// Get application session count   Returns a number of user sessions associated with this client   {      "count": number  }
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/session-count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getapplicationsessioncount>
    pub async fn realm_clients_with_id_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// Test if registered cluster nodes are available   Tests availability by sending 'ping' request to all cluster nodes.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/test-nodes-available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_testnodesavailable>
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

    /// Get user sessions for client   Returns a list of user sessions associated with this client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Clients`
    ///
    /// `GET /{realm}/clients/{id}/user-sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getusersessions>
    pub async fn realm_clients_with_id_user_sessions_get(
        &self,
        realm: &str,
        id: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Component`
    ///
    /// `POST /{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_create>
    pub async fn realm_components_post(
        &self,
        realm: &str,
        rep: ComponentRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/components", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `name`
    /// - `parent`
    /// - `type`
    ///
    /// Resource: `Component`
    ///
    /// `GET /{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcomponents>
    pub async fn realm_components_get(
        &self,
        realm: &str,
        name: Option<String>,
        parent: Option<String>,
        type_: Option<String>,
    ) -> Result<Vec<ComponentRepresentation>, KeycloakError> {
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Component`
    ///
    /// `GET /{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcomponent>
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Component`
    ///
    /// `PUT /{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updatecomponent>
    pub async fn realm_components_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ComponentRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/components/{id}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Component`
    ///
    /// `DELETE /{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removecomponent>
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `type`
    ///
    /// Resource: `Component`
    ///
    /// `GET /{realm}/components/{id}/sub-component-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getsubcomponentconfig>
    pub async fn realm_components_with_id_sub_component_types_get(
        &self,
        realm: &str,
        id: &str,
        type_: Option<String>,
    ) -> Result<Vec<ComponentRepresentation>, KeycloakError> {
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

    /// create or add a top level realm groupSet or create child.
    ///
    /// This will update the group and set the parent if it exists. Create it and set the parent  if the group doesn’t exist.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Groups`
    ///
    /// `POST /{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addtoplevelgroup>
    pub async fn realm_groups_post(
        &self,
        realm: &str,
        rep: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/groups", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get group hierarchy.
    ///
    /// Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `exact`
    /// - `first`
    /// - `max`
    /// - `q`
    /// - `search`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroups>
    pub async fn realm_groups_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
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
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    /// `GET /{realm}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroupcount>
    pub async fn realm_groups_count_get(
        &self,
        realm: &str,
        search: Option<String>,
        top: Option<bool>,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Groups`
    ///
    /// `GET /{realm}/groups/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroup>
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /{realm}/groups/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updategroup>
    pub async fn realm_groups_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/groups/{id}", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Groups`
    ///
    /// `DELETE /{realm}/groups/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deletegroup>
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
    /// This will just set the parent if it exists. Create it and set the parent  if the group doesn’t exist.
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Groups`
    ///
    /// `POST /{realm}/groups/{id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addchild>
    pub async fn realm_groups_with_id_children_post(
        &self,
        realm: &str,
        id: &str,
        rep: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/groups/{id}/children",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Groups`
    ///
    /// `GET /{realm}/groups/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmanagementpermissions>
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
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `ref`
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /{realm}/groups/{id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setmanagementpermissionsenabled>
    pub async fn realm_groups_with_id_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/groups/{id}/management/permissions",
                self.url
            ))
            .json(&ref_)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get users   Returns a stream of users, filtered according to query parameters
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Groups`
    ///
    /// `GET /{realm}/groups/{id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmembers>
    pub async fn realm_groups_with_id_members_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserRepresentation>, KeycloakError> {
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

    /// Import identity provider from uploaded JSON file
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `input`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /{realm}/identity-provider/import-config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_importfrom>
    pub async fn realm_identity_provider_import_config_post(
        &self,
        realm: &str,
        input: &[u8],
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/identity-provider/import-config",
                self.url
            ))
            .form(&json!({
                "input": input,
            }))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `representation`: JSON body
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_create>
    pub async fn realm_identity_provider_instances_post(
        &self,
        realm: &str,
        representation: IdentityProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
                self.url
            ))
            .json(&representation)
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
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getidentityproviders>
    pub async fn realm_identity_provider_instances_get(
        &self,
        realm: &str,
    ) -> Result<Vec<IdentityProviderRepresentation>, KeycloakError> {
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

    /// Get the identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getidentityprovider>
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
    /// - `alias`
    /// - `realm`: realm name (not id!)
    /// - `provider_rep`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_identity_provider_instances_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        provider_rep: IdentityProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .json(&provider_rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_delete>
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
    /// - `alias`
    /// - `realm`: realm name (not id!)
    /// - `format`: Format to use
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances/{alias}/export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_export>
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
    /// - `alias`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmanagementpermissions>
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `realm`: realm name (not id!)
    /// - `ref`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setmanagementpermissionsenabled>
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_put(
        &self,
        realm: &str,
        alias: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions", self.url))
            .json(&ref_)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper types for identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances/{alias}/mapper-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmappertypes>
    pub async fn realm_identity_provider_instances_with_alias_mapper_types_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// Add a mapper to identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `realm`: realm name (not id!)
    /// - `mapper`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addmapper>
    pub async fn realm_identity_provider_instances_with_alias_mappers_post(
        &self,
        realm: &str,
        alias: &str,
        mapper: IdentityProviderMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers",
                self.url
            ))
            .json(&mapper)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers for identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmappers>
    pub async fn realm_identity_provider_instances_with_alias_mappers_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<Vec<IdentityProviderMapperRepresentation>, KeycloakError> {
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

    /// Get mapper by id for the identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmapperbyid>
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
    /// - `alias`
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_put(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
        rep: IdentityProviderMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a mapper for the identity provider
    ///
    /// Parameters:
    ///
    /// - `alias`
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_delete>
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
    /// - `provider_id`: Provider id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /{realm}/identity-provider/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getidentityproviders>
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

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Key`
    ///
    /// `GET /{realm}/keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getkeymetadata>
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

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `reps`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /{realm}/client-scopes/{id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createmapper>
    pub async fn realm_client_scopes_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        reps: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&reps)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /{realm}/client-scopes/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createmapper>
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /{realm}/client-scopes/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmappers>
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
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

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /{realm}/client-scopes/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmapperbyid>
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
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /{realm}/client-scopes/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /{realm}/client-scopes/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_delete>
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
    /// - `id`: id of client scope (not name)
    /// - `protocol`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /{realm}/client-scopes/{id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmappersperprotocol>
    pub async fn realm_client_scopes_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `reps`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /{realm}/clients/{id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createmapper>
    pub async fn realm_clients_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        reps: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/add-models",
                self.url
            ))
            .json(&reps)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /{realm}/clients/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createmapper>
    pub async fn realm_clients_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /{realm}/clients/{id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmappers>
    pub async fn realm_clients_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
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

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /{realm}/clients/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmapperbyid>
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
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /{realm}/clients/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/protocol-mappers/models/{id}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `id`: Mapper id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /{realm}/clients/{id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_delete>
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
    /// - `id`: id of client (not client-id)
    /// - `protocol`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /{realm}/clients/{id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmappersperprotocol>
    pub async fn realm_clients_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
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

    /// Import a realm.
    ///
    /// Imports a realm from a full representation of that realm. Realm name must be unique.
    ///
    /// Parameters:
    ///
    /// - `request_body`: InputStream
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_importrealm>
    pub async fn post(&self, request_body: RealmRepresentation) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/", self.url))
            .json(&request_body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the top-level representation of the realm   It will not include nested information like User and Client representations.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealm>
    pub async fn realm_get(&self, realm: &str) -> Result<RealmRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the top-level information of the realm   Any user, roles or client information in the representation  will be ignored.
    ///
    /// This will only update top-level attributes of the realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updaterealm>
    pub async fn realm_put(
        &self,
        realm: &str,
        rep: RealmRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}", self.url))
            .json(&rep)
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
    /// `DELETE /{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealm>
    pub async fn realm_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get admin events   Returns all admin events, or filters events based on URL query parameters listed here
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
    /// `GET /{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getevents>
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
        operation_types: Option<String>,
        resource_path: Option<String>,
        resource_types: Option<String>,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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
    /// `DELETE /{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_clearadminevents>
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
    /// - `description`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /{realm}/client-description-converter`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_convertclientdescription>
    pub async fn realm_client_description_converter_post(
        &self,
        realm: &str,
        description: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-description-converter",
                self.url
            ))
            .json(&description)
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
    /// `GET /{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getpolicies>
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
    /// - `client_policies`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updatepolicies>
    pub async fn realm_client_policies_policies_put(
        &self,
        realm: &str,
        client_policies: ClientPoliciesRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-policies/policies",
                self.url
            ))
            .json(&client_policies)
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
    /// `GET /{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getprofiles>
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
    /// - `client_profiles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updateprofiles>
    pub async fn realm_client_policies_profiles_put(
        &self,
        realm: &str,
        client_profiles: ClientProfilesRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/client-policies/profiles",
                self.url
            ))
            .json(&client_profiles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client session stats   Returns a JSON map.
    ///
    /// The key is the client id, the value is the number of sessions that currently are active  with that client. Only clients that actually have a session associated with them will be in this map.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/client-session-stats`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientsessionstats>
    pub async fn realm_client_session_stats_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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
    /// `GET /{realm}/credential-registrators`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcredentialregistrators>
    pub async fn realm_credential_registrators_get(
        &self,
        realm: &str,
    ) -> Result<Vec<String>, KeycloakError> {
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

    /// Get realm default client scopes.
    ///
    /// Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/default-default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getdefaultdefaultclientscopes>
    pub async fn realm_default_default_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
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

    /// PUT /{realm}/default-default-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_adddefaultdefaultclientscope>
    ///
    /// REST method: `PUT /{realm}/default-default-client-scopes/{clientScopeId}`
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
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-default-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removedefaultdefaultclientscope>
    ///
    /// REST method: `DELETE /{realm}/default-default-client-scopes/{clientScopeId}`
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

    /// Get group hierarchy.
    ///
    /// Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/default-groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getdefaultgroups>
    pub async fn realm_default_groups_get(
        &self,
        realm: &str,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/default-groups", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-groups/{groupId}
    ///
    /// Parameters:
    ///
    /// - `group_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_adddefaultgroup>
    ///
    /// REST method: `PUT /{realm}/default-groups/{groupId}`
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
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-groups/{groupId}
    ///
    /// Parameters:
    ///
    /// - `group_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removedefaultgroup>
    ///
    /// REST method: `DELETE /{realm}/default-groups/{groupId}`
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

    /// Get realm optional client scopes.
    ///
    /// Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/default-optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getdefaultoptionalclientscopes>
    pub async fn realm_default_optional_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
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

    /// PUT /{realm}/default-optional-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_adddefaultoptionalclientscope>
    ///
    /// REST method: `PUT /{realm}/default-optional-client-scopes/{clientScopeId}`
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
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-optional-client-scopes/{clientScopeId}
    ///
    /// Parameters:
    ///
    /// - `client_scope_id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removedefaultoptionalclientscope>
    ///
    /// REST method: `DELETE /{realm}/default-optional-client-scopes/{clientScopeId}`
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

    /// Get events   Returns all events, or filters them based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client`: App or oauth client name
    /// - `date_from`: From date
    /// - `date_to`: To date
    /// - `first`: Paging offset
    /// - `ip_address`: IP address
    /// - `max`: Maximum results size (defaults to 100)
    /// - `type`: The types of events to return
    /// - `user`: User id
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getevents>
    pub async fn realm_events_get(
        &self,
        realm: &str,
        client: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        first: Option<i32>,
        ip_address: Option<String>,
        max: Option<i32>,
        type_: Option<String>,
        user: Option<String>,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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
    /// `DELETE /{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_clearevents>
    pub async fn realm_events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{realm}/events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the events provider configuration   Returns JSON object with events provider configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmeventsconfig>
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

    /// Update the events provider   Change the events provider and/or its configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updaterealmeventsconfig>
    pub async fn realm_events_config_put(
        &self,
        realm: &str,
        rep: RealmEventsConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/events/config", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `path`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/group-by-path/{path}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroupbypath>
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
    /// `GET /{realm}/localization`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmlocalizationlocales>
    pub async fn realm_localization_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/localization", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `locale`
    /// - `realm`: realm name (not id!)
    /// - `localization_texts`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createorupdaterealmlocalizationtexts>
    pub async fn realm_localization_with_locale_post(
        &self,
        realm: &str,
        locale: &str,
        localization_texts: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .json(&localization_texts)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `locale`
    /// - `realm`: realm name (not id!)
    /// - `use_realm_default_locale_fallback`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmlocalizationtexts>
    pub async fn realm_localization_with_locale_get(
        &self,
        realm: &str,
        locale: &str,
        use_realm_default_locale_fallback: Option<bool>,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// Parameters:
    ///
    /// - `locale`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealmlocalizationtexts>
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
    /// - `key`
    /// - `locale`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmlocalizationtext>
    pub async fn realm_localization_with_locale_with_key_get(
        &self,
        realm: &str,
        locale: &str,
        key: &str,
    ) -> Result<String, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `key`
    /// - `locale`
    /// - `realm`: realm name (not id!)
    /// - `text`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_saverealmlocalizationtext>
    pub async fn realm_localization_with_locale_with_key_put(
        &self,
        realm: &str,
        locale: &str,
        key: &str,
        text: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .json(&text)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Parameters:
    ///
    /// - `key`
    /// - `locale`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealmlocalizationtext>
    pub async fn realm_localization_with_locale_with_key_delete(
        &self,
        realm: &str,
        locale: &str,
        key: &str,
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
    /// Any client that has an admin url will also be told to invalidate any sessions  they have.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /{realm}/logout-all`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_logoutall>
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
    /// `POST /{realm}/partial-export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_partialexport>
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
    /// - `rep`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /{realm}/partialImport`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_partialimport>
    pub async fn realm_partial_import_post(
        &self,
        realm: &str,
        rep: PartialImportRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/partialImport", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Push the realm’s revocation policy to any client that has an admin url associated with it.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /{realm}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_pushrevocation>
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
    /// Any client that has an admin url will also be told to invalidate this  particular session.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `session`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /{realm}/sessions/{session}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deletesession>
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

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `settings`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /{realm}/testSMTPConnection`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_testsmtpconnection>
    pub async fn realm_test_smtp_connection_post(
        &self,
        realm: &str,
        settings: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/testSMTPConnection",
                self.url
            ))
            .json(&settings)
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
    /// `GET /{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getusermgmtpermissions>
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
    /// - `ref`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setusersmanagementpermissionsenabled>
    pub async fn realm_users_management_permissions_put(
        &self,
        realm: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users-management-permissions",
                self.url
            ))
            .json(&ref_)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get role mappings
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrolemappings>
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

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `roles`: Roles to add
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /{realm}/groups/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addrealmrolemappings>
    pub async fn realm_groups_with_id_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmrolemappings>
    pub async fn realm_groups_with_id_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /{realm}/groups/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealmrolemappings>
    pub async fn realm_groups_with_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/groups/{id}/role-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailablerealmrolemappings>
    pub async fn realm_groups_with_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `id`
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/groups/{id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositerealmrolemappings>
    pub async fn realm_groups_with_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/users/{id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrolemappings>
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

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `roles`: Roles to add
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /{realm}/users/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addrealmrolemappings>
    pub async fn realm_users_with_id_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/users/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmrolemappings>
    pub async fn realm_users_with_id_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /{realm}/users/{id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealmrolemappings>
    pub async fn realm_users_with_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/users/{id}/role-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/users/{id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailablerealmrolemappings>
    pub async fn realm_users_with_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /{realm}/users/{id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositerealmrolemappings>
    pub async fn realm_users_with_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Roles`
    ///
    /// `POST /{realm}/clients/{id}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createrole>
    pub async fn realm_clients_with_id_roles_post(
        &self,
        realm: &str,
        id: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get all roles for the realm or client
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getroles>
    pub async fn realm_clients_with_id_roles_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrole>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}`
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    /// - `rep`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /{realm}/clients/{id}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updaterole>
    ///
    /// REST method: `PUT /{realm}/clients/{id}/roles/{role-name}`
    pub async fn realm_clients_with_id_roles_with_role_name_put(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /{realm}/clients/{id}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterole>
    ///
    /// REST method: `DELETE /{realm}/clients/{id}/roles/{role-name}`
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

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    /// - `roles`
    ///
    /// Resource: `Roles`
    ///
    /// `POST /{realm}/clients/{id}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addcomposites>
    ///
    /// REST method: `POST /{realm}/clients/{id}/roles/{role-name}/composites`
    pub async fn realm_clients_with_id_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get composites of the role
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrolecomposites>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}/composites`
    pub async fn realm_clients_with_id_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove roles from the role’s composite
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    /// - `roles`: roles to remove
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /{realm}/clients/{id}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deletecomposites>
    ///
    /// REST method: `DELETE /{realm}/clients/{id}/roles/{role-name}/composites`
    pub async fn realm_clients_with_id_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    ///
    /// Parameters:
    ///
    /// - `client_uuid`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientrolecomposites>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}/composites/clients/{clientUuid}`
    pub async fn realm_clients_with_id_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/composites/clients/{client_uuid}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role’s composite
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmrolecomposites>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}/composites/realm`
    pub async fn realm_clients_with_id_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroupsinrole>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}/groups`
    pub async fn realm_clients_with_id_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmanagementpermissions>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}/management/permissions`
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
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    /// - `ref`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /{realm}/clients/{id}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setmanagementpermissionsenabled>
    ///
    /// REST method: `PUT /{realm}/clients/{id}/roles/{role-name}/management/permissions`
    pub async fn realm_clients_with_id_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/clients/{id}/roles/{role_name}/management/permissions",
                self.url
            ))
            .json(&ref_)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/clients/{id}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getusersinrole>
    ///
    /// REST method: `GET /{realm}/clients/{id}/roles/{role-name}/users`
    pub async fn realm_clients_with_id_roles_with_role_name_users_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserRepresentation>, KeycloakError> {
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

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Roles`
    ///
    /// `POST /{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createrole>
    pub async fn realm_roles_post(
        &self,
        realm: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/roles", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
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
    /// `GET /{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getroles>
    pub async fn realm_roles_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrole>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}`
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
    /// - `role_name`: role’s name (not id!)
    /// - `rep`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updaterole>
    ///
    /// REST method: `PUT /{realm}/roles/{role-name}`
    pub async fn realm_roles_with_role_name_put(
        &self,
        realm: &str,
        role_name: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .json(&rep)
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
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterole>
    ///
    /// REST method: `DELETE /{realm}/roles/{role-name}`
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

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    /// - `roles`
    ///
    /// Resource: `Roles`
    ///
    /// `POST /{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addcomposites>
    ///
    /// REST method: `POST /{realm}/roles/{role-name}/composites`
    pub async fn realm_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&roles)
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
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrolecomposites>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}/composites`
    pub async fn realm_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove roles from the role’s composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    /// - `roles`: roles to remove
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deletecomposites>
    ///
    /// REST method: `DELETE /{realm}/roles/{role-name}/composites`
    pub async fn realm_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    ///
    /// Parameters:
    ///
    /// - `client_uuid`
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientrolecomposites>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}/composites/clients/{clientUuid}`
    pub async fn realm_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        role_name: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get realm-level roles of the role’s composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role’s name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /{realm}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmrolecomposites>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}/composites/realm`
    pub async fn realm_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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
    /// `GET /{realm}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroupsinrole>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}/groups`
    pub async fn realm_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
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
    /// `GET /{realm}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmanagementpermissions>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}/management/permissions`
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
    /// - `ref`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /{realm}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setmanagementpermissionsenabled>
    ///
    /// REST method: `PUT /{realm}/roles/{role-name}/management/permissions`
    pub async fn realm_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        role_name: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/roles/{role_name}/management/permissions",
                self.url
            ))
            .json(&ref_)
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
    /// `GET /{realm}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getusersinrole>
    ///
    /// REST method: `GET /{realm}/roles/{role-name}/users`
    pub async fn realm_roles_with_role_name_users_get(
        &self,
        realm: &str,
        role_name: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserRepresentation>, KeycloakError> {
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

    /// Get a specific role’s representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrole>
    ///
    /// REST method: `GET /{realm}/roles-by-id/{role-id}`
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
    /// - `rep`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updaterole>
    ///
    /// REST method: `PUT /{realm}/roles-by-id/{role-id}`
    pub async fn realm_roles_by_id_with_role_id_put(
        &self,
        realm: &str,
        role_id: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .json(&rep)
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
    /// `DELETE /{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterole>
    ///
    /// REST method: `DELETE /{realm}/roles-by-id/{role-id}`
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

    /// Make the role a composite role by associating some child roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `roles`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `POST /{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addcomposites>
    ///
    /// REST method: `POST /{realm}/roles-by-id/{role-id}/composites`
    pub async fn realm_roles_by_id_with_role_id_composites_post(
        &self,
        realm: &str,
        role_id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get role’s children   Returns a set of role’s children provided the role is a composite.
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
    /// `GET /{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrolecomposites>
    ///
    /// REST method: `GET /{realm}/roles-by-id/{role-id}/composites`
    pub async fn realm_roles_by_id_with_role_id_composites_get(
        &self,
        realm: &str,
        role_id: &str,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove a set of roles from the role’s composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: Role id
    /// - `roles`: A set of roles to be removed
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deletecomposites>
    ///
    /// REST method: `DELETE /{realm}/roles-by-id/{role-id}/composites`
    pub async fn realm_roles_by_id_with_role_id_composites_delete(
        &self,
        realm: &str,
        role_id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    ///
    /// Parameters:
    ///
    /// - `client_uuid`
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /{realm}/roles-by-id/{role_id}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientrolecomposites>
    ///
    /// REST method: `GET /{realm}/roles-by-id/{role-id}/composites/clients/{clientUuid}`
    pub async fn realm_roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        role_id: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get realm-level roles that are in the role’s composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /{realm}/roles-by-id/{role_id}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmrolecomposites>
    ///
    /// REST method: `GET /{realm}/roles-by-id/{role-id}/composites/realm`
    pub async fn realm_roles_by_id_with_role_id_composites_realm_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /{realm}/roles-by-id/{role_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getmanagementpermissions>
    ///
    /// REST method: `GET /{realm}/roles-by-id/{role-id}/management/permissions`
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

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `ref`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /{realm}/roles-by-id/{role_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setmanagementpermissionsenabled>
    ///
    /// REST method: `PUT /{realm}/roles-by-id/{role-id}/management/permissions`
    pub async fn realm_roles_by_id_with_role_id_management_permissions_put(
        &self,
        realm: &str,
        role_id: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/management/permissions",
                self.url
            ))
            .json(&ref_)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client’s scope
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /{realm}/client-scopes/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addclientscopemapping>
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the roles associated with a client’s scope   Returns roles for the client.
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove client-level roles from the client’s scope.
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /{realm}/client-scopes/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteclientscopemapping>
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles   Returns the roles for the client that can be associated with the client’s scope
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailableclientscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}/available", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositeclientscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a set of realm-level roles to the client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /{realm}/client-scopes/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addrealmscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles associated with the client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/client-scopes/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove a set of realm-level roles from the client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /{realm}/client-scopes/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealmscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/client-scopes/{id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailablerealmscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client scope (not name)
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/client-scopes/{id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositerealmscopemappings>
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Add client-level roles to the client’s scope
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /{realm}/clients/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addclientscopemapping>
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the roles associated with a client’s scope   Returns roles for the client.
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/clients/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getclientscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove client-level roles from the client’s scope.
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /{realm}/clients/{id}/scope-mappings/clients/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteclientscopemapping>
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles   Returns the roles for the client that can be associated with the client’s scope
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/clients/{id}/scope-mappings/clients/{client}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailableclientscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    ///
    /// Parameters:
    ///
    /// - `client`
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/clients/{id}/scope-mappings/clients/{client}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositeclientscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Add a set of realm-level roles to the client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `POST /{realm}/clients/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addrealmscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles associated with the client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/clients/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getrealmscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Remove a set of realm-level roles from the client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `roles`
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `DELETE /{realm}/clients/{id}/scope-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleterealmscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{realm}/clients/{id}/scope-mappings/realm",
                self.url
            ))
            .json(&roles)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client’s scope
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/clients/{id}/scope-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getavailablerealmscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    ///
    /// Parameters:
    ///
    /// - `id`: id of client (not client-id)
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Scope Mappings`
    ///
    /// `GET /{realm}/clients/{id}/scope-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getcompositerealmscopemappings>
    pub async fn realm_clients_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
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

    /// Create a new user   Username must be unique.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Users`
    ///
    /// `POST /{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_createuser>
    pub async fn realm_users_post(
        &self,
        realm: &str,
        rep: UserRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/users", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get users   Returns a stream of users, filtered according to query parameters.
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
    /// - `search`: A String contained in username, first or last name, or email
    /// - `username`: A String contained in username, or the complete username, if param "exact" is true
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getusers>
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
    ) -> Result<Vec<UserRepresentation>, KeycloakError> {
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

    /// Returns the number of users that match the given criteria.
    ///
    /// It can be called in three different ways.  1. Don’t specify any criteria and pass {@code null}. The number of all  users within that realm will be returned.  <p>  2. If {@code search} is specified other criteria such as {@code last} will  be ignored even though you set them. The {@code search} string will be  matched against the first and last name, the username and the email of a  user.  <p>  3. If {@code search} is unspecified but any of {@code last}, {@code first},  {@code email} or {@code username} those criteria are matched against their  respective fields on a user entity. Combined with a logical and.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `email`: email filter
    /// - `email_verified`
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `first_name`: first name filter
    /// - `last_name`: last name filter
    /// - `search`: arbitrary search string for all the fields below
    /// - `username`: username filter
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getuserscount>
    pub async fn realm_users_count_get(
        &self,
        realm: &str,
        email: Option<String>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        first_name: Option<String>,
        last_name: Option<String>,
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
    /// `GET /{realm}/users/profile`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getconfiguration>
    pub async fn realm_users_profile_get(&self, realm: &str) -> Result<String, KeycloakError> {
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
    /// - `text`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/profile`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_update>
    pub async fn realm_users_profile_put(
        &self,
        realm: &str,
        text: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/users/profile", self.url))
            .json(&text)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get representation of the user
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getuser>
    pub async fn realm_users_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<UserRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{realm}/users/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the user
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_updateuser>
    pub async fn realm_users_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: UserRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{realm}/users/{id}", self.url))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the user
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /{realm}/users/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_deleteuser>
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
    /// Returned values can contain for example "password", "otp" etc.  This will always return empty list for "local" users, which are not backed by any user storage
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/configured-user-storage-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getconfigureduserstoragecredentialtypes>
    pub async fn realm_users_with_id_configured_user_storage_credential_types_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<String>, KeycloakError> {
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/consents`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getconsents>
    pub async fn realm_users_with_id_consents_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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
    /// - `client`: Client id
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /{realm}/users/{id}/consents/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_revokeconsent>
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/credentials`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_credentials>
    pub async fn realm_users_with_id_credentials_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<CredentialRepresentation>, KeycloakError> {
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
    /// - `credential_id`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /{realm}/users/{id}/credentials/{credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removecredential>
    ///
    /// REST method: `DELETE /{realm}/users/{id}/credentials/{credentialId}`
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
    /// - `credential_id`: The credential to move
    /// - `id`: User id
    /// - `new_previous_credential_id`: The credential that will be the previous element in the list. If set to null, the moved credential will be the first element in the list.
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `POST /{realm}/users/{id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_movecredentialafter>
    ///
    /// REST method: `POST /{realm}/users/{id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}`
    pub async fn realm_users_with_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
        new_previous_credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{realm}/users/{id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a first position in the credentials list of the user
    ///
    /// Parameters:
    ///
    /// - `credential_id`: The credential to move
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `POST /{realm}/users/{id}/credentials/{credential_id}/moveToFirst`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_movecredentialtofirst>
    ///
    /// REST method: `POST /{realm}/users/{id}/credentials/{credentialId}/moveToFirst`
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
    /// - `credential_id`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `user_label`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}/credentials/{credential_id}/userLabel`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_setcredentialuserlabel>
    ///
    /// REST method: `PUT /{realm}/users/{id}/credentials/{credentialId}/userLabel`
    pub async fn realm_users_with_id_credentials_with_credential_id_user_label_put(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
        user_label: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/credentials/{credential_id}/userLabel",
                self.url
            ))
            .json(&user_label)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Disable all credentials for a user of a specific type
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `credential_types`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}/disable-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_disablecredentialtype>
    pub async fn realm_users_with_id_disable_credential_types_put(
        &self,
        realm: &str,
        id: &str,
        credential_types: Vec<String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/disable-credential-types",
                self.url
            ))
            .json(&credential_types)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Send a update account email to the user   An email contains a link the user can click to perform a set of required actions.
    ///
    /// The redirectUri and clientId parameters are optional. If no redirect is given, then there will  be no link back to click after actions have completed. Redirect uri must be a valid uri for the  particular clientId.
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    /// - `actions`: required actions the user needs to complete
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}/execute-actions-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_executeactionsemail>
    pub async fn realm_users_with_id_execute_actions_email_put(
        &self,
        realm: &str,
        id: &str,
        client_id: Option<String>,
        lifespan: Option<i32>,
        redirect_uri: Option<String>,
        actions: Vec<String>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/execute-actions-email",
                self.url
            ))
            .json(&actions)
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/federated-identity`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getfederatedidentity>
    pub async fn realm_users_with_id_federated_identity_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<FederatedIdentityRepresentation>, KeycloakError> {
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
    /// - `id`: User id
    /// - `provider`: Social login provider id
    /// - `realm`: realm name (not id!)
    /// - `rep`
    ///
    /// Resource: `Users`
    ///
    /// `POST /{realm}/users/{id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_addfederatedidentity>
    pub async fn realm_users_with_id_federated_identity_with_provider_post(
        &self,
        realm: &str,
        id: &str,
        provider: &str,
        rep: FederatedIdentityRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{realm}/users/{id}/federated-identity/{provider}",
                self.url
            ))
            .json(&rep)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a social login provider from user
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `provider`: Social login provider id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /{realm}/users/{id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removefederatedidentity>
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_groupmembership>
    pub async fn realm_users_with_id_groups_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getgroupmembershipcount>
    pub async fn realm_users_with_id_groups_count_get(
        &self,
        realm: &str,
        id: &str,
        search: Option<String>,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// PUT /{realm}/users/{id}/groups/{groupId}
    ///
    /// Parameters:
    ///
    /// - `group_id`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_joingroup>
    ///
    /// REST method: `PUT /{realm}/users/{id}/groups/{groupId}`
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
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/users/{id}/groups/{groupId}
    ///
    /// Parameters:
    ///
    /// - `group_id`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /{realm}/users/{id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_removemembership>
    ///
    /// REST method: `DELETE /{realm}/users/{id}/groups/{groupId}`
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `POST /{realm}/users/{id}/impersonation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_impersonate>
    pub async fn realm_users_with_id_impersonation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
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

    /// Remove all user sessions associated with the user   Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `POST /{realm}/users/{id}/logout`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_logout>
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
    /// - `client_uuid`
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/offline-sessions/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getofflinesessions>
    ///
    /// REST method: `GET /{realm}/users/{id}/offline-sessions/{clientUuid}`
    pub async fn realm_users_with_id_offline_sessions_with_client_uuid_get(
        &self,
        realm: &str,
        id: &str,
        client_uuid: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `cred`: The representation must contain a rawPassword with the plain-text password
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}/reset-password`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_resetpassword>
    pub async fn realm_users_with_id_reset_password_put(
        &self,
        realm: &str,
        id: &str,
        cred: CredentialRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{realm}/users/{id}/reset-password",
                self.url
            ))
            .json(&cred)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Send an email-verification email to the user   An email contains a link the user can click to verify their email address.
    ///
    /// The redirectUri and clientId parameters are optional. The default for the  redirect is the account client.
    ///
    /// Parameters:
    ///
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    /// - `client_id`: Client id
    /// - `redirect_uri`: Redirect uri
    ///
    /// Resource: `Users`
    ///
    /// `PUT /{realm}/users/{id}/send-verify-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_sendverifyemail>
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
    /// - `id`: User id
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /{realm}/users/{id}/sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getsessions>
    pub async fn realm_users_with_id_sessions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
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

    /// Get themes, social providers, auth providers, and event listeners available on this server
    ///
    /// Resource: `Root`
    ///
    /// `GET /`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_getinfo>
    pub async fn get(&self) -> Result<ServerInfoRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// CORS preflight
    ///
    /// Resource: `Root`
    ///
    /// `OPTIONS /`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/20.0.1/rest-api/index.html#_preflight>
    ///
    /// REST method: `OPTIONS /{any}`
    pub async fn any_options(&self) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .request(
                reqwest::Method::OPTIONS,
                &format!("{}/admin/realms/", self.url),
            )
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }
}
