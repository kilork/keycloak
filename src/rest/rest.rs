use serde_json::{json, Value};
use std::{borrow::Cow, collections::HashMap};

use super::*;

impl<'a> KeycloakAdmin<'a> {
    /// Clear any user login failures for all users   This can release temporary disabled users
    /// DELETE /{realm}/attack-detection/brute-force/users
    pub async fn attack_detection_brute_force_users_delete(
        &self,
        realm: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/attack-detection/brute-force/users",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get status of a username in brute force detection
    /// GET /{realm}/attack-detection/brute-force/users/{userId}
    pub async fn attack_detection_brute_force_users_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/attack-detection/brute-force/users/{}",
                self.url, realm, user_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Clear any user login failures for the user   This can release temporary disabled user
    /// DELETE /{realm}/attack-detection/brute-force/users/{userId}
    pub async fn attack_detection_brute_force_user_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/attack-detection/brute-force/users/{}",
                self.url, realm, user_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authenticator providers   Returns a list of authenticator providers.
    /// GET /{realm}/authentication/authenticator-providers
    pub async fn authentication_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/authenticator-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client authenticator providers   Returns a list of client authenticator providers.
    /// GET /{realm}/authentication/client-authenticator-providers
    pub async fn authentication_client_authenticator_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/client-authenticator-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get authenticator provider’s configuration description
    /// GET /{realm}/authentication/config-description/{providerId}
    pub async fn authentication_config_description_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<AuthenticatorConfigInfoRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/config-description/{}",
                self.url, realm, provider_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get authenticator configuration
    /// GET /{realm}/authentication/config/{id}
    pub async fn authentication_config_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticatorConfigRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/config/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update authenticator configuration
    /// PUT /{realm}/authentication/config/{id}
    pub async fn authentication_config_put(
        &self,
        realm: &str,
        id: &str,
        rep: AuthenticatorConfigRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/authentication/config/{}",
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
    pub async fn authentication_config_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/authentication/config/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add new authentication execution
    /// POST /{realm}/authentication/executions
    pub async fn authentication_executions_post(
        &self,
        realm: &str,
        execution: AuthenticationExecutionRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/executions",
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
    pub async fn authentication_executions_get(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/executions/{}",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete execution
    /// DELETE /{realm}/authentication/executions/{executionId}
    pub async fn authentication_executions_delete(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/authentication/executions/{}",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update execution with new configuration
    /// POST /{realm}/authentication/executions/{executionId}/config
    pub async fn authentication_executions_config_post(
        &self,
        realm: &str,
        execution_id: &str,
        json: AuthenticatorConfigRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/executions/{}/config",
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
    pub async fn authentication_executions_lower_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/executions/{}/lower-priority",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Raise execution’s priority
    /// POST /{realm}/authentication/executions/{executionId}/raise-priority
    pub async fn authentication_executions_raise_priority_post(
        &self,
        realm: &str,
        execution_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/executions/{}/raise-priority",
                self.url, realm, execution_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a new authentication flow
    /// POST /{realm}/authentication/flows
    pub async fn authentication_flows_post(
        &self,
        realm: &str,
        flow: AuthenticationFlowRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/flows",
                self.url, realm
            ))
            .json(&flow)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get authentication flows   Returns a list of authentication flows.
    /// GET /{realm}/authentication/flows
    pub async fn authentication_flows_get(
        &self,
        realm: &str,
    ) -> Result<Vec<AuthenticationFlowRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/flows",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Copy existing authentication flow under a new name   The new name is given as 'newName' attribute of the passed JSON object
    /// POST /{realm}/authentication/flows/{flowAlias}/copy
    pub async fn authentication_flows_copy_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<&str, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}/copy",
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
    pub async fn authentication_flows_executions_get(
        &self,
        realm: &str,
        flow_alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}/executions",
                self.url, realm, flow_alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update authentication executions of a Flow
    /// PUT /{realm}/authentication/flows/{flowAlias}/executions
    pub async fn authentication_flows_executions_put(
        &self,
        realm: &str,
        flow_alias: &str,
        rep: AuthenticationExecutionInfoRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}/executions",
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
    pub async fn authentication_flows_executions_execution_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<&str, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}/executions/execution",
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
    pub async fn authentication_flows_executions_flow_post(
        &self,
        realm: &str,
        flow_alias: &str,
        data: HashMap<&str, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}/executions/flow",
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
    pub async fn authentication_flow_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<AuthenticationFlowRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update an authentication flow
    /// PUT /{realm}/authentication/flows/{id}
    pub async fn authentication_flows_put(
        &self,
        realm: &str,
        id: &str,
        flow: AuthenticationFlowRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}",
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
    pub async fn authentication_flows_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/authentication/flows/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get form action providers   Returns a list of form action providers.
    /// GET /{realm}/authentication/form-action-providers
    pub async fn authentication_form_action_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/form-action-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get form providers   Returns a list of form providers.
    /// GET /{realm}/authentication/form-providers
    pub async fn authentication_form_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/form-providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get configuration descriptions for all clients
    /// GET /{realm}/authentication/per-client-config-description
    pub async fn authentication_per_client_config_description_get(
        &self,
        realm: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/per-client-config-description",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a new required actions
    /// POST /{realm}/authentication/register-required-action
    pub async fn authentication_register_required_action_post(
        &self,
        realm: &str,
        data: HashMap<&str, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/register-required-action",
                self.url, realm
            ))
            .json(&data)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get required actions   Returns a list of required actions.
    /// GET /{realm}/authentication/required-actions
    pub async fn authentication_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<Vec<RequiredActionProviderRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/required-actions",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get required action for alias
    /// GET /{realm}/authentication/required-actions/{alias}
    pub async fn authentication_required_action_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<RequiredActionProviderRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/required-actions/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update required action
    /// PUT /{realm}/authentication/required-actions/{alias}
    pub async fn authentication_required_actions_put(
        &self,
        realm: &str,
        alias: &str,
        rep: RequiredActionProviderRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/authentication/required-actions/{}",
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
    pub async fn authentication_required_actions_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/authentication/required-actions/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Lower required action’s priority
    /// POST /{realm}/authentication/required-actions/{alias}/lower-priority
    pub async fn authentication_required_actions_lower_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/required-actions/{}/lower-priority",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Raise required action’s priority
    /// POST /{realm}/authentication/required-actions/{alias}/raise-priority
    pub async fn authentication_required_actions_raise_priority_post(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/authentication/required-actions/{}/raise-priority",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get unregistered required actions   Returns a list of unregistered required actions.
    /// GET /{realm}/authentication/unregistered-required-actions
    pub async fn authentication_unregistered_required_actions_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/authentication/unregistered-required-actions",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get key info
    /// GET /{realm}/clients/{id}/certificates/{attr}
    pub async fn clients_certificates_get(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/certificates/{}",
                self.url, realm, id, attr
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get a keystore file for the client, containing private key and public certificate
    /// POST /{realm}/clients/{id}/certificates/{attr}/download
    pub async fn clients_certificates_download_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        config: KeyStoreConfig<'_>,
    ) -> Result<Vec<u8>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/certificates/{}/download",
                self.url, realm, id, attr
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.bytes().await?.to_vec())
    }

    /// Generate a new certificate with new key pair
    /// POST /{realm}/clients/{id}/certificates/{attr}/generate
    pub async fn clients_certificates_generate_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/certificates/{}/generate",
                self.url, realm, id, attr
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new keypair and certificate, and get the private key file   Generates a keypair and certificate and serves the private key in a specified keystore format.
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    /// POST /{realm}/clients/{id}/certificates/{attr}/generate-and-download
    pub async fn clients_certificates_generate_and_download_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        config: KeyStoreConfig<'_>,
    ) -> Result<Vec<u8>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/certificates/{}/generate-and-download",
                self.url, realm, id, attr
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.bytes().await?.to_vec())
    }

    /// Upload certificate and eventually private key
    /// POST /{realm}/clients/{id}/certificates/{attr}/upload
    pub async fn clients_certificates_upload_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        input: &[u8],
    ) -> Result<CertificateRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/certificates/{}/upload",
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
    pub async fn clients_certificates_upload_certificate_post(
        &self,
        realm: &str,
        id: &str,
        attr: &str,
        input: &[u8],
    ) -> Result<CertificateRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/certificates/{}/upload-certificate",
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
    pub async fn clients_initial_access_post(
        &self,
        realm: &str,
        config: ClientInitialAccessCreatePresentation,
    ) -> Result<ClientInitialAccessPresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients-initial-access",
                self.url, realm
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/clients-initial-access
    pub async fn clients_initial_access_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientInitialAccessPresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients-initial-access",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// DELETE /{realm}/clients-initial-access/{id}
    pub async fn clients_initial_access_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients-initial-access/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Base path for retrieve providers with the configProperties properly filled
    /// GET /{realm}/client-registration-policy/providers
    pub async fn client_registration_policy_providers_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ComponentTypeRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-registration-policy/providers",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user role mapping
    /// POST /{realm}/groups/{id}/role-mappings/clients/{client}
    pub async fn groups_role_mappings_clients_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/clients/{}",
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
    pub async fn groups_role_mappings_clients_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete client-level roles from user role mapping
    /// DELETE /{realm}/groups/{id}/role-mappings/clients/{client}
    pub async fn groups_role_mappings_clients_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/clients/{}",
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
    pub async fn groups_role_mappings_clients_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings   This recurses any composite roles
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}/composite
    pub async fn groups_role_mappings_clients_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/clients/{}/composite",
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
    pub async fn users_role_mappings_clients_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/clients/{}",
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
    pub async fn users_role_mappings_clients_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete client-level roles from user role mapping
    /// DELETE /{realm}/users/{id}/role-mappings/clients/{client}
    pub async fn users_role_mappings_clients_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/clients/{}",
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
    pub async fn users_role_mappings_clients_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings   This recurses any composite roles
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}/composite
    pub async fn users_role_mappings_clients_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/clients/{}/composite",
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
    pub async fn client_scopes_post(
        &self,
        realm: &str,
        rep: ClientScopeRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/client-scopes",
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
    pub async fn client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get representation of the client scope
    /// GET /{realm}/client-scopes/{id}
    pub async fn client_scope_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientScopeRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the client scope
    /// PUT /{realm}/client-scopes/{id}
    pub async fn client_scopes_put(
        &self,
        realm: &str,
        id: &str,
        rep: ClientScopeRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}",
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
    pub async fn client_scopes_delete(&self, realm: &str, id: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create a new client   Client’s client_id must be unique!
    /// POST /{realm}/clients
    pub async fn clients_post(
        &self,
        realm: &str,
        rep: ClientRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/auth/admin/realms/{}/clients", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get clients belonging to the realm   Returns a list of clients belonging to the realm
    /// GET /{realm}/clients
    pub async fn clients_get(
        &self,
        realm: &str,
        client_id: Option<&str>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<bool>,
        viewable_only: Option<bool>,
    ) -> Result<Vec<ClientRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/clients", self.url, realm))
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
    pub async fn client_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the client
    /// PUT /{realm}/clients/{id}
    pub async fn clients_put(
        &self,
        realm: &str,
        id: &str,
        rep: ClientRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}",
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
    pub async fn clients_delete(&self, realm: &str, id: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Generate a new secret for the client
    /// POST /{realm}/clients/{id}/client-secret
    pub async fn clients_client_secret_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/client-secret",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the client secret
    /// GET /{realm}/clients/{id}/client-secret
    pub async fn clients_client_secret_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<CredentialRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/client-secret",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get default client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/clients/{id}/default-client-scopes
    pub async fn clients_default_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ClientScopeRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/default-client-scopes",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    pub async fn clients_default_client_scopes_put(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}/default-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    pub async fn clients_default_client_scopes_delete(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/default-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Create JSON with payload of example access token
    /// GET /{realm}/clients/{id}/evaluate-scopes/generate-example-access-token
    pub async fn clients_evaluate_scopes_generate_example_access_token_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<AccessToken<'_>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-access-token",
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
    pub async fn clients_evaluate_scopes_protocol_mappers_get(
        &self,
        realm: &str,
        id: &str,
        scope: Option<&str>,
    ) -> Result<
        Vec<ClientScopeEvaluateResourceProtocolMapperEvaluationRepresentation<'_>>,
        KeycloakError,
    > {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/evaluate-scopes/protocol-mappers",
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
    pub async fn clients_evaluate_scopes_scope_mappings_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<&str>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/evaluate-scopes/scope-mappings/{}/granted",
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
    pub async fn clients_evaluate_scopes_scope_mappings_not_granted_get(
        &self,
        realm: &str,
        id: &str,
        role_container_id: &str,
        scope: Option<&str>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/evaluate-scopes/scope-mappings/{}/not-granted",
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
    pub async fn clients_installation_providers_get(
        &self,
        realm: &str,
        id: &str,
        provider_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/installation/providers/{}",
                self.url, realm, id, provider_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/clients/{id}/management/permissions
    pub async fn clients_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/management/permissions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/clients/{id}/management/permissions
    pub async fn clients_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}/management/permissions",
                self.url, realm, id
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Register a cluster node with the client   Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle  by sending registration request to Keycloak
    /// POST /{realm}/clients/{id}/nodes
    pub async fn clients_nodes_post(
        &self,
        realm: &str,
        id: &str,
        form_params: HashMap<&str, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/nodes",
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
    pub async fn clients_nodes_delete(
        &self,
        realm: &str,
        id: &str,
        node: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/nodes/{}",
                self.url, realm, id, node
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get application offline session count   Returns a number of offline user sessions associated with this client   {      "count": number  }
    /// GET /{realm}/clients/{id}/offline-session-count
    pub async fn clients_offline_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/offline-session-count",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get offline sessions for client   Returns a list of offline user sessions associated with this client
    /// GET /{realm}/clients/{id}/offline-sessions
    pub async fn clients_offline_sessions_get(
        &self,
        realm: &str,
        id: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserSessionRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/offline-sessions",
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
    pub async fn clients_optional_client_scopes_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ClientScopeRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/optional-client-scopes",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    pub async fn clients_optional_client_scopes_put(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}/optional-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    pub async fn clients_optional_client_scopes_delete(
        &self,
        realm: &str,
        id: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/optional-client-scopes/{}",
                self.url, realm, id, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Push the client’s revocation policy to its admin URL   If the client has an admin URL, push revocation policy to it.
    /// POST /{realm}/clients/{id}/push-revocation
    pub async fn clients_push_revocation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GlobalRequestResult<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/push-revocation",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Generate a new registration access token for the client
    /// POST /{realm}/clients/{id}/registration-access-token
    pub async fn clients_registration_access_token_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ClientRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/registration-access-token",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get a user dedicated to the service account
    /// GET /{realm}/clients/{id}/service-account-user
    pub async fn clients_service_account_user_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<UserRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/service-account-user",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get application session count   Returns a number of user sessions associated with this client   {      "count": number  }
    /// GET /{realm}/clients/{id}/session-count
    pub async fn clients_session_count_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/session-count",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Test if registered cluster nodes are available   Tests availability by sending 'ping' request to all cluster nodes.
    /// GET /{realm}/clients/{id}/test-nodes-available
    pub async fn clients_test_nodes_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GlobalRequestResult<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/test-nodes-available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get user sessions for client   Returns a list of user sessions associated with this client
    /// GET /{realm}/clients/{id}/user-sessions
    pub async fn clients_user_sessions_get(
        &self,
        realm: &str,
        id: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserSessionRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/user-sessions",
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
    pub async fn components_post(
        &self,
        realm: &str,
        rep: ComponentRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/components",
                self.url, realm
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/components
    pub async fn components_get(
        &self,
        realm: &str,
        name: Option<&str>,
        parent: Option<&str>,
        type_: Option<&str>,
    ) -> Result<Vec<ComponentRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/components",
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
    pub async fn component_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ComponentRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/components/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/components/{id}
    pub async fn components_put(
        &self,
        realm: &str,
        id: &str,
        rep: ComponentRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/components/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/components/{id}
    pub async fn components_delete(&self, realm: &str, id: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/components/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
    /// GET /{realm}/components/{id}/sub-component-types
    pub async fn components_sub_component_types_get(
        &self,
        realm: &str,
        id: &str,
        type_: Option<&str>,
    ) -> Result<Vec<ComponentTypeRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/components/{}/sub-component-types",
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
    pub async fn groups_post(
        &self,
        realm: &str,
        rep: GroupRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/auth/admin/realms/{}/groups", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get group hierarchy.
    /// Only name and ids are returned.
    /// GET /{realm}/groups
    pub async fn groups_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&str>,
    ) -> Result<Vec<GroupRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/groups", self.url, realm))
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
    pub async fn groups_count_get(
        &self,
        realm: &str,
        search: Option<&str>,
        top: Option<bool>,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/count",
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
    pub async fn group_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<GroupRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update group, ignores subgroups.
    /// PUT /{realm}/groups/{id}
    pub async fn groups_put(
        &self,
        realm: &str,
        id: &str,
        rep: GroupRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/groups/{}",
                self.url, realm, id
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/groups/{id}
    pub async fn groups_delete(&self, realm: &str, id: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/groups/{}",
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
    pub async fn groups_children_post(
        &self,
        realm: &str,
        id: &str,
        rep: GroupRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/groups/{}/children",
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
    pub async fn groups_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/management/permissions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/groups/{id}/management/permissions
    pub async fn groups_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/groups/{}/management/permissions",
                self.url, realm, id
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get users   Returns a list of users, filtered according to query parameters
    /// GET /{realm}/groups/{id}/members
    pub async fn groups_members_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/members",
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
    pub async fn identity_provider_import_config_post(
        &self,
        realm: &str,
        input: &[u8],
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/identity-provider/import-config",
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
    pub async fn identity_provider_instances_post(
        &self,
        realm: &str,
        representation: IdentityProviderRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances",
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
    pub async fn identity_provider_instances_get(
        &self,
        realm: &str,
    ) -> Result<Vec<IdentityProviderRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the identity provider
    /// GET /{realm}/identity-provider/instances/{alias}
    pub async fn identity_provider_instance_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the identity provider
    /// PUT /{realm}/identity-provider/instances/{alias}
    pub async fn identity_provider_instances_put(
        &self,
        realm: &str,
        alias: &str,
        provider_rep: IdentityProviderRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}",
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
    pub async fn identity_provider_instances_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Export public broker configuration for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/export
    pub async fn identity_provider_instances_export_get(
        &self,
        realm: &str,
        alias: &str,
        format: Option<&str>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/export",
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
    pub async fn identity_provider_instances_management_permissions_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/management/permissions",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/identity-provider/instances/{alias}/management/permissions
    pub async fn identity_provider_instances_management_permissions_put(
        &self,
        realm: &str,
        alias: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/management/permissions",
                self.url, realm, alias
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper types for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mapper-types
    pub async fn identity_provider_instances_mapper_types_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/mapper-types",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a mapper to identity provider
    /// POST /{realm}/identity-provider/instances/{alias}/mappers
    pub async fn identity_provider_instances_mappers_post(
        &self,
        realm: &str,
        alias: &str,
        mapper: IdentityProviderMapperRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/mappers",
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
    pub async fn identity_provider_instances_mappers_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<Vec<IdentityProviderMapperRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/mappers",
                self.url, realm, alias
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper by id for the identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn identity_provider_instances_mapper_get(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<IdentityProviderMapperRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
                self.url, realm, alias, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a mapper for the identity provider
    /// PUT /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn identity_provider_instances_mappers_put(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
        rep: IdentityProviderMapperRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
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
    pub async fn identity_provider_instances_mappers_delete(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
                self.url, realm, alias, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get identity providers
    /// GET /{realm}/identity-provider/providers/{provider_id}
    pub async fn identity_provider_providers_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/identity-provider/providers/{}",
                self.url, realm, provider_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/keys
    pub async fn keys_get(
        &self,
        realm: &str,
    ) -> Result<KeysMetadataRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/keys", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create multiple mappers
    /// POST /{realm}/client-scopes/{id}/protocol-mappers/add-models
    pub async fn client_scopes_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        reps: Vec<ProtocolMapperRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/add-models",
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
    pub async fn client_scopes_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
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
    pub async fn client_scopes_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper by id
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn client_scopes_protocol_mappers_model_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    /// PUT /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn client_scopes_protocol_mappers_models_put(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
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
    pub async fn client_scopes_protocol_mappers_models_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/protocol/{protocol}
    pub async fn client_scopes_protocol_mappers_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/protocol-mappers/protocol/{}",
                self.url, realm, id, protocol
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create multiple mappers
    /// POST /{realm}/clients/{id}/protocol-mappers/add-models
    pub async fn clients_protocol_mappers_add_models_post(
        &self,
        realm: &str,
        id: &str,
        reps: Vec<ProtocolMapperRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/add-models",
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
    pub async fn clients_protocol_mappers_models_post(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/models",
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
    pub async fn clients_protocol_mappers_models_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/models",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper by id
    /// GET /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn clients_protocol_mappers_model_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ProtocolMapperRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the mapper
    /// PUT /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn clients_protocol_mappers_models_put(
        &self,
        realm: &str,
        id: &str,
        rep: ProtocolMapperRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
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
    pub async fn clients_protocol_mappers_models_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
                self.url, realm, id, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    /// GET /{realm}/clients/{id}/protocol-mappers/protocol/{protocol}
    pub async fn clients_protocol_mappers_protocol_get(
        &self,
        realm: &str,
        id: &str,
        protocol: &str,
    ) -> Result<Vec<ProtocolMapperRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/protocol-mappers/protocol/{}",
                self.url, realm, id, protocol
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Import a realm   Imports a realm from a full representation of that realm.
    /// Realm name must be unique.
    /// POST /
    pub async fn post(&self, rep: RealmRepresentation<'_>) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/auth/admin/realms/", self.url,))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the top-level representation of the realm   It will not include nested information like User and Client representations.
    /// GET /{realm}
    pub async fn get(&self, realm: &str) -> Result<RealmRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the top-level information of the realm   Any user, roles or client information in the representation  will be ignored.
    /// This will only update top-level attributes of the realm.
    /// PUT /{realm}
    pub async fn put(
        &self,
        realm: &str,
        rep: RealmRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!("{}/auth/admin/realms/{}", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Delete the realm
    /// DELETE /{realm}
    pub async fn delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/auth/admin/realms/{}", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get admin events   Returns all admin events, or filters events based on URL query parameters listed here
    /// GET /{realm}/admin-events
    pub async fn admin_events_get(
        &self,
        realm: &str,
        auth_client: Option<&str>,
        auth_ip_address: Option<&str>,
        auth_realm: Option<&str>,
        auth_user: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        first: Option<i32>,
        max: Option<i32>,
        operation_types: Option<&str>,
        resource_path: Option<&str>,
        resource_types: Option<&str>,
    ) -> Result<Vec<AdminEventRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/admin-events",
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
    pub async fn admin_events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/admin-events",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Clear cache of external public keys (Public keys of clients or Identity providers)
    /// POST /{realm}/clear-keys-cache
    pub async fn clear_keys_cache_post(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clear-keys-cache",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Clear realm cache
    /// POST /{realm}/clear-realm-cache
    pub async fn clear_realm_cache_post(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clear-realm-cache",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Clear user cache
    /// POST /{realm}/clear-user-cache
    pub async fn clear_user_cache_post(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clear-user-cache",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Base path for importing clients under this realm.
    /// POST /{realm}/client-description-converter
    pub async fn client_description_converter_post(
        &self,
        realm: &str,
        description: &str,
    ) -> Result<ClientRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/client-description-converter",
                self.url, realm
            ))
            .json(&description)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client session stats   Returns a JSON map.
    /// The key is the client id, the value is the number of sessions that currently are active  with that client. Only clients that actually have a session associated with them will be in this map.
    /// GET /{realm}/client-session-stats
    pub async fn client_session_stats_get(
        &self,
        realm: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-session-stats",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// GET /{realm}/credential-registrators
    pub async fn credential_registrators_get(
        &self,
        realm: &str,
    ) -> Result<Vec<Cow<'_, str>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/credential-registrators",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm default client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/default-default-client-scopes
    pub async fn default_default_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/default-default-client-scopes",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-default-client-scopes/{clientScopeId}
    pub async fn default_default_client_scopes_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/default-default-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-default-client-scopes/{clientScopeId}
    pub async fn default_default_client_scopes_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/default-default-client-scopes/{}",
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
    pub async fn default_groups_get(
        &self,
        realm: &str,
    ) -> Result<Vec<GroupRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/default-groups",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-groups/{groupId}
    pub async fn default_groups_put(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/default-groups/{}",
                self.url, realm, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-groups/{groupId}
    pub async fn default_groups_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/default-groups/{}",
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
    pub async fn default_optional_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<Vec<ClientScopeRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/default-optional-client-scopes",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/default-optional-client-scopes/{clientScopeId}
    pub async fn default_optional_client_scopes_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/default-optional-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/default-optional-client-scopes/{clientScopeId}
    pub async fn default_optional_client_scopes_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/default-optional-client-scopes/{}",
                self.url, realm, client_scope_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get events   Returns all events, or filters them based on URL query parameters listed here
    /// GET /{realm}/events
    pub async fn events_get(
        &self,
        realm: &str,
        client: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        first: Option<i32>,
        ip_address: Option<&str>,
        max: Option<i32>,
        type_: Option<&str>,
        user: Option<&str>,
    ) -> Result<Vec<EventRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/events", self.url, realm))
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
    pub async fn events_delete(&self, realm: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!("{}/auth/admin/realms/{}/events", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get the events provider configuration   Returns JSON object with events provider configuration
    /// GET /{realm}/events/config
    pub async fn events_config_get(
        &self,
        realm: &str,
    ) -> Result<RealmEventsConfigRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/events/config",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the events provider   Change the events provider and/or its configuration
    /// PUT /{realm}/events/config
    pub async fn events_config_put(
        &self,
        realm: &str,
        rep: RealmEventsConfigRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/events/config",
                self.url, realm
            ))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/group-by-path/{path}
    pub async fn group_by_path_get(
        &self,
        realm: &str,
        path: &str,
    ) -> Result<GroupRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/group-by-path/{}",
                self.url, realm, path
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get LDAP supported extensions.
    /// POST /{realm}/ldap-server-capabilities
    pub async fn ldap_server_capabilities_post(
        &self,
        realm: &str,
        config: TestLdapConnectionRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/ldap-server-capabilities",
                self.url, realm
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Removes all user sessions.
    /// Any client that has an admin url will also be told to invalidate any sessions  they have.
    /// POST /{realm}/logout-all
    pub async fn logout_all_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/logout-all",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Partial export of existing realm into a JSON file.
    /// POST /{realm}/partial-export
    pub async fn partial_export_post(
        &self,
        realm: &str,
        export_clients: Option<bool>,
        export_groups_and_roles: Option<bool>,
    ) -> Result<RealmRepresentation<'_>, KeycloakError> {
        let mut builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/partial-export",
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
    pub async fn partial_import_post(
        &self,
        realm: &str,
        rep: PartialImportRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/partialImport",
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
    pub async fn push_revocation_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult<'_>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/push-revocation",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a specific user session.
    /// Any client that has an admin url will also be told to invalidate this  particular session.
    /// DELETE /{realm}/sessions/{session}
    pub async fn sessions_delete(&self, realm: &str, session: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/sessions/{}",
                self.url, realm, session
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Test LDAP connection
    /// POST /{realm}/testLDAPConnection
    pub async fn test_ldap_connection_post(
        &self,
        realm: &str,
        config: TestLdapConnectionRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/testLDAPConnection",
                self.url, realm
            ))
            .json(&config)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// POST /{realm}/testSMTPConnection
    pub async fn test_smtp_connection_post(
        &self,
        realm: &str,
        settings: HashMap<&str, Value>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/testSMTPConnection",
                self.url, realm
            ))
            .json(&settings)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/users-management-permissions
    pub async fn users_management_permissions_get(
        &self,
        realm: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users-management-permissions",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// PUT /{realm}/users-management-permissions
    pub async fn users_management_permissions_put(
        &self,
        realm: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users-management-permissions",
                self.url, realm
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get role mappings
    /// GET /{realm}/groups/{id}/role-mappings
    pub async fn groups_role_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    /// POST /{realm}/groups/{id}/role-mappings/realm
    pub async fn groups_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/realm",
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
    pub async fn groups_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete realm-level role mappings
    /// DELETE /{realm}/groups/{id}/role-mappings/realm
    pub async fn groups_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/realm",
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
    pub async fn groups_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    /// GET /{realm}/groups/{id}/role-mappings/realm/composite
    pub async fn groups_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/groups/{}/role-mappings/realm/composite",
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
    pub async fn users_role_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    /// POST /{realm}/users/{id}/role-mappings/realm
    pub async fn users_role_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/realm",
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
    pub async fn users_role_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete realm-level role mappings
    /// DELETE /{realm}/users/{id}/role-mappings/realm
    pub async fn users_role_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/realm",
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
    pub async fn users_role_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    /// GET /{realm}/users/{id}/role-mappings/realm/composite
    pub async fn users_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/role-mappings/realm/composite",
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
    pub async fn clients_roles_post(
        &self,
        realm: &str,
        id: &str,
        rep: RoleRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles",
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
    pub async fn clients_roles_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&str>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles",
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
    pub async fn clients_role_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    /// PUT /{realm}/clients/{id}/roles/{role-name}
    pub async fn clients_roles_put(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        rep: RoleRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}",
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
    pub async fn clients_roles_delete(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add a composite to the role
    /// POST /{realm}/clients/{id}/roles/{role-name}/composites
    pub async fn clients_roles_composites_post(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/composites",
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
    pub async fn clients_roles_composites_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/composites",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove roles from the role’s composite
    /// DELETE /{realm}/clients/{id}/roles/{role-name}/composites
    pub async fn clients_roles_composites_delete(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/composites",
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
    pub async fn clients_roles_composites_clients_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/composites/clients/{}",
                self.url, realm, id, role_name, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role’s composite
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites/realm
    pub async fn clients_roles_composites_realm_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/composites/realm",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return List of Groups that have the specified role name
    /// GET /{realm}/clients/{id}/roles/{role-name}/groups
    pub async fn clients_roles_groups_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<GroupRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/groups",
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

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/clients/{id}/roles/{role-name}/management/permissions
    pub async fn clients_roles_management_permissions_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/management/permissions",
                self.url, realm, id, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/clients/{id}/roles/{role-name}/management/permissions
    pub async fn clients_roles_management_permissions_put(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/management/permissions",
                self.url, realm, id, role_name
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return List of Users that have the specified role name
    /// GET /{realm}/clients/{id}/roles/{role-name}/users
    pub async fn clients_roles_users_get(
        &self,
        realm: &str,
        id: &str,
        role_name: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/roles/{}/users",
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
    pub async fn roles_post(
        &self,
        realm: &str,
        rep: RoleRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/auth/admin/realms/{}/roles", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get all roles for the realm or client
    /// GET /{realm}/roles
    pub async fn roles_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&str>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/roles", self.url, realm))
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
    pub async fn role_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    /// PUT /{realm}/roles/{role-name}
    pub async fn roles_put(
        &self,
        realm: &str,
        role_name: &str,
        rep: RoleRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/roles/{}",
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
    pub async fn roles_delete(&self, realm: &str, role_name: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/roles/{}",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Add a composite to the role
    /// POST /{realm}/roles/{role-name}/composites
    pub async fn roles_composites_post(
        &self,
        realm: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/roles/{}/composites",
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
    pub async fn roles_composites_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}/composites",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove roles from the role’s composite
    /// DELETE /{realm}/roles/{role-name}/composites
    pub async fn roles_composites_delete(
        &self,
        realm: &str,
        role_name: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/roles/{}/composites",
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
    pub async fn roles_composites_clients_get(
        &self,
        realm: &str,
        role_name: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}/composites/clients/{}",
                self.url, realm, role_name, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role’s composite
    /// GET /{realm}/roles/{role-name}/composites/realm
    pub async fn roles_composites_realm_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}/composites/realm",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return List of Groups that have the specified role name
    /// GET /{realm}/roles/{role-name}/groups
    pub async fn roles_groups_get(
        &self,
        realm: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<GroupRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}/groups",
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

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/roles/{role-name}/management/permissions
    pub async fn roles_management_permissions_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}/management/permissions",
                self.url, realm, role_name
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/roles/{role-name}/management/permissions
    pub async fn roles_management_permissions_put(
        &self,
        realm: &str,
        role_name: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/roles/{}/management/permissions",
                self.url, realm, role_name
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return List of Users that have the specified role name
    /// GET /{realm}/roles/{role-name}/users
    pub async fn roles_users_get(
        &self,
        realm: &str,
        role_name: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<UserRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles/{}/users",
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
    pub async fn roles_by_id_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<RoleRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the role
    /// PUT /{realm}/roles-by-id/{role-id}
    pub async fn roles_by_id_put(
        &self,
        realm: &str,
        role_id: &str,
        rep: RoleRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}",
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
    pub async fn roles_by_id_delete(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Make the role a composite role by associating some child roles
    /// POST /{realm}/roles-by-id/{role-id}/composites
    pub async fn roles_by_id_composites_post(
        &self,
        realm: &str,
        role_id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/composites",
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
    pub async fn roles_by_id_composites_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/composites",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a set of roles from the role’s composite
    /// DELETE /{realm}/roles-by-id/{role-id}/composites
    pub async fn roles_by_id_composites_delete(
        &self,
        realm: &str,
        role_id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/composites",
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
    pub async fn roles_by_id_composites_clients_get(
        &self,
        realm: &str,
        role_id: &str,
        client_uuid: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/composites/clients/{}",
                self.url, realm, role_id, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles that are in the role’s composite
    /// GET /{realm}/roles-by-id/{role-id}/composites/realm
    pub async fn roles_by_id_composites_realm_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/composites/realm",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/roles-by-id/{role-id}/management/permissions
    pub async fn roles_by_id_management_permissions_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/management/permissions",
                self.url, realm, role_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/roles-by-id/{role-id}/management/permissions
    pub async fn roles_by_id_management_permissions_put(
        &self,
        realm: &str,
        role_id: &str,
        ref_: ManagementPermissionReference<'_>,
    ) -> Result<ManagementPermissionReference<'_>, KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/roles-by-id/{}/management/permissions",
                self.url, realm, role_id
            ))
            .json(&ref_)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get all scope mappings for the client
    /// GET /{realm}/client-scopes/{id}/scope-mappings
    pub async fn client_scopes_scope_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client’s scope
    /// POST /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
    pub async fn client_scopes_scope_mappings_clients_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
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
    pub async fn client_scopes_scope_mappings_clients_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove client-level roles from the client’s scope.
    /// DELETE /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
    pub async fn client_scopes_scope_mappings_clients_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
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
    pub async fn client_scopes_scope_mappings_clients_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite
    pub async fn client_scopes_scope_mappings_clients_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}/composite",
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
    pub async fn client_scopes_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
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
    pub async fn client_scopes_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a set of realm-level roles from the client’s scope
    /// DELETE /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn client_scopes_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
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
    pub async fn client_scopes_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm/composite
    pub async fn client_scopes_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/client-scopes/{}/scope-mappings/realm/composite",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get all scope mappings for the client
    /// GET /{realm}/clients/{id}/scope-mappings
    pub async fn clients_scope_mappings_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<MappingsRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the client’s scope
    /// POST /{realm}/clients/{id}/scope-mappings/clients/{client}
    pub async fn clients_scope_mappings_clients_post(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
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
    pub async fn clients_scope_mappings_clients_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove client-level roles from the client’s scope.
    /// DELETE /{realm}/clients/{id}/scope-mappings/clients/{client}
    pub async fn clients_scope_mappings_clients_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
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
    pub async fn clients_scope_mappings_clients_available_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/clients/{}/available",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}/composite
    pub async fn clients_scope_mappings_clients_composite_get(
        &self,
        realm: &str,
        id: &str,
        client: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/clients/{}/composite",
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
    pub async fn clients_scope_mappings_realm_post(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/realm",
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
    pub async fn clients_scope_mappings_realm_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/realm",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a set of realm-level roles from the client’s scope
    /// DELETE /{realm}/clients/{id}/scope-mappings/realm
    pub async fn clients_scope_mappings_realm_delete(
        &self,
        realm: &str,
        id: &str,
        roles: Vec<RoleRepresentation<'_>>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/realm",
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
    pub async fn clients_scope_mappings_realm_available_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/realm/available",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    /// GET /{realm}/clients/{id}/scope-mappings/realm/composite
    pub async fn clients_scope_mappings_realm_composite_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
    ) -> Result<Vec<RoleRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/clients/{}/scope-mappings/realm/composite",
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
    pub async fn name_get(&self, id: &str) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/name", self.url, id))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Need this for admin console to display simple name of provider when displaying user detail   KEYCLOAK-4328
    /// GET /{realm}/user-storage/{id}/name
    pub async fn user_storage_name_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/user-storage/{}/name",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove imported users
    /// POST /{realm}/user-storage/{id}/remove-imported-users
    pub async fn user_storage_remove_imported_users_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/user-storage/{}/remove-imported-users",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Trigger sync of users   Action can be "triggerFullSync" or "triggerChangedUsersSync"
    /// POST /{realm}/user-storage/{id}/sync
    pub async fn user_storage_sync_post(
        &self,
        realm: &str,
        id: &str,
        action: Option<&str>,
    ) -> Result<SynchronizationResult<'_>, KeycloakError> {
        let mut builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/user-storage/{}/sync",
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
    pub async fn user_storage_unlink_users_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/user-storage/{}/unlink-users",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Trigger sync of mapper data related to ldap mapper (roles, groups, …​)   direction is "fedToKeycloak" or "keycloakToFed"
    /// POST /{realm}/user-storage/{parentId}/mappers/{id}/sync
    pub async fn user_storage_mappers_sync_post(
        &self,
        realm: &str,
        parent_id: &str,
        id: &str,
        direction: Option<&str>,
    ) -> Result<SynchronizationResult<'_>, KeycloakError> {
        let mut builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/user-storage/{}/mappers/{}/sync",
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
    pub async fn users_post(
        &self,
        realm: &str,
        rep: UserRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!("{}/auth/admin/realms/{}/users", self.url, realm))
            .json(&rep)
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get users   Returns a list of users, filtered according to query parameters
    /// GET /{realm}/users
    pub async fn users_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        email: Option<&str>,
        enabled: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        max: Option<i32>,
        search: Option<&str>,
        username: Option<&str>,
    ) -> Result<Vec<UserRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!("{}/auth/admin/realms/{}/users", self.url, realm))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
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
        if let Some(v) = last_name {
            builder = builder.query(&[("lastName", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
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
    pub async fn users_count_get(
        &self,
        realm: &str,
        email: Option<&str>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        search: Option<&str>,
        username: Option<&str>,
    ) -> Result<i32, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/count",
                self.url, realm
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
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

    /// Get representation of the user
    /// GET /{realm}/users/{id}
    pub async fn user_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<UserRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the user
    /// PUT /{realm}/users/{id}
    pub async fn users_put(
        &self,
        realm: &str,
        id: &str,
        rep: UserRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}",
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
    pub async fn users_delete(&self, realm: &str, id: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}",
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
    pub async fn users_configured_user_storage_credential_types_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<Cow<'_, str>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/configured-user-storage-credential-types",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get consents granted by the user
    /// GET /{realm}/users/{id}/consents
    pub async fn users_consents_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<HashMap<Cow<'_, str>, Value>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/consents",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Revoke consent and offline tokens for particular client from user
    /// DELETE /{realm}/users/{id}/consents/{client}
    pub async fn users_consents_delete(
        &self,
        realm: &str,
        id: &str,
        client: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}/consents/{}",
                self.url, realm, id, client
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/users/{id}/credentials
    pub async fn users_credentials_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<CredentialRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/credentials",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a credential for a user
    /// DELETE /{realm}/users/{id}/credentials/{credentialId}
    pub async fn users_credentials_delete(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}/credentials/{}",
                self.url, realm, id, credential_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a position behind another credential
    /// POST /{realm}/users/{id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}
    pub async fn users_credentials_move_after_post(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
        new_previous_credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/credentials/{}/moveAfter/{}",
                self.url, realm, id, credential_id, new_previous_credential_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Move a credential to a first position in the credentials list of the user
    /// POST /{realm}/users/{id}/credentials/{credentialId}/moveToFirst
    pub async fn users_credentials_move_to_first_post(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/credentials/{}/moveToFirst",
                self.url, realm, id, credential_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Update a credential label for a user
    /// PUT /{realm}/users/{id}/credentials/{credentialId}/userLabel
    pub async fn users_credentials_user_label_put(
        &self,
        realm: &str,
        id: &str,
        credential_id: &str,
        user_label: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}/credentials/{}/userLabel",
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
    pub async fn users_disable_credential_types_put(
        &self,
        realm: &str,
        id: &str,
        credential_types: Vec<&str>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}/disable-credential-types",
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
    pub async fn users_execute_actions_email_put(
        &self,
        realm: &str,
        id: &str,
        client_id: Option<&str>,
        lifespan: Option<i32>,
        redirect_uri: Option<&str>,
        actions: Vec<&str>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}/execute-actions-email",
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
    pub async fn users_federated_identity_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<FederatedIdentityRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/federated-identity",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a social login provider to the user
    /// POST /{realm}/users/{id}/federated-identity/{provider}
    pub async fn users_federated_identity_post(
        &self,
        realm: &str,
        id: &str,
        provider: &str,
        rep: FederatedIdentityRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/federated-identity/{}",
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
    pub async fn users_federated_identity_delete(
        &self,
        realm: &str,
        id: &str,
        provider: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}/federated-identity/{}",
                self.url, realm, id, provider
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// GET /{realm}/users/{id}/groups
    pub async fn users_groups_get(
        &self,
        realm: &str,
        id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&str>,
    ) -> Result<Vec<GroupRepresentation<'_>>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/groups",
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
    pub async fn users_groups_count_get(
        &self,
        realm: &str,
        id: &str,
        search: Option<&str>,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let mut builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/groups/count",
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
    pub async fn users_groups_put(
        &self,
        realm: &str,
        id: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}/groups/{}",
                self.url, realm, id, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// DELETE /{realm}/users/{id}/groups/{groupId}
    pub async fn users_groups_delete(
        &self,
        realm: &str,
        id: &str,
        group_id: &str,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .delete(&format!(
                "{}/auth/admin/realms/{}/users/{}/groups/{}",
                self.url, realm, id, group_id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Impersonate the user
    /// POST /{realm}/users/{id}/impersonation
    pub async fn users_impersonation_post(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<HashMap<Cow<'_, str>, Value>, KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/impersonation",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove all user sessions associated with the user   Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    /// POST /{realm}/users/{id}/logout
    pub async fn users_logout_post(&self, realm: &str, id: &str) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .post(&format!(
                "{}/auth/admin/realms/{}/users/{}/logout",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }

    /// Get offline sessions associated with the user and client
    /// GET /{realm}/users/{id}/offline-sessions/{clientUuid}
    pub async fn users_offline_sessions_get(
        &self,
        realm: &str,
        id: &str,
        client_uuid: &str,
    ) -> Result<Vec<UserSessionRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/offline-sessions/{}",
                self.url, realm, id, client_uuid
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Set up a new password for the user.
    /// PUT /{realm}/users/{id}/reset-password
    pub async fn users_reset_password_put(
        &self,
        realm: &str,
        id: &str,
        cred: CredentialRepresentation<'_>,
    ) -> Result<(), KeycloakError> {
        let builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}/reset-password",
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
    pub async fn users_send_verify_email_put(
        &self,
        realm: &str,
        id: &str,
        client_id: Option<&str>,
        redirect_uri: Option<&str>,
    ) -> Result<(), KeycloakError> {
        let mut builder = self
            .client
            .put(&format!(
                "{}/auth/admin/realms/{}/users/{}/send-verify-email",
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
    pub async fn users_sessions_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<Vec<UserSessionRepresentation<'_>>, KeycloakError> {
        let builder = self
            .client
            .get(&format!(
                "{}/auth/admin/realms/{}/users/{}/sessions",
                self.url, realm, id
            ))
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get themes, social providers, auth providers, and event listeners available on this server
    /// GET /
    pub async fn root_get(&self) -> Result<ServerInfoRepresentation<'_>, KeycloakError> {
        let builder = self
            .client
            .get(&format!("{}/auth/admin/realms/", self.url,))
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
                &format!("{}/auth/admin/realms/", self.url,),
            )
            .bearer_auth(self.admin_token.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await?;
        Ok(())
    }
}
