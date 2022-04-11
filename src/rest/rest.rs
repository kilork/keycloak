use serde_json::{json, Value};
use std::collections::HashMap;

use super::*;

impl KeycloakAdmin {
    /// Clear any user login failures for all users   This can release temporary disabled users
    /// DELETE /{realm}/attack-detection/brute-force/users
    pub async fn realm_attack_detection_brute_force_users_delete(
        &self,
        realm: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/attack-detection/brute-force/users",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get status of a username in brute force detection
    /// GET /{realm}/attack-detection/brute-force/users/{userId}
    pub async fn realm_attack_detection_brute_force_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/attack-detection/brute-force/users/{}",
                self.url, realm, user_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Clear any user login failures for the user   This can release temporary disabled user
    /// DELETE /{realm}/attack-detection/brute-force/users/{userId}
    pub async fn realm_attack_detection_brute_force_users_with_user_id_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/attack-detection/brute-force/users/{}",
                self.url, realm, user_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authenticator providers   Returns a stream of authenticator providers.
    /// GET /{realm}/authentication/authenticator-providers
    pub async fn realm_authentication_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/authenticator-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client authenticator providers   Returns a stream of client authenticator providers.
    /// GET /{realm}/authentication/client-authenticator-providers
    pub async fn realm_authentication_client_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/client-authenticator-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get authenticator provider’s configuration description
    /// GET /{realm}/authentication/config-description/{providerId}
    pub async fn realm_authentication_config_description_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<AuthenticatorConfigInfoRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/config-description/{}",
                self.url, realm, provider_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get authenticator configuration
    /// GET /{realm}/authentication/config/{id}
    pub async fn realm_authentication_config_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticatorConfigRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/config/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update authenticator configuration
    /// PUT /{realm}/authentication/config/{id}
    pub async fn realm_authentication_config_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/authentication/config/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete authenticator configuration
    /// DELETE /{realm}/authentication/config/{id}
    pub async fn realm_authentication_config_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/authentication/config/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new authentication execution
    /// POST /{realm}/authentication/executions
    pub async fn realm_authentication_executions_post(
        &self,
        realm: &str,
        execution: AuthenticationExecutionRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/executions",
                self.url, realm
            ))
            .json(&execution)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get Single Execution
    /// GET /{realm}/authentication/executions/{executionId}
    pub async fn realm_authentication_executions_with_execution_id_get(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/executions/{}",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete execution
    /// DELETE /{realm}/authentication/executions/{executionId}
    pub async fn realm_authentication_executions_with_execution_id_delete(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/authentication/executions/{}",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update execution with new configuration
    /// POST /{realm}/authentication/executions/{executionId}/config
    pub async fn realm_authentication_executions_with_execution_id_config_post(
        &self,
        realm: &str,
        execution_id: &str,
        json: AuthenticatorConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/executions/{}/config",
                self.url, realm, execution_id
            ))
            .json(&json)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Lower execution’s priority
    /// POST /{realm}/authentication/executions/{executionId}/lower-priority
    pub async fn realm_authentication_executions_with_execution_id_lower_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/executions/{}/lower-priority",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Raise execution’s priority
    /// POST /{realm}/authentication/executions/{executionId}/raise-priority
    pub async fn realm_authentication_executions_with_execution_id_raise_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/executions/{}/raise-priority",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a new authentication flow
    /// POST /{realm}/authentication/flows
    pub async fn realm_authentication_flows_post(
        &self,
        realm: &str,
        flow: AuthenticationFlowRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/flows",
                self.url, realm
            ))
            .json(&flow)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication flows   Returns a stream of authentication flows.
    /// GET /{realm}/authentication/flows
    pub async fn realm_authentication_flows_get(
        &self,
        realm: &str,
    ) -> Result<Vec<AuthenticationFlowRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/flows",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Copy existing authentication flow under a new name   The new name is given as 'newName' attribute of the passed JSON object
    /// POST /{realm}/authentication/flows/{flowAlias}/copy
    pub async fn realm_authentication_flows_with_flow_alias_copy_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/flows/{}/copy",
                self.url, realm, flow_alias
            ))
            .json(&data)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication executions for a flow
    /// GET /{realm}/authentication/flows/{flowAlias}/executions
    pub async fn realm_authentication_flows_with_flow_alias_executions_get(
        &self,
        realm: &str,
        flow_alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/flows/{}/executions",
                self.url, realm, flow_alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update authentication executions of a Flow
    /// PUT /{realm}/authentication/flows/{flowAlias}/executions
    pub async fn realm_authentication_flows_with_flow_alias_executions_put(
        &self,
        realm: &str,
        flow_alias: &str,
        rep: AuthenticationExecutionInfoRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/authentication/flows/{}/executions",
                self.url, realm, flow_alias
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new authentication execution to a flow
    /// POST /{realm}/authentication/flows/{flowAlias}/executions/execution
    pub async fn realm_authentication_flows_with_flow_alias_executions_execution_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/flows/{}/executions/execution",
                self.url, realm, flow_alias
            ))
            .json(&data)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new flow with new execution to existing flow
    /// POST /{realm}/authentication/flows/{flowAlias}/executions/flow
    pub async fn realm_authentication_flows_with_flow_alias_executions_flow_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/flows/{}/executions/flow",
                self.url, realm, flow_alias
            ))
            .json(&data)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication flow for id
    /// GET /{realm}/authentication/flows/{id}
    pub async fn realm_authentication_flows_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticationFlowRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/flows/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update an authentication flow
    /// PUT /{realm}/authentication/flows/{id}
    pub async fn realm_authentication_flows_with_id_put(
        &self,
        realm: &str,
        id: &str,
        flow: AuthenticationFlowRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/authentication/flows/{}",
                self.url, realm, id
            ))
            .json(&flow)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete an authentication flow
    /// DELETE /{realm}/authentication/flows/{id}
    pub async fn realm_authentication_flows_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/authentication/flows/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get form action providers   Returns a stream of form action providers.
    /// GET /{realm}/authentication/form-action-providers
    pub async fn realm_authentication_form_action_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/form-action-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get form providers   Returns a stream of form providers.
    /// GET /{realm}/authentication/form-providers
    pub async fn realm_authentication_form_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/form-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get configuration descriptions for all clients
    /// GET /{realm}/authentication/per-client-config-description
    pub async fn realm_authentication_per_client_config_description_get(
        &self,
        realm: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/per-client-config-description",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a new required actions
    /// POST /{realm}/authentication/register-required-action
    pub async fn realm_authentication_register_required_action_post(
        &self,
        realm: &str,
        data: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/register-required-action",
                self.url, realm
            ))
            .json(&data)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get required actions   Returns a stream of required actions.
    /// GET /{realm}/authentication/required-actions
    pub async fn realm_authentication_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<Vec<RequiredActionProviderRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/required-actions",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get required action for alias
    /// GET /{realm}/authentication/required-actions/{alias}
    pub async fn realm_authentication_required_actions_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<RequiredActionProviderRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/required-actions/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update required action
    /// PUT /{realm}/authentication/required-actions/{alias}
    pub async fn realm_authentication_required_actions_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        rep: RequiredActionProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/authentication/required-actions/{}",
                self.url, realm, alias
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete required action
    /// DELETE /{realm}/authentication/required-actions/{alias}
    pub async fn realm_authentication_required_actions_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/authentication/required-actions/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Lower required action’s priority
    /// POST /{realm}/authentication/required-actions/{alias}/lower-priority
    pub async fn realm_authentication_required_actions_with_alias_lower_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/required-actions/{}/lower-priority",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Raise required action’s priority
    /// POST /{realm}/authentication/required-actions/{alias}/raise-priority
    pub async fn realm_authentication_required_actions_with_alias_raise_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/authentication/required-actions/{}/raise-priority",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get unregistered required actions   Returns a stream of unregistered required actions.
    /// GET /{realm}/authentication/unregistered-required-actions
    pub async fn realm_authentication_unregistered_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/authentication/unregistered-required-actions",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get key info
    /// GET /{realm}/clients/{id}/certificates/{attr}
    pub async fn realm_clients_with_id_certificates_with_attr_get(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/certificates/{}",
                self.url, realm, id, attr
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get a keystore file for the client, containing private key and public certificate
    /// POST /{realm}/clients/{id}/certificates/{attr}/download
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
                "{}/admin/realms/{}/clients/{}/certificates/{}/download",
                self.url, realm, id, attr
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.bytes().await?.to_vec())
    }

    /// Generate a new certificate with new key pair
    /// POST /{realm}/clients/{id}/certificates/{attr}/generate
    pub async fn realm_clients_with_id_certificates_with_attr_generate_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/certificates/{}/generate",
                self.url, realm, id, attr
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new keypair and certificate, and get the private key file   Generates a keypair and certificate and serves the private key in a specified keystore format.
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    /// POST /{realm}/clients/{id}/certificates/{attr}/generate-and-download
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
                "{}/admin/realms/{}/clients/{}/certificates/{}/generate-and-download",
                self.url, realm, id, attr
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.bytes().await?.to_vec())
    }

    /// Upload certificate and eventually private key
    /// POST /{realm}/clients/{id}/certificates/{attr}/upload
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
                "{}/admin/realms/{}/clients/{}/certificates/{}/upload",
                self.url, realm, id, attr
            ))
            .form(&json!({
                "input": input,
            }))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Upload only certificate, not private key
    /// POST /{realm}/clients/{id}/certificates/{attr}/upload-certificate
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
                "{}/admin/realms/{}/clients/{}/certificates/{}/upload-certificate",
                self.url, realm, id, attr
            ))
            .form(&json!({
                "input": input,
            }))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new initial access token.
    /// POST /{realm}/clients-initial-access
    pub async fn realm_clients_initial_access_post(
        &self,
        realm: &str,
        config: ClientInitialAccessCreatePresentation,
    ) -> Result<ClientInitialAccessPresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients-initial-access",
                self.url, realm
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/clients-initial-access
    pub async fn realm_clients_initial_access_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientInitialAccessPresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients-initial-access",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// DELETE /{realm}/clients-initial-access/{id}
    pub async fn realm_clients_initial_access_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients-initial-access/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Base path for retrieve providers with the configProperties properly filled
    /// GET /{realm}/client-registration-policy/providers
    pub async fn realm_client_registration_policy_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-registration-policy/providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user role mapping
    /// POST /{realm}/groups/{id}/role-mappings/clients/{client}
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
                "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level role mappings for the user, and the app
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete client-level roles from user role mapping
    /// DELETE /{realm}/groups/{id}/role-mappings/clients/{client}
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
                "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}/available
    pub async fn realm_groups_with_id_role_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings   This recurses any composite roles
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}/composite
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
                "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}/composite",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user role mapping
    /// POST /{realm}/users/{id}/role-mappings/clients/{client}
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
                "{}/admin/realms/{}/users/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level role mappings for the user, and the app
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}
    pub async fn realm_users_with_id_role_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete client-level roles from user role mapping
    /// DELETE /{realm}/users/{id}/role-mappings/clients/{client}
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
                "{}/admin/realms/{}/users/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}/available
    pub async fn realm_users_with_id_role_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings   This recurses any composite roles
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}/composite
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
                "{}/admin/realms/{}/users/{}/role-mappings/clients/{}/composite",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new client scope   Client Scope’s name must be unique!
    /// POST /{realm}/client-scopes
    pub async fn realm_client_scopes_post(
        &self,
        realm: &str,
        rep: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/client-scopes",
                self.url, realm
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client scopes belonging to the realm   Returns a list of client scopes belonging to the realm
    /// GET /{realm}/client-scopes
    pub async fn realm_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get representation of the client scope
    /// GET /{realm}/client-scopes/{id}
    pub async fn realm_client_scopes_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientScopeRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the client scope
    /// PUT /{realm}/client-scopes/{id}
    pub async fn realm_client_scopes_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ClientScopeRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/client-scopes/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client scope
    /// DELETE /{realm}/client-scopes/{id}
    pub async fn realm_client_scopes_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/client-scopes/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a new client   Client’s client_id must be unique!
    /// POST /{realm}/clients
    pub async fn realm_clients_post(
        &self,
        realm: &str,
        rep: ClientRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{}/clients", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get clients belonging to the realm.
    /// If a client can’t be retrieved from the storage due to a problem with the underlying storage,  it is silently removed from the returned list.  This ensures that concurrent modifications to the list don’t prevent callers from retrieving this list.
    /// GET /{realm}/clients
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
            .get(&format!("{}/admin/realms/{}/clients", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/clients/{id}
    pub async fn realm_clients_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the client
    /// PUT /{realm}/clients/{id}
    pub async fn realm_clients_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ClientRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/clients/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the client
    /// DELETE /{realm}/clients/{id}
    pub async fn realm_clients_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Generate a new secret for the client
    /// POST /{realm}/clients/{id}/client-secret
    pub async fn realm_clients_with_id_client_secret_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/client-secret",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the client secret
    /// GET /{realm}/clients/{id}/client-secret
    pub async fn realm_clients_with_id_client_secret_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/client-secret",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get default client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/clients/{id}/default-client-scopes
    pub async fn realm_clients_with_id_default_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/default-client-scopes",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    pub async fn realm_clients_with_id_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/clients/{}/default-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    pub async fn realm_clients_with_id_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}/default-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create JSON with payload of example access token
    /// GET /{realm}/clients/{id}/evaluate-scopes/generate-example-access-token
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
                "{}/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-access-token",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/clients/{id}/evaluate-scopes/generate-example-id-token
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
                "{}/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-id-token",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/clients/{id}/evaluate-scopes/generate-example-userinfo
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
                "{}/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-userinfo",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// This means  protocol mappers assigned to this client directly and protocol mappers assigned to all client scopes of this client.
    /// GET /{realm}/clients/{id}/evaluate-scopes/protocol-mappers
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
                "{}/admin/realms/{}/clients/{}/evaluate-scopes/protocol-mappers",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective scope mapping of all roles of particular role container, which this client is defacto allowed to have in the accessToken issued for him.
    /// This contains scope mappings, which this client has directly, as well as scope mappings, which are granted to all client scopes,  which are linked with this client.
    /// GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/granted
    pub async fn realm_clients_with_id_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/evaluate-scopes/scope-mappings/{}/granted",
                self.url, realm, id, role_container_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get roles, which this client doesn’t have scope for and can’t have them in the accessToken issued for him.
    /// Defacto all the  other roles of particular role container, which are not in {@link #getGrantedScopeMappings()}
    /// GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/not-granted
    pub async fn realm_clients_with_id_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<String>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/evaluate-scopes/scope-mappings/{}/not-granted",
                self.url, realm, id, role_container_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/clients/{id}/installation/providers/{providerId}
    pub async fn realm_clients_with_id_installation_providers_with_provider_id_get(
        &self,
        realm: &str,
        id: &str,
        provider_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/installation/providers/{}",
                self.url, realm, id, provider_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/clients/{id}/management/permissions
    pub async fn realm_clients_with_id_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/management/permissions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/clients/{id}/management/permissions
    pub async fn realm_clients_with_id_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/clients/{}/management/permissions",
                self.url, realm, id
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a cluster node with the client   Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle  by sending registration request to Keycloak
    /// POST /{realm}/clients/{id}/nodes
    pub async fn realm_clients_with_id_nodes_post(
        &self,
        realm: &str,
        id: &str,
        form_params: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/nodes",
                self.url, realm, id
            ))
            .json(&form_params)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Unregister a cluster node from the client
    /// DELETE /{realm}/clients/{id}/nodes/{node}
    pub async fn realm_clients_with_id_nodes_with_node_delete(
        &self,
        realm: &str,
        id: &str,
        node: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}/nodes/{}",
                self.url, realm, id, node
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get application offline session count   Returns a number of offline user sessions associated with this client   {      "count": number  }
    /// GET /{realm}/clients/{id}/offline-session-count
    pub async fn realm_clients_with_id_offline_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/offline-session-count",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get offline sessions for client   Returns a list of offline user sessions associated with this client
    /// GET /{realm}/clients/{id}/offline-sessions
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
                "{}/admin/realms/{}/clients/{}/offline-sessions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// Only name and ids are returned.
    /// GET /{realm}/clients/{id}/optional-client-scopes
    pub async fn realm_clients_with_id_optional_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/optional-client-scopes",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    pub async fn realm_clients_with_id_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/clients/{}/optional-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    pub async fn realm_clients_with_id_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}/optional-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Push the client’s revocation policy to its admin URL   If the client has an admin URL, push revocation policy to it.
    /// POST /{realm}/clients/{id}/push-revocation
    pub async fn realm_clients_with_id_push_revocation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/push-revocation",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new registration access token for the client
    /// POST /{realm}/clients/{id}/registration-access-token
    pub async fn realm_clients_with_id_registration_access_token_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/registration-access-token",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get a user dedicated to the service account
    /// GET /{realm}/clients/{id}/service-account-user
    pub async fn realm_clients_with_id_service_account_user_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<UserRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/service-account-user",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get application session count   Returns a number of user sessions associated with this client   {      "count": number  }
    /// GET /{realm}/clients/{id}/session-count
    pub async fn realm_clients_with_id_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/session-count",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Test if registered cluster nodes are available   Tests availability by sending 'ping' request to all cluster nodes.
    /// GET /{realm}/clients/{id}/test-nodes-available
    pub async fn realm_clients_with_id_test_nodes_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/test-nodes-available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get user sessions for client   Returns a list of user sessions associated with this client
    /// GET /{realm}/clients/{id}/user-sessions
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
                "{}/admin/realms/{}/clients/{}/user-sessions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// POST /{realm}/components
    pub async fn realm_components_post(
        &self,
        realm: &str,
        rep: ComponentRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/components",
                self.url, realm
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/components
    pub async fn realm_components_get(
        &self,
        realm: &str,
        name: Option<String>,
        parent: Option<String>,
        type_: Option<String>,
    ) -> Result<Vec<ComponentRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/components",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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

    /// GET /{realm}/components/{id}
    pub async fn realm_components_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ComponentRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/components/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/components/{id}
    pub async fn realm_components_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ComponentRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/components/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/components/{id}
    pub async fn realm_components_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/components/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
    /// GET /{realm}/components/{id}/sub-component-types
    pub async fn realm_components_with_id_sub_component_types_get(
        &self,
        realm: &str,
        id: &str,
        type_: Option<String>,
    ) -> Result<Vec<ComponentRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/components/{}/sub-component-types",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// create or add a top level realm groupSet or create child.
    /// This will update the group and set the parent if it exists. Create it and set the parent  if the group doesn’t exist.
    /// POST /{realm}/groups
    pub async fn realm_groups_post(
        &self,
        realm: &str,
        rep: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{}/groups", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get group hierarchy.
    /// Only name and ids are returned.
    /// GET /{realm}/groups
    pub async fn realm_groups_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/admin/realms/{}/groups", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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

    /// Returns the groups counts.
    /// GET /{realm}/groups/count
    pub async fn realm_groups_count_get(
        &self,
        realm: &str,
        search: Option<String>,
        top: Option<bool>,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/count",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = top {
            builder = builder.query(&[("top", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/groups/{id}
    pub async fn realm_groups_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update group, ignores subgroups.
    /// PUT /{realm}/groups/{id}
    pub async fn realm_groups_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/groups/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/groups/{id}
    pub async fn realm_groups_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/groups/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Set or create child.
    /// This will just set the parent if it exists. Create it and set the parent  if the group doesn’t exist.
    /// POST /{realm}/groups/{id}/children
    pub async fn realm_groups_with_id_children_post(
        &self,
        realm: &str,
        id: &str,
        rep: GroupRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/groups/{}/children",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/groups/{id}/management/permissions
    pub async fn realm_groups_with_id_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/management/permissions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/groups/{id}/management/permissions
    pub async fn realm_groups_with_id_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/groups/{}/management/permissions",
                self.url, realm, id
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get users   Returns a stream of users, filtered according to query parameters
    /// GET /{realm}/groups/{id}/members
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
                "{}/admin/realms/{}/groups/{}/members",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// POST /{realm}/identity-provider/import-config
    pub async fn realm_identity_provider_import_config_post(
        &self,
        realm: &str,
        input: &[u8],
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/identity-provider/import-config",
                self.url, realm
            ))
            .form(&json!({
                "input": input,
            }))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new identity provider
    /// POST /{realm}/identity-provider/instances
    pub async fn realm_identity_provider_instances_post(
        &self,
        realm: &str,
        representation: IdentityProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/identity-provider/instances",
                self.url, realm
            ))
            .json(&representation)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get identity providers
    /// GET /{realm}/identity-provider/instances
    pub async fn realm_identity_provider_instances_get(
        &self,
        realm: &str,
    ) -> Result<Vec<IdentityProviderRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the identity provider
    /// GET /{realm}/identity-provider/instances/{alias}
    pub async fn realm_identity_provider_instances_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the identity provider
    /// PUT /{realm}/identity-provider/instances/{alias}
    pub async fn realm_identity_provider_instances_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        provider_rep: IdentityProviderRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}",
                self.url, realm, alias
            ))
            .json(&provider_rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the identity provider
    /// DELETE /{realm}/identity-provider/instances/{alias}
    pub async fn realm_identity_provider_instances_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Export public broker configuration for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/export
    pub async fn realm_identity_provider_instances_with_alias_export_get(
        &self,
        realm: &str,
        alias: &str,
        format: Option<String>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/export",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = format {
            builder = builder.query(&[("format", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/identity-provider/instances/{alias}/management/permissions
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/management/permissions",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/identity-provider/instances/{alias}/management/permissions
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_put(
        &self,
        realm: &str,
        alias: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/management/permissions",
                self.url, realm, alias
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper types for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mapper-types
    pub async fn realm_identity_provider_instances_with_alias_mapper_types_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/mapper-types",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a mapper to identity provider
    /// POST /{realm}/identity-provider/instances/{alias}/mappers
    pub async fn realm_identity_provider_instances_with_alias_mappers_post(
        &self,
        realm: &str,
        alias: &str,
        mapper: IdentityProviderMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/mappers",
                self.url, realm, alias
            ))
            .json(&mapper)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mappers
    pub async fn realm_identity_provider_instances_with_alias_mappers_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<Vec<IdentityProviderMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/mappers",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper by id for the identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_get(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<IdentityProviderMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
                self.url, realm, alias, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a mapper for the identity provider
    /// PUT /{realm}/identity-provider/instances/{alias}/mappers/{id}
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
                "{}/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
                self.url, realm, alias, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a mapper for the identity provider
    /// DELETE /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_delete(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
                self.url, realm, alias, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get identity providers
    /// GET /{realm}/identity-provider/providers/{provider_id}
    pub async fn realm_identity_provider_providers_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/identity-provider/providers/{}",
                self.url, realm, provider_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/keys
    pub async fn realm_keys_get(
        &self,
        realm: &str,
    ) -> Result<KeysMetadataRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{}/keys", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create multiple mappers
    /// POST /{realm}/client-scopes/{id}/protocol-mappers/add-models
    pub async fn realm_client_scopes_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        reps: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/add-models",
                self.url, realm, id
            ))
            .json(&reps)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a mapper
    /// POST /{realm}/client-scopes/{id}/protocol-mappers/models
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/models
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper by id
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    /// PUT /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    /// DELETE /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn realm_client_scopes_with_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/protocol/{protocol}
    pub async fn realm_client_scopes_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/protocol/{}",
                self.url, realm, id, protocol
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create multiple mappers
    /// POST /{realm}/clients/{id}/protocol-mappers/add-models
    pub async fn realm_clients_with_id_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        reps: Vec<ProtocolMapperRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/add-models",
                self.url, realm, id
            ))
            .json(&reps)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a mapper
    /// POST /{realm}/clients/{id}/protocol-mappers/models
    pub async fn realm_clients_with_id_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/models",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers
    /// GET /{realm}/clients/{id}/protocol-mappers/models
    pub async fn realm_clients_with_id_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/models",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper by id
    /// GET /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    /// PUT /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the mapper
    /// DELETE /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn realm_clients_with_id_protocol_mappers_models_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    /// GET /{realm}/clients/{id}/protocol-mappers/protocol/{protocol}
    pub async fn realm_clients_with_id_protocol_mappers_protocol_with_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/protocol-mappers/protocol/{}",
                self.url, realm, id, protocol
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Import a realm   Imports a realm from a full representation of that realm.
    /// Realm name must be unique.
    /// POST /
    pub async fn post(&self, rep: RealmRepresentation) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/", self.url,))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the top-level representation of the realm   It will not include nested information like User and Client representations.
    /// GET /{realm}
    pub async fn realm_get(&self, realm: &str) -> Result<RealmRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{}", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the top-level information of the realm   Any user, roles or client information in the representation  will be ignored.
    /// This will only update top-level attributes of the realm.
    /// PUT /{realm}
    pub async fn realm_put(
        &self,
        realm: &str,
        rep: RealmRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/admin/realms/{}", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the realm
    /// DELETE /{realm}
    pub async fn realm_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{}", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get admin events   Returns all admin events, or filters events based on URL query parameters listed here
    /// GET /{realm}/admin-events
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
            .get(&format!(
                "{}/admin/realms/{}/admin-events",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// DELETE /{realm}/admin-events
    pub async fn realm_admin_events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/admin-events",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Clear cache of external public keys (Public keys of clients or Identity providers)
    /// POST /{realm}/clear-keys-cache
    pub async fn realm_clear_keys_cache_post(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clear-keys-cache",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Clear realm cache
    /// POST /{realm}/clear-realm-cache
    pub async fn realm_clear_realm_cache_post(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clear-realm-cache",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Clear user cache
    /// POST /{realm}/clear-user-cache
    pub async fn realm_clear_user_cache_post(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clear-user-cache",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Base path for importing clients under this realm.
    /// POST /{realm}/client-description-converter
    pub async fn realm_client_description_converter_post(
        &self,
        realm: &str,
        description: &str,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/client-description-converter",
                self.url, realm
            ))
            .json(&description)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/client-policies/policies
    pub async fn realm_client_policies_policies_get(
        &self,
        realm: &str,
    ) -> Result<ClientPoliciesRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-policies/policies",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/client-policies/policies
    pub async fn realm_client_policies_policies_put(
        &self,
        realm: &str,
        client_policies: ClientPoliciesRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/client-policies/policies",
                self.url, realm
            ))
            .json(&client_policies)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/client-policies/profiles
    pub async fn realm_client_policies_profiles_get(
        &self,
        realm: &str,
        include_global_profiles: Option<bool>,
    ) -> Result<ClientProfilesRepresentation, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-policies/profiles",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = include_global_profiles {
            builder = builder.query(&[("include-global-profiles", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/client-policies/profiles
    pub async fn realm_client_policies_profiles_put(
        &self,
        realm: &str,
        client_profiles: ClientProfilesRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/client-policies/profiles",
                self.url, realm
            ))
            .json(&client_profiles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client session stats   Returns a JSON map.
    /// The key is the client id, the value is the number of sessions that currently are active  with that client. Only clients that actually have a session associated with them will be in this map.
    /// GET /{realm}/client-session-stats
    pub async fn realm_client_session_stats_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-session-stats",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/credential-registrators
    pub async fn realm_credential_registrators_get(
        &self,
        realm: &str,
    ) -> Result<Vec<String>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/credential-registrators",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm default client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/default-default-client-scopes
    pub async fn realm_default_default_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/default-default-client-scopes",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-default-client-scopes/{clientScopeId}
    pub async fn realm_default_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/default-default-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-default-client-scopes/{clientScopeId}
    pub async fn realm_default_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/default-default-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get group hierarchy.
    /// Only name and ids are returned.
    /// GET /{realm}/default-groups
    pub async fn realm_default_groups_get(
        &self,
        realm: &str,
    ) -> Result<Vec<GroupRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/default-groups",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-groups/{groupId}
    pub async fn realm_default_groups_with_group_id_put(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/default-groups/{}",
                self.url, realm, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-groups/{groupId}
    pub async fn realm_default_groups_with_group_id_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/default-groups/{}",
                self.url, realm, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm optional client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/default-optional-client-scopes
    pub async fn realm_default_optional_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/default-optional-client-scopes",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-optional-client-scopes/{clientScopeId}
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/default-optional-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-optional-client-scopes/{clientScopeId}
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/default-optional-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get events   Returns all events, or filters them based on URL query parameters listed here
    /// GET /{realm}/events
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
            .get(&format!("{}/admin/realms/{}/events", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// DELETE /{realm}/events
    pub async fn realm_events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/admin/realms/{}/events", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the events provider configuration   Returns JSON object with events provider configuration
    /// GET /{realm}/events/config
    pub async fn realm_events_config_get(
        &self,
        realm: &str,
    ) -> Result<RealmEventsConfigRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/events/config",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the events provider   Change the events provider and/or its configuration
    /// PUT /{realm}/events/config
    pub async fn realm_events_config_put(
        &self,
        realm: &str,
        rep: RealmEventsConfigRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/events/config",
                self.url, realm
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/group-by-path/{path}
    pub async fn realm_group_by_path_with_path_get(
        &self,
        realm: &str,
        path: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/group-by-path/{}",
                self.url, realm, path
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get LDAP supported extensions.
    /// POST /{realm}/ldap-server-capabilities
    pub async fn realm_ldap_server_capabilities_post(
        &self,
        realm: &str,
        config: TestLdapConnectionRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/ldap-server-capabilities",
                self.url, realm
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/localization
    pub async fn realm_localization_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/localization",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// POST /{realm}/localization/{locale}
    pub async fn realm_localization_with_locale_post(
        &self,
        realm: &str,
        locale: &str,
        localization_texts: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/localization/{}",
                self.url, realm, locale
            ))
            .json(&localization_texts)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/localization/{locale}
    pub async fn realm_localization_with_locale_get(
        &self,
        realm: &str,
        locale: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/localization/{}",
                self.url, realm, locale
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// DELETE /{realm}/localization/{locale}
    pub async fn realm_localization_with_locale_delete(
        &self,
        realm: &str,
        locale: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/localization/{}",
                self.url, realm, locale
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/localization/{locale}/{key}
    pub async fn realm_localization_with_locale_with_key_get(
        &self,
        realm: &str,
        locale: &str,
        key: &str,
    ) -> Result<String, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/localization/{}/{}",
                self.url, realm, locale, key
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/localization/{locale}/{key}
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
                "{}/admin/realms/{}/localization/{}/{}",
                self.url, realm, locale, key
            ))
            .json(&text)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/localization/{locale}/{key}
    pub async fn realm_localization_with_locale_with_key_delete(
        &self,
        realm: &str,
        locale: &str,
        key: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/localization/{}/{}",
                self.url, realm, locale, key
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Removes all user sessions.
    /// Any client that has an admin url will also be told to invalidate any sessions  they have.
    /// POST /{realm}/logout-all
    pub async fn realm_logout_all_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/logout-all",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Partial export of existing realm into a JSON file.
    /// POST /{realm}/partial-export
    pub async fn realm_partial_export_post(
        &self,
        realm: &str,
        export_clients: Option<bool>,
        export_groups_and_roles: Option<bool>,
    ) -> Result<RealmRepresentation, KeycloakError> {
        let mut builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/partial-export",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// POST /{realm}/partialImport
    pub async fn realm_partial_import_post(
        &self,
        realm: &str,
        rep: PartialImportRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/partialImport",
                self.url, realm
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Push the realm’s revocation policy to any client that has an admin url associated with it.
    /// POST /{realm}/push-revocation
    pub async fn realm_push_revocation_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/push-revocation",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a specific user session.
    /// Any client that has an admin url will also be told to invalidate this  particular session.
    /// DELETE /{realm}/sessions/{session}
    pub async fn realm_sessions_with_session_delete(
        &self,
        realm: &str,
        session: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/sessions/{}",
                self.url, realm, session
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Test LDAP connection
    /// POST /{realm}/testLDAPConnection
    pub async fn realm_test_ldap_connection_post(
        &self,
        realm: &str,
        config: TestLdapConnectionRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/testLDAPConnection",
                self.url, realm
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// POST /{realm}/testSMTPConnection
    pub async fn realm_test_smtp_connection_post(
        &self,
        realm: &str,
        settings: HashMap<String, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/testSMTPConnection",
                self.url, realm
            ))
            .json(&settings)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/users-management-permissions
    pub async fn realm_users_management_permissions_get(
        &self,
        realm: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users-management-permissions",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/users-management-permissions
    pub async fn realm_users_management_permissions_put(
        &self,
        realm: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/users-management-permissions",
                self.url, realm
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get role mappings
    /// GET /{realm}/groups/{id}/role-mappings
    pub async fn realm_groups_with_id_role_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    /// POST /{realm}/groups/{id}/role-mappings/realm
    pub async fn realm_groups_with_id_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level role mappings
    /// GET /{realm}/groups/{id}/role-mappings/realm
    pub async fn realm_groups_with_id_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete realm-level role mappings
    /// DELETE /{realm}/groups/{id}/role-mappings/realm
    pub async fn realm_groups_with_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    /// GET /{realm}/groups/{id}/role-mappings/realm/available
    pub async fn realm_groups_with_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    /// GET /{realm}/groups/{id}/role-mappings/realm/composite
    pub async fn realm_groups_with_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/groups/{}/role-mappings/realm/composite",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get role mappings
    /// GET /{realm}/users/{id}/role-mappings
    pub async fn realm_users_with_id_role_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    /// POST /{realm}/users/{id}/role-mappings/realm
    pub async fn realm_users_with_id_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level role mappings
    /// GET /{realm}/users/{id}/role-mappings/realm
    pub async fn realm_users_with_id_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete realm-level role mappings
    /// DELETE /{realm}/users/{id}/role-mappings/realm
    pub async fn realm_users_with_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    /// GET /{realm}/users/{id}/role-mappings/realm/available
    pub async fn realm_users_with_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    /// GET /{realm}/users/{id}/role-mappings/realm/composite
    pub async fn realm_users_with_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/role-mappings/realm/composite",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new role for the realm or client
    /// POST /{realm}/clients/{id}/roles
    pub async fn realm_clients_with_id_roles_post(
        &self,
        realm: &str,
        id: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/roles",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get all roles for the realm or client
    /// GET /{realm}/clients/{id}/roles
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
                "{}/admin/realms/{}/clients/{}/roles",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/clients/{id}/roles/{role-name}
    pub async fn realm_clients_with_id_roles_with_role_name_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/roles/{}",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    /// PUT /{realm}/clients/{id}/roles/{role-name}
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
                "{}/admin/realms/{}/clients/{}/roles/{}",
                self.url, realm, id, role_name
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a role by name
    /// DELETE /{realm}/clients/{id}/roles/{role-name}
    pub async fn realm_clients_with_id_roles_with_role_name_delete(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}/roles/{}",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add a composite to the role
    /// POST /{realm}/clients/{id}/roles/{role-name}/composites
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
                "{}/admin/realms/{}/clients/{}/roles/{}/composites",
                self.url, realm, id, role_name
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get composites of the role
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites
    pub async fn realm_clients_with_id_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/roles/{}/composites",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove roles from the role’s composite
    /// DELETE /{realm}/clients/{id}/roles/{role-name}/composites
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
                "{}/admin/realms/{}/clients/{}/roles/{}/composites",
                self.url, realm, id, role_name
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites/clients/{clientUuid}
    pub async fn realm_clients_with_id_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/roles/{}/composites/clients/{}",
                self.url, realm, id, role_name, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role’s composite
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites/realm
    pub async fn realm_clients_with_id_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/roles/{}/composites/realm",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of groups that have the specified role name
    /// GET /{realm}/clients/{id}/roles/{role-name}/groups
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
                "{}/admin/realms/{}/clients/{}/roles/{}/groups",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/clients/{id}/roles/{role-name}/management/permissions
    pub async fn realm_clients_with_id_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/roles/{}/management/permissions",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/clients/{id}/roles/{role-name}/management/permissions
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
                "{}/admin/realms/{}/clients/{}/roles/{}/management/permissions",
                self.url, realm, id, role_name
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    /// GET /{realm}/clients/{id}/roles/{role-name}/users
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
                "{}/admin/realms/{}/clients/{}/roles/{}/users",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// POST /{realm}/roles
    pub async fn realm_roles_post(
        &self,
        realm: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{}/roles", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get all roles for the realm or client
    /// GET /{realm}/roles
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
            .get(&format!("{}/admin/realms/{}/roles", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/roles/{role-name}
    pub async fn realm_roles_with_role_name_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles/{}",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    /// PUT /{realm}/roles/{role-name}
    pub async fn realm_roles_with_role_name_put(
        &self,
        realm: &str,
        role_name: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/roles/{}",
                self.url, realm, role_name
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete a role by name
    /// DELETE /{realm}/roles/{role-name}
    pub async fn realm_roles_with_role_name_delete(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/roles/{}",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add a composite to the role
    /// POST /{realm}/roles/{role-name}/composites
    pub async fn realm_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/roles/{}/composites",
                self.url, realm, role_name
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get composites of the role
    /// GET /{realm}/roles/{role-name}/composites
    pub async fn realm_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles/{}/composites",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove roles from the role’s composite
    /// DELETE /{realm}/roles/{role-name}/composites
    pub async fn realm_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/roles/{}/composites",
                self.url, realm, role_name
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    /// GET /{realm}/roles/{role-name}/composites/clients/{clientUuid}
    pub async fn realm_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        role_name: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles/{}/composites/clients/{}",
                self.url, realm, role_name, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role’s composite
    /// GET /{realm}/roles/{role-name}/composites/realm
    pub async fn realm_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles/{}/composites/realm",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of groups that have the specified role name
    /// GET /{realm}/roles/{role-name}/groups
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
                "{}/admin/realms/{}/roles/{}/groups",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/roles/{role-name}/management/permissions
    pub async fn realm_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles/{}/management/permissions",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/roles/{role-name}/management/permissions
    pub async fn realm_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        role_name: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/roles/{}/management/permissions",
                self.url, realm, role_name
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    /// GET /{realm}/roles/{role-name}/users
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
                "{}/admin/realms/{}/roles/{}/users",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/roles-by-id/{role-id}
    pub async fn realm_roles_by_id_with_role_id_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles-by-id/{}",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the role
    /// PUT /{realm}/roles-by-id/{role-id}
    pub async fn realm_roles_by_id_with_role_id_put(
        &self,
        realm: &str,
        role_id: &str,
        rep: RoleRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/roles-by-id/{}",
                self.url, realm, role_id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the role
    /// DELETE /{realm}/roles-by-id/{role-id}
    pub async fn realm_roles_by_id_with_role_id_delete(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/roles-by-id/{}",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Make the role a composite role by associating some child roles
    /// POST /{realm}/roles-by-id/{role-id}/composites
    pub async fn realm_roles_by_id_with_role_id_composites_post(
        &self,
        realm: &str,
        role_id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/roles-by-id/{}/composites",
                self.url, realm, role_id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get role’s children   Returns a set of role’s children provided the role is a composite.
    /// GET /{realm}/roles-by-id/{role-id}/composites
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
                "{}/admin/realms/{}/roles-by-id/{}/composites",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// DELETE /{realm}/roles-by-id/{role-id}/composites
    pub async fn realm_roles_by_id_with_role_id_composites_delete(
        &self,
        realm: &str,
        role_id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/roles-by-id/{}/composites",
                self.url, realm, role_id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    /// GET /{realm}/roles-by-id/{role-id}/composites/clients/{clientUuid}
    pub async fn realm_roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        role_id: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles-by-id/{}/composites/clients/{}",
                self.url, realm, role_id, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles that are in the role’s composite
    /// GET /{realm}/roles-by-id/{role-id}/composites/realm
    pub async fn realm_roles_by_id_with_role_id_composites_realm_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles-by-id/{}/composites/realm",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/roles-by-id/{role-id}/management/permissions
    pub async fn realm_roles_by_id_with_role_id_management_permissions_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/roles-by-id/{}/management/permissions",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/roles-by-id/{role-id}/management/permissions
    pub async fn realm_roles_by_id_with_role_id_management_permissions_put(
        &self,
        realm: &str,
        role_id: &str,
        ref_: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/roles-by-id/{}/management/permissions",
                self.url, realm, role_id
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client’s scope
    /// POST /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
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
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the roles associated with a client’s scope   Returns roles for the client.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove client-level roles from the client’s scope.
    /// DELETE /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
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
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles   Returns the roles for the client that can be associated with the client’s scope
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/available
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite
    pub async fn realm_client_scopes_with_id_scope_mappings_clients_with_client_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}/composite",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a set of realm-level roles to the client’s scope
    /// POST /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles associated with the client’s scope
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a set of realm-level roles from the client’s scope
    /// DELETE /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client’s scope
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm/available
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm/composite
    pub async fn realm_client_scopes_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm/composite",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client’s scope
    /// POST /{realm}/clients/{id}/scope-mappings/clients/{client}
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
                "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the roles associated with a client’s scope   Returns roles for the client.
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove client-level roles from the client’s scope.
    /// DELETE /{realm}/clients/{id}/scope-mappings/clients/{client}
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
                "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// The available client-level roles   Returns the roles for the client that can be associated with the client’s scope
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}/available
    pub async fn realm_clients_with_id_scope_mappings_clients_with_client_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}/composite
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
                "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}/composite",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a set of realm-level roles to the client’s scope
    /// POST /{realm}/clients/{id}/scope-mappings/realm
    pub async fn realm_clients_with_id_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles associated with the client’s scope
    /// GET /{realm}/clients/{id}/scope-mappings/realm
    pub async fn realm_clients_with_id_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a set of realm-level roles from the client’s scope
    /// DELETE /{realm}/clients/{id}/scope-mappings/realm
    pub async fn realm_clients_with_id_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .json(&roles)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client’s scope
    /// GET /{realm}/clients/{id}/scope-mappings/realm/available
    pub async fn realm_clients_with_id_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    /// GET /{realm}/clients/{id}/scope-mappings/realm/composite
    pub async fn realm_clients_with_id_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/clients/{}/scope-mappings/realm/composite",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Need this for admin console to display simple name of provider when displaying client detail   KEYCLOAK-4328
    /// GET /{id}/name
    pub async fn with_id_name_get(
        &self,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/{}/name", self.url, id))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Need this for admin console to display simple name of provider when displaying user detail   KEYCLOAK-4328
    /// GET /{realm}/user-storage/{id}/name
    pub async fn realm_user_storage_with_id_name_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/user-storage/{}/name",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove imported users
    /// POST /{realm}/user-storage/{id}/remove-imported-users
    pub async fn realm_user_storage_with_id_remove_imported_users_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/user-storage/{}/remove-imported-users",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Trigger sync of users   Action can be "triggerFullSync" or "triggerChangedUsersSync"
    /// POST /{realm}/user-storage/{id}/sync
    pub async fn realm_user_storage_with_id_sync_post(
        &self,
        realm: &str,
        id: &str,
        action: Option<String>,
    ) -> Result<SynchronizationResult, KeycloakError> {
        let mut builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/user-storage/{}/sync",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = action {
            builder = builder.query(&[("action", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Unlink imported users from a storage provider
    /// POST /{realm}/user-storage/{id}/unlink-users
    pub async fn realm_user_storage_with_id_unlink_users_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/user-storage/{}/unlink-users",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Trigger sync of mapper data related to ldap mapper (roles, groups, …​)   direction is "fedToKeycloak" or "keycloakToFed"
    /// POST /{realm}/user-storage/{parentId}/mappers/{id}/sync
    pub async fn realm_user_storage_with_parent_id_mappers_with_id_sync_post(
        &self,
        realm: &str,
        parent_id: &str,
        id: &str,
        direction: Option<String>,
    ) -> Result<SynchronizationResult, KeycloakError> {
        let mut builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/user-storage/{}/mappers/{}/sync",
                self.url, realm, parent_id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = direction {
            builder = builder.query(&[("direction", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new user   Username must be unique.
    /// POST /{realm}/users
    pub async fn realm_users_post(
        &self,
        realm: &str,
        rep: UserRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/admin/realms/{}/users", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get users   Returns a stream of users, filtered according to query parameters.
    /// GET /{realm}/users
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
            .get(&format!("{}/admin/realms/{}/users", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// It can be called in three different ways.  1. Don’t specify any criteria and pass {@code null}. The number of all  users within that realm will be returned.  <p>  2. If {@code search} is specified other criteria such as {@code last} will  be ignored even though you set them. The {@code search} string will be  matched against the first and last name, the username and the email of a  user.  <p>  3. If {@code search} is unspecified but any of {@code last}, {@code first},  {@code email} or {@code username} those criteria are matched against their  respective fields on a user entity. Combined with a logical and.
    /// GET /{realm}/users/count
    pub async fn realm_users_count_get(
        &self,
        realm: &str,
        email: Option<String>,
        email_verified: Option<bool>,
        first_name: Option<String>,
        last_name: Option<String>,
        search: Option<String>,
        username: Option<String>,
    ) -> Result<i32, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/count",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
        }
        if let Some(v) = email_verified {
            builder = builder.query(&[("emailVerified", v)]);
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

    /// GET /{realm}/users/profile
    pub async fn realm_users_profile_get(&self, realm: &str) -> Result<String, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/profile",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/users/profile
    pub async fn realm_users_profile_put(
        &self,
        realm: &str,
        text: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/users/profile",
                self.url, realm
            ))
            .json(&text)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get representation of the user
    /// GET /{realm}/users/{id}
    pub async fn realm_users_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<UserRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the user
    /// PUT /{realm}/users/{id}
    pub async fn realm_users_with_id_put(
        &self,
        realm: &str,
        id: &str,
        rep: UserRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/users/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the user
    /// DELETE /{realm}/users/{id}
    pub async fn realm_users_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/users/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    /// Returned values can contain for example "password", "otp" etc.  This will always return empty list for "local" users, which are not backed by any user storage
    /// GET /{realm}/users/{id}/configured-user-storage-credential-types
    pub async fn realm_users_with_id_configured_user_storage_credential_types_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<String>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/configured-user-storage-credential-types",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get consents granted by the user
    /// GET /{realm}/users/{id}/consents
    pub async fn realm_users_with_id_consents_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/consents",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Revoke consent and offline tokens for particular client from user
    /// DELETE /{realm}/users/{id}/consents/{client}
    pub async fn realm_users_with_id_consents_with_client_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/users/{}/consents/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/users/{id}/credentials
    pub async fn realm_users_with_id_credentials_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<CredentialRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/credentials",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a credential for a user
    /// DELETE /{realm}/users/{id}/credentials/{credentialId}
    pub async fn realm_users_with_id_credentials_with_credential_id_delete(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/users/{}/credentials/{}",
                self.url, realm, id, credential_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a position behind another credential
    /// POST /{realm}/users/{id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}
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
                "{}/admin/realms/{}/users/{}/credentials/{}/moveAfter/{}",
                self.url, realm, id, credential_id, new_previous_credential_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a first position in the credentials list of the user
    /// POST /{realm}/users/{id}/credentials/{credentialId}/moveToFirst
    pub async fn realm_users_with_id_credentials_with_credential_id_move_to_first_post(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/users/{}/credentials/{}/moveToFirst",
                self.url, realm, id, credential_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update a credential label for a user
    /// PUT /{realm}/users/{id}/credentials/{credentialId}/userLabel
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
                "{}/admin/realms/{}/users/{}/credentials/{}/userLabel",
                self.url, realm, id, credential_id
            ))
            .json(&user_label)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Disable all credentials for a user of a specific type
    /// PUT /{realm}/users/{id}/disable-credential-types
    pub async fn realm_users_with_id_disable_credential_types_put(
        &self,
        realm: &str,
        id: &str,
        credential_types: Vec<String>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/users/{}/disable-credential-types",
                self.url, realm, id
            ))
            .json(&credential_types)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Send a update account email to the user   An email contains a link the user can click to perform a set of required actions.
    /// The redirectUri and clientId parameters are optional. If no redirect is given, then there will  be no link back to click after actions have completed. Redirect uri must be a valid uri for the  particular clientId.
    /// PUT /{realm}/users/{id}/execute-actions-email
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
                "{}/admin/realms/{}/users/{}/execute-actions-email",
                self.url, realm, id
            ))
            .json(&actions)
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/users/{id}/federated-identity
    pub async fn realm_users_with_id_federated_identity_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<FederatedIdentityRepresentation>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/federated-identity",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a social login provider to the user
    /// POST /{realm}/users/{id}/federated-identity/{provider}
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
                "{}/admin/realms/{}/users/{}/federated-identity/{}",
                self.url, realm, id, provider
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Remove a social login provider from user
    /// DELETE /{realm}/users/{id}/federated-identity/{provider}
    pub async fn realm_users_with_id_federated_identity_with_provider_delete(
        &self,
        realm: &str,
        id: &str,
        provider: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/users/{}/federated-identity/{}",
                self.url, realm, id, provider
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/users/{id}/groups
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
                "{}/admin/realms/{}/users/{}/groups",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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

    /// GET /{realm}/users/{id}/groups/count
    pub async fn realm_users_with_id_groups_count_get(
        &self,
        realm: &str,
        id: &str,
        search: Option<String>,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/groups/count",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/users/{id}/groups/{groupId}
    pub async fn realm_users_with_id_groups_with_group_id_put(
        &self,
        realm: &str,
        id: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/users/{}/groups/{}",
                self.url, realm, id, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/users/{id}/groups/{groupId}
    pub async fn realm_users_with_id_groups_with_group_id_delete(
        &self,
        realm: &str,
        id: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/admin/realms/{}/users/{}/groups/{}",
                self.url, realm, id, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Impersonate the user
    /// POST /{realm}/users/{id}/impersonation
    pub async fn realm_users_with_id_impersonation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<String, Value>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/users/{}/impersonation",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove all user sessions associated with the user   Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    /// POST /{realm}/users/{id}/logout
    pub async fn realm_users_with_id_logout_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/admin/realms/{}/users/{}/logout",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get offline sessions associated with the user and client
    /// GET /{realm}/users/{id}/offline-sessions/{clientUuid}
    pub async fn realm_users_with_id_offline_sessions_with_client_uuid_get(
        &self,
        realm: &str,
        id: &str,
        client_uuid: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/offline-sessions/{}",
                self.url, realm, id, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Set up a new password for the user.
    /// PUT /{realm}/users/{id}/reset-password
    pub async fn realm_users_with_id_reset_password_put(
        &self,
        realm: &str,
        id: &str,
        cred: CredentialRepresentation,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/admin/realms/{}/users/{}/reset-password",
                self.url, realm, id
            ))
            .json(&cred)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Send an email-verification email to the user   An email contains a link the user can click to verify their email address.
    /// The redirectUri and clientId parameters are optional. The default for the  redirect is the account client.
    /// PUT /{realm}/users/{id}/send-verify-email
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
                "{}/admin/realms/{}/users/{}/send-verify-email",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
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
    /// GET /{realm}/users/{id}/sessions
    pub async fn realm_users_with_id_sessions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<HashMap<String, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/admin/realms/{}/users/{}/sessions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get themes, social providers, auth providers, and event listeners available on this server
    /// GET /
    pub async fn get(&self) -> Result<ServerInfoRepresentation, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/admin/realms/", self.url,))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// CORS preflight
    /// OPTIONS /{any}
    pub async fn any_options(&self) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .request(
                reqwest::Method::OPTIONS,
                &format!("{}/admin/realms/", self.url,),
            )
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }
}
