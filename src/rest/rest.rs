use super::*;

impl<'a> KeycloakAdmin<'a> {
    /// Clear any user login failures for all users   This can release temporary disabled users
    /// DELETE /{realm}/attack-detection/brute-force/users
    pub async fn attack_detection_brute_force_users_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get status of a username in brute force detection
    /// GET /{realm}/attack-detection/brute-force/users/{userId}
    pub async fn attack_detection_brute_force_users_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Clear any user login failures for the user   This can release temporary disabled user
    /// DELETE /{realm}/attack-detection/brute-force/users/{userId}
    pub async fn attack_detection_brute_force_user_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get authenticator providers   Returns a list of authenticator providers.
    /// GET /{realm}/authentication/authenticator-providers
    pub async fn authentication_authenticator_providers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get client authenticator providers   Returns a list of client authenticator providers.
    /// GET /{realm}/authentication/client-authenticator-providers
    pub async fn authentication_client_authenticator_providers_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get authenticator provider’s configuration description
    /// GET /{realm}/authentication/config-description/{providerId}
    pub async fn authentication_config_description_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get authenticator configuration
    /// GET /{realm}/authentication/config/{id}
    pub async fn authentication_config_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update authenticator configuration
    /// PUT /{realm}/authentication/config/{id}
    pub async fn authentication_config_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete authenticator configuration
    /// DELETE /{realm}/authentication/config/{id}
    pub async fn authentication_config_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add new authentication execution
    /// POST /{realm}/authentication/executions
    pub async fn authentication_executions_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get Single Execution
    /// GET /{realm}/authentication/executions/{executionId}
    pub async fn authentication_executions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete execution
    /// DELETE /{realm}/authentication/executions/{executionId}
    pub async fn authentication_executions_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update execution with new configuration
    /// POST /{realm}/authentication/executions/{executionId}/config
    pub async fn authentication_executions_config_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Lower execution’s priority
    /// POST /{realm}/authentication/executions/{executionId}/lower-priority
    pub async fn authentication_executions_lower_priority_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Raise execution’s priority
    /// POST /{realm}/authentication/executions/{executionId}/raise-priority
    pub async fn authentication_executions_raise_priority_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new authentication flow
    /// POST /{realm}/authentication/flows
    pub async fn authentication_flows_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get authentication flows   Returns a list of authentication flows.
    /// GET /{realm}/authentication/flows
    pub async fn authentication_flows_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Copy existing authentication flow under a new name   The new name is given as 'newName' attribute of the passed JSON object
    /// POST /{realm}/authentication/flows/{flowAlias}/copy
    pub async fn authentication_flows_copy_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get authentication executions for a flow
    /// GET /{realm}/authentication/flows/{flowAlias}/executions
    pub async fn authentication_flows_executions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update authentication executions of a flow
    /// PUT /{realm}/authentication/flows/{flowAlias}/executions
    pub async fn authentication_flows_executions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add new authentication execution to a flow
    /// POST /{realm}/authentication/flows/{flowAlias}/executions/execution
    pub async fn authentication_flows_executions_execution_post(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add new flow with new execution to existing flow
    /// POST /{realm}/authentication/flows/{flowAlias}/executions/flow
    pub async fn authentication_flows_executions_flow_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get authentication flow for id
    /// GET /{realm}/authentication/flows/{id}
    pub async fn authentication_flow_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update an authentication flow
    /// PUT /{realm}/authentication/flows/{id}
    pub async fn authentication_flows_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete an authentication flow
    /// DELETE /{realm}/authentication/flows/{id}
    pub async fn authentication_flows_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get form action providers   Returns a list of form action providers.
    /// GET /{realm}/authentication/form-action-providers
    pub async fn authentication_form_action_providers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get form providers   Returns a list of form providers.
    /// GET /{realm}/authentication/form-providers
    pub async fn authentication_form_providers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get configuration descriptions for all clients
    /// GET /{realm}/authentication/per-client-config-description
    pub async fn authentication_per_client_config_description_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Register a new required actions
    /// POST /{realm}/authentication/register-required-action
    pub async fn authentication_register_required_action_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get required actions   Returns a list of required actions.
    /// GET /{realm}/authentication/required-actions
    pub async fn authentication_required_actions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get required action for alias
    /// GET /{realm}/authentication/required-actions/{alias}
    pub async fn authentication_required_action_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update required action
    /// PUT /{realm}/authentication/required-actions/{alias}
    pub async fn authentication_required_actions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete required action
    /// DELETE /{realm}/authentication/required-actions/{alias}
    pub async fn authentication_required_actions_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Lower required action’s priority
    /// POST /{realm}/authentication/required-actions/{alias}/lower-priority
    pub async fn authentication_required_actions_lower_priority_post(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Raise required action’s priority
    /// POST /{realm}/authentication/required-actions/{alias}/raise-priority
    pub async fn authentication_required_actions_raise_priority_post(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get unregistered required actions   Returns a list of unregistered required actions.
    /// GET /{realm}/authentication/unregistered-required-actions
    pub async fn authentication_unregistered_required_actions_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get key info
    /// GET /{realm}/clients/{id}/certificates/{attr}
    pub async fn clients_certificates_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get a keystore file for the client, containing private key and public certificate
    /// POST /{realm}/clients/{id}/certificates/{attr}/download
    pub async fn clients_certificates_download_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Generate a new certificate with new key pair
    /// POST /{realm}/clients/{id}/certificates/{attr}/generate
    pub async fn clients_certificates_generate_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Generate a new keypair and certificate, and get the private key file   Generates a keypair and certificate and serves the private key in a specified keystore format.
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    /// POST /{realm}/clients/{id}/certificates/{attr}/generate-and-download
    pub async fn clients_certificates_generate_and_download_post(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Upload certificate and eventually private key
    /// POST /{realm}/clients/{id}/certificates/{attr}/upload
    pub async fn clients_certificates_upload_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Upload only certificate, not private key
    /// POST /{realm}/clients/{id}/certificates/{attr}/upload-certificate
    pub async fn clients_certificates_upload_certificate_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new initial access token.
    /// POST /{realm}/clients-initial-access
    pub async fn clients_initial_access_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/clients-initial-access
    pub async fn clients_initial_access_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/clients-initial-access/{id}
    pub async fn clients_initial_access_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Base path for retrieve providers with the configProperties properly filled
    /// GET /{realm}/client-registration-policy/providers
    pub async fn client_registration_policy_providers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add client-level roles to the user role mapping
    /// POST /{realm}/groups/{id}/role-mappings/clients/{client}
    pub async fn groups_role_mappings_clients_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get client-level role mappings for the user, and the app
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}
    pub async fn groups_role_mappings_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete client-level roles from user role mapping
    /// DELETE /{realm}/groups/{id}/role-mappings/clients/{client}
    pub async fn groups_role_mappings_clients_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}/available
    pub async fn groups_role_mappings_clients_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective client-level role mappings   This recurses any composite roles
    /// GET /{realm}/groups/{id}/role-mappings/clients/{client}/composite
    pub async fn groups_role_mappings_clients_composite_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add client-level roles to the user role mapping
    /// POST /{realm}/users/{id}/role-mappings/clients/{client}
    pub async fn users_role_mappings_clients_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get client-level role mappings for the user, and the app
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}
    pub async fn users_role_mappings_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete client-level roles from user role mapping
    /// DELETE /{realm}/users/{id}/role-mappings/clients/{client}
    pub async fn users_role_mappings_clients_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get available client-level roles that can be mapped to the user
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}/available
    pub async fn users_role_mappings_clients_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective client-level role mappings   This recurses any composite roles
    /// GET /{realm}/users/{id}/role-mappings/clients/{client}/composite
    pub async fn users_role_mappings_clients_composite_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new client scope   Client Scope’s name must be unique!
    /// POST /{realm}/client-scopes
    pub async fn client_scopes_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get client scopes belonging to the realm   Returns a list of client scopes belonging to the realm
    /// GET /{realm}/client-scopes
    pub async fn client_scopes_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get representation of the client scope
    /// GET /{realm}/client-scopes/{id}
    pub async fn client_scope_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the client scope
    /// PUT /{realm}/client-scopes/{id}
    pub async fn client_scopes_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the client scope
    /// DELETE /{realm}/client-scopes/{id}
    pub async fn client_scopes_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new client   Client’s client_id must be unique!
    /// POST /{realm}/clients
    pub async fn clients_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get clients belonging to the realm   Returns a list of clients belonging to the realm
    /// GET /{realm}/clients
    pub async fn clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get representation of the client
    /// GET /{realm}/clients/{id}
    pub async fn client_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the client
    /// PUT /{realm}/clients/{id}
    pub async fn clients_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the client
    /// DELETE /{realm}/clients/{id}
    pub async fn clients_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Generate a new secret for the client
    /// POST /{realm}/clients/{id}/client-secret
    pub async fn clients_client_secret_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get the client secret
    /// GET /{realm}/clients/{id}/client-secret
    pub async fn clients_client_secret_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get default client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/clients/{id}/default-client-scopes
    pub async fn clients_default_client_scopes_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    pub async fn clients_default_client_scopes_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/default-client-scopes/{clientScopeId}
    pub async fn clients_default_client_scopes_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create JSON with payload of example access token
    /// GET /{realm}/clients/{id}/evaluate-scopes/generate-example-access-token
    pub async fn clients_evaluate_scopes_generate_example_access_token_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return list of all protocol mappers, which will be used when generating tokens issued for particular client.
    /// This means  protocol mappers assigned to this client directly and protocol mappers assigned to all client scopes of this client.
    /// GET /{realm}/clients/{id}/evaluate-scopes/protocol-mappers
    pub async fn clients_evaluate_scopes_protocol_mappers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective scope mapping of all roles of particular role container, which this client is defacto allowed to have in the accessToken issued for him.
    /// This contains scope mappings, which this client has directly, as well as scope mappings, which are granted to all client scopes,  which are linked with this client.
    /// GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/granted
    pub async fn clients_evaluate_scopes_scope_mappings_granted_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get roles, which this client doesn’t have scope for and can’t have them in the accessToken issued for him.
    /// Defacto all the  other roles of particular role container, which are not in {@link #getGrantedScopeMappings()}
    /// GET /{realm}/clients/{id}/evaluate-scopes/scope-mappings/{roleContainerId}/not-granted
    pub async fn clients_evaluate_scopes_scope_mappings_not_granted_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/clients/{id}/installation/providers/{providerId}
    pub async fn clients_installation_providers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/clients/{id}/management/permissions
    pub async fn clients_management_permissions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/clients/{id}/management/permissions
    pub async fn clients_management_permissions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Register a cluster node with the client   Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle  by sending registration request to Keycloak
    /// POST /{realm}/clients/{id}/nodes
    pub async fn clients_nodes_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Unregister a cluster node from the client
    /// DELETE /{realm}/clients/{id}/nodes/{node}
    pub async fn clients_nodes_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get application offline session count   Returns a number of offline user sessions associated with this client   {      "count": number  }
    /// GET /{realm}/clients/{id}/offline-session-count
    pub async fn clients_offline_session_count_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get offline sessions for client   Returns a list of offline user sessions associated with this client
    /// GET /{realm}/clients/{id}/offline-sessions
    pub async fn clients_offline_sessions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get optional client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/clients/{id}/optional-client-scopes
    pub async fn clients_optional_client_scopes_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    pub async fn clients_optional_client_scopes_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/clients/{id}/optional-client-scopes/{clientScopeId}
    pub async fn clients_optional_client_scopes_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Push the client’s revocation policy to its admin URL   If the client has an admin URL, push revocation policy to it.
    /// POST /{realm}/clients/{id}/push-revocation
    pub async fn clients_push_revocation_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Generate a new registration access token for the client
    /// POST /{realm}/clients/{id}/registration-access-token
    pub async fn clients_registration_access_token_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get a user dedicated to the service account
    /// GET /{realm}/clients/{id}/service-account-user
    pub async fn clients_service_account_user_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get application session count   Returns a number of user sessions associated with this client   {      "count": number  }
    /// GET /{realm}/clients/{id}/session-count
    pub async fn clients_session_count_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Test if registered cluster nodes are available   Tests availability by sending 'ping' request to all cluster nodes.
    /// GET /{realm}/clients/{id}/test-nodes-available
    pub async fn clients_test_nodes_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get user sessions for client   Returns a list of user sessions associated with this client
    /// GET /{realm}/clients/{id}/user-sessions
    pub async fn clients_user_sessions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// POST /{realm}/components
    pub async fn components_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/components
    pub async fn components_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/components/{id}
    pub async fn component_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/components/{id}
    pub async fn components_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/components/{id}
    pub async fn components_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
    /// GET /{realm}/components/{id}/sub-component-types
    pub async fn components_sub_component_types_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// create or add a top level realm groupSet or create child.
    /// This will update the group and set the parent if it exists. Create it and set the parent  if the group doesn’t exist.
    /// POST /{realm}/groups
    pub async fn groups_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get group hierarchy.
    /// Only name and ids are returned.
    /// GET /{realm}/groups
    pub async fn groups_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Returns the groups counts.
    /// GET /{realm}/groups/count
    pub async fn groups_count_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/groups/{id}
    pub async fn group_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update group, ignores subgroups.
    /// PUT /{realm}/groups/{id}
    pub async fn groups_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/groups/{id}
    pub async fn groups_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Set or create child.
    /// This will just set the parent if it exists. Create it and set the parent  if the group doesn’t exist.
    /// POST /{realm}/groups/{id}/children
    pub async fn groups_children_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/groups/{id}/management/permissions
    pub async fn groups_management_permissions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/groups/{id}/management/permissions
    pub async fn groups_management_permissions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get users   Returns a list of users, filtered according to query parameters
    /// GET /{realm}/groups/{id}/members
    pub async fn groups_members_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Import identity provider from uploaded JSON file
    /// POST /{realm}/identity-provider/import-config
    pub async fn identity_provider_import_config_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new identity provider
    /// POST /{realm}/identity-provider/instances
    pub async fn identity_provider_instances_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get identity providers
    /// GET /{realm}/identity-provider/instances
    pub async fn identity_provider_instances_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get the identity provider
    /// GET /{realm}/identity-provider/instances/{alias}
    pub async fn identity_provider_instance_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the identity provider
    /// PUT /{realm}/identity-provider/instances/{alias}
    pub async fn identity_provider_instances_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the identity provider
    /// DELETE /{realm}/identity-provider/instances/{alias}
    pub async fn identity_provider_instances_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Export public broker configuration for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/export
    pub async fn identity_provider_instances_export_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// GET /{realm}/identity-provider/instances/{alias}/management/permissions
    pub async fn identity_provider_instances_management_permissions_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    /// PUT /{realm}/identity-provider/instances/{alias}/management/permissions
    pub async fn identity_provider_instances_management_permissions_put(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mapper types for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mapper-types
    pub async fn identity_provider_instances_mapper_types_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add a mapper to identity provider
    /// POST /{realm}/identity-provider/instances/{alias}/mappers
    pub async fn identity_provider_instances_mappers_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mappers for identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mappers
    pub async fn identity_provider_instances_mappers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mapper by id for the identity provider
    /// GET /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn identity_provider_instances_mapper_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update a mapper for the identity provider
    /// PUT /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn identity_provider_instances_mappers_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete a mapper for the identity provider
    /// DELETE /{realm}/identity-provider/instances/{alias}/mappers/{id}
    pub async fn identity_provider_instances_mappers_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get identity providers
    /// GET /{realm}/identity-provider/providers/{provider_id}
    pub async fn identity_provider_providers_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/keys
    pub async fn keys_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create multiple mappers
    /// POST /{realm}/client-scopes/{id}/protocol-mappers/add-models
    pub async fn client_scopes_protocol_mappers_add_models_post(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a mapper
    /// POST /{realm}/client-scopes/{id}/protocol-mappers/models
    pub async fn client_scopes_protocol_mappers_models_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mappers
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/models
    pub async fn client_scopes_protocol_mappers_models_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mapper by id
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn client_scopes_protocol_mappers_model_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the mapper
    /// PUT /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn client_scopes_protocol_mappers_models_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the mapper
    /// DELETE /{realm}/client-scopes/{id}/protocol-mappers/models/{id}
    pub async fn client_scopes_protocol_mappers_models_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    /// GET /{realm}/client-scopes/{id}/protocol-mappers/protocol/{protocol}
    pub async fn client_scopes_protocol_mappers_protocol_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create multiple mappers
    /// POST /{realm}/clients/{id}/protocol-mappers/add-models
    pub async fn clients_protocol_mappers_add_models_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a mapper
    /// POST /{realm}/clients/{id}/protocol-mappers/models
    pub async fn clients_protocol_mappers_models_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mappers
    /// GET /{realm}/clients/{id}/protocol-mappers/models
    pub async fn clients_protocol_mappers_models_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mapper by id
    /// GET /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn clients_protocol_mappers_model_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the mapper
    /// PUT /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn clients_protocol_mappers_models_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the mapper
    /// DELETE /{realm}/clients/{id}/protocol-mappers/models/{id}
    pub async fn clients_protocol_mappers_models_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get mappers by name for a specific protocol
    /// GET /{realm}/clients/{id}/protocol-mappers/protocol/{protocol}
    pub async fn clients_protocol_mappers_protocol_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Import a realm   Imports a realm from a full representation of that realm.
    /// Realm name must be unique.
    /// POST /
    pub async fn post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get the top-level representation of the realm   It will not include nested information like User and Client representations.
    /// GET /{realm}
    pub async fn get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the top-level information of the realm   Any user, roles or client information in the representation  will be ignored.
    /// This will only update top-level attributes of the realm.
    /// PUT /{realm}
    pub async fn put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the realm
    /// DELETE /{realm}
    pub async fn delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get admin events   Returns all admin events, or filters events based on URL query parameters listed here
    /// GET /{realm}/admin-events
    pub async fn admin_events_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete all admin events
    /// DELETE /{realm}/admin-events
    pub async fn admin_events_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Clear cache of external public keys (Public keys of clients or Identity providers)
    /// POST /{realm}/clear-keys-cache
    pub async fn clear_keys_cache_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Clear realm cache
    /// POST /{realm}/clear-realm-cache
    pub async fn clear_realm_cache_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Clear user cache
    /// POST /{realm}/clear-user-cache
    pub async fn clear_user_cache_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Base path for importing clients under this realm.
    /// POST /{realm}/client-description-converter
    pub async fn client_description_converter_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get client session stats   Returns a JSON map.
    /// The key is the client id, the value is the number of sessions that currently are active  with that client. Only clients that actually have a session associated with them will be in this map.
    /// GET /{realm}/client-session-stats
    pub async fn client_session_stats_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/credential-registrators
    pub async fn credential_registrators_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm default client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/default-default-client-scopes
    pub async fn default_default_client_scopes_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/default-default-client-scopes/{clientScopeId}
    pub async fn default_default_client_scopes_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/default-default-client-scopes/{clientScopeId}
    pub async fn default_default_client_scopes_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get group hierarchy.
    /// Only name and ids are returned.
    /// GET /{realm}/default-groups
    pub async fn default_groups_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/default-groups/{groupId}
    pub async fn default_groups_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/default-groups/{groupId}
    pub async fn default_groups_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm optional client scopes.
    /// Only name and ids are returned.
    /// GET /{realm}/default-optional-client-scopes
    pub async fn default_optional_client_scopes_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/default-optional-client-scopes/{clientScopeId}
    pub async fn default_optional_client_scopes_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/default-optional-client-scopes/{clientScopeId}
    pub async fn default_optional_client_scopes_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get events   Returns all events, or filters them based on URL query parameters listed here
    /// GET /{realm}/events
    pub async fn events_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete all events
    /// DELETE /{realm}/events
    pub async fn events_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get the events provider configuration   Returns JSON object with events provider configuration
    /// GET /{realm}/events/config
    pub async fn events_config_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the events provider   Change the events provider and/or its configuration
    /// PUT /{realm}/events/config
    pub async fn events_config_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/group-by-path/{path}
    pub async fn group_by_path_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Removes all user sessions.
    /// Any client that has an admin url will also be told to invalidate any sessions  they have.
    /// POST /{realm}/logout-all
    pub async fn logout_all_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Partial export of existing realm into a JSON file.
    /// POST /{realm}/partial-export
    pub async fn partial_export_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Partial import from a JSON file to an existing realm.
    /// POST /{realm}/partialImport
    pub async fn partial_import_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Push the realm’s revocation policy to any client that has an admin url associated with it.
    /// POST /{realm}/push-revocation
    pub async fn push_revocation_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove a specific user session.
    /// Any client that has an admin url will also be told to invalidate this  particular session.
    /// DELETE /{realm}/sessions/{session}
    pub async fn sessions_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Test LDAP connection
    /// POST /{realm}/testLDAPConnection
    pub async fn test_ldap_connection_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Test SMTP connection with current logged in user
    /// POST /{realm}/testSMTPConnection
    pub async fn test_smtp_connection_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/users-management-permissions
    pub async fn users_management_permissions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/users-management-permissions
    pub async fn users_management_permissions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get role mappings
    /// GET /{realm}/groups/{id}/role-mappings
    pub async fn groups_role_mappings_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add realm-level role mappings to the user
    /// POST /{realm}/groups/{id}/role-mappings/realm
    pub async fn groups_role_mappings_realm_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level role mappings
    /// GET /{realm}/groups/{id}/role-mappings/realm
    pub async fn groups_role_mappings_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete realm-level role mappings
    /// DELETE /{realm}/groups/{id}/role-mappings/realm
    pub async fn groups_role_mappings_realm_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    /// GET /{realm}/groups/{id}/role-mappings/realm/available
    pub async fn groups_role_mappings_realm_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    /// GET /{realm}/groups/{id}/role-mappings/realm/composite
    pub async fn groups_role_mappings_realm_composite_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get role mappings
    /// GET /{realm}/users/{id}/role-mappings
    pub async fn users_role_mappings_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add realm-level role mappings to the user
    /// POST /{realm}/users/{id}/role-mappings/realm
    pub async fn users_role_mappings_realm_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level role mappings
    /// GET /{realm}/users/{id}/role-mappings/realm
    pub async fn users_role_mappings_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete realm-level role mappings
    /// DELETE /{realm}/users/{id}/role-mappings/realm
    pub async fn users_role_mappings_realm_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles that can be mapped
    /// GET /{realm}/users/{id}/role-mappings/realm/available
    pub async fn users_role_mappings_realm_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective realm-level role mappings   This will recurse all composite roles to get the result.
    /// GET /{realm}/users/{id}/role-mappings/realm/composite
    pub async fn users_role_mappings_realm_composite_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new role for the realm or client
    /// POST /{realm}/clients/{id}/roles
    pub async fn clients_roles_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get all roles for the realm or client
    /// GET /{realm}/clients/{id}/roles
    pub async fn clients_roles_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get a role by name
    /// GET /{realm}/clients/{id}/roles/{role-name}
    pub async fn clients_role_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update a role by name
    /// PUT /{realm}/clients/{id}/roles/{role-name}
    pub async fn clients_roles_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete a role by name
    /// DELETE /{realm}/clients/{id}/roles/{role-name}
    pub async fn clients_roles_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add a composite to the role
    /// POST /{realm}/clients/{id}/roles/{role-name}/composites
    pub async fn clients_roles_composites_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get composites of the role
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites
    pub async fn clients_roles_composites_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove roles from the role’s composite
    /// DELETE /{realm}/clients/{id}/roles/{role-name}/composites
    pub async fn clients_roles_composites_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// An app-level roles for the specified app for the role’s composite
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites/clients/{client}
    pub async fn clients_roles_composites_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles of the role’s composite
    /// GET /{realm}/clients/{id}/roles/{role-name}/composites/realm
    pub async fn clients_roles_composites_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return List of Groups that have the specified role name
    /// GET /{realm}/clients/{id}/roles/{role-name}/groups
    pub async fn clients_roles_groups_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/clients/{id}/roles/{role-name}/management/permissions
    pub async fn clients_roles_management_permissions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/clients/{id}/roles/{role-name}/management/permissions
    pub async fn clients_roles_management_permissions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return List of Users that have the specified role name
    /// GET /{realm}/clients/{id}/roles/{role-name}/users
    pub async fn clients_roles_users_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new role for the realm or client
    /// POST /{realm}/roles
    pub async fn roles_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get all roles for the realm or client
    /// GET /{realm}/roles
    pub async fn roles_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get a role by name
    /// GET /{realm}/roles/{role-name}
    pub async fn role_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update a role by name
    /// PUT /{realm}/roles/{role-name}
    pub async fn roles_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete a role by name
    /// DELETE /{realm}/roles/{role-name}
    pub async fn roles_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add a composite to the role
    /// POST /{realm}/roles/{role-name}/composites
    pub async fn roles_composites_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get composites of the role
    /// GET /{realm}/roles/{role-name}/composites
    pub async fn roles_composites_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove roles from the role’s composite
    /// DELETE /{realm}/roles/{role-name}/composites
    pub async fn roles_composites_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// An app-level roles for the specified app for the role’s composite
    /// GET /{realm}/roles/{role-name}/composites/clients/{client}
    pub async fn roles_composites_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles of the role’s composite
    /// GET /{realm}/roles/{role-name}/composites/realm
    pub async fn roles_composites_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return List of Groups that have the specified role name
    /// GET /{realm}/roles/{role-name}/groups
    pub async fn roles_groups_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/roles/{role-name}/management/permissions
    pub async fn roles_management_permissions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/roles/{role-name}/management/permissions
    pub async fn roles_management_permissions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return List of Users that have the specified role name
    /// GET /{realm}/roles/{role-name}/users
    pub async fn roles_users_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get a specific role’s representation
    /// GET /{realm}/roles-by-id/{role-id}
    pub async fn roles_by_id_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the role
    /// PUT /{realm}/roles-by-id/{role-id}
    pub async fn roles_by_id_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the role
    /// DELETE /{realm}/roles-by-id/{role-id}
    pub async fn roles_by_id_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Make the role a composite role by associating some child roles
    /// POST /{realm}/roles-by-id/{role-id}/composites
    pub async fn roles_by_id_composites_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get role’s children   Returns a set of role’s children provided the role is a composite.
    /// GET /{realm}/roles-by-id/{role-id}/composites
    pub async fn roles_by_id_composites_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove a set of roles from the role’s composite
    /// DELETE /{realm}/roles-by-id/{role-id}/composites
    pub async fn roles_by_id_composites_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get client-level roles for the client that are in the role’s composite
    /// GET /{realm}/roles-by-id/{role-id}/composites/clients/{client}
    pub async fn roles_by_id_composites_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles that are in the role’s composite
    /// GET /{realm}/roles-by-id/{role-id}/composites/realm
    pub async fn roles_by_id_composites_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// GET /{realm}/roles-by-id/{role-id}/management/permissions
    pub async fn roles_by_id_management_permissions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return object stating whether role Authoirzation permissions have been initialized or not and a reference
    /// PUT /{realm}/roles-by-id/{role-id}/management/permissions
    pub async fn roles_by_id_management_permissions_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get all scope mappings for the client
    /// GET /{realm}/client-scopes/{id}/scope-mappings
    pub async fn client_scopes_scope_mappings_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add client-level roles to the client’s scope
    /// POST /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
    pub async fn client_scopes_scope_mappings_clients_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get the roles associated with a client’s scope   Returns roles for the client.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
    pub async fn client_scopes_scope_mappings_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove client-level roles from the client’s scope.
    /// DELETE /{realm}/client-scopes/{id}/scope-mappings/clients/{client}
    pub async fn client_scopes_scope_mappings_clients_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// The available client-level roles   Returns the roles for the client that can be associated with the client’s scope
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/available
    pub async fn client_scopes_scope_mappings_clients_available_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/clients/{client}/composite
    pub async fn client_scopes_scope_mappings_clients_composite_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add a set of realm-level roles to the client’s scope
    /// POST /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn client_scopes_scope_mappings_realm_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles associated with the client’s scope
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn client_scopes_scope_mappings_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove a set of realm-level roles from the client’s scope
    /// DELETE /{realm}/client-scopes/{id}/scope-mappings/realm
    pub async fn client_scopes_scope_mappings_realm_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client’s scope
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm/available
    pub async fn client_scopes_scope_mappings_realm_available_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    /// GET /{realm}/client-scopes/{id}/scope-mappings/realm/composite
    pub async fn client_scopes_scope_mappings_realm_composite_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get all scope mappings for the client
    /// GET /{realm}/clients/{id}/scope-mappings
    pub async fn clients_scope_mappings_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add client-level roles to the client’s scope
    /// POST /{realm}/clients/{id}/scope-mappings/clients/{client}
    pub async fn clients_scope_mappings_clients_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get the roles associated with a client’s scope   Returns roles for the client.
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}
    pub async fn clients_scope_mappings_clients_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove client-level roles from the client’s scope.
    /// DELETE /{realm}/clients/{id}/scope-mappings/clients/{client}
    pub async fn clients_scope_mappings_clients_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// The available client-level roles   Returns the roles for the client that can be associated with the client’s scope
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}/available
    pub async fn clients_scope_mappings_clients_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective client roles   Returns the roles for the client that are associated with the client’s scope.
    /// GET /{realm}/clients/{id}/scope-mappings/clients/{client}/composite
    pub async fn clients_scope_mappings_clients_composite_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add a set of realm-level roles to the client’s scope
    /// POST /{realm}/clients/{id}/scope-mappings/realm
    pub async fn clients_scope_mappings_realm_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles associated with the client’s scope
    /// GET /{realm}/clients/{id}/scope-mappings/realm
    pub async fn clients_scope_mappings_realm_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove a set of realm-level roles from the client’s scope
    /// DELETE /{realm}/clients/{id}/scope-mappings/realm
    pub async fn clients_scope_mappings_realm_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get realm-level roles that are available to attach to this client’s scope
    /// GET /{realm}/clients/{id}/scope-mappings/realm/available
    pub async fn clients_scope_mappings_realm_available_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get effective realm-level roles associated with the client’s scope   What this does is recurse  any composite roles associated with the client’s scope and adds the roles to this lists.
    /// The method is really  to show a comprehensive total view of realm-level roles associated with the client.
    /// GET /{realm}/clients/{id}/scope-mappings/realm/composite
    pub async fn clients_scope_mappings_realm_composite_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Need this for admin console to display simple name of provider when displaying client detail   KEYCLOAK-4328
    /// GET /{id}/name
    pub async fn name_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Need this for admin console to display simple name of provider when displaying user detail   KEYCLOAK-4328
    /// GET /{realm}/user-storage/{id}/name
    pub async fn user_storage_name_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove imported users
    /// POST /{realm}/user-storage/{id}/remove-imported-users
    pub async fn user_storage_remove_imported_users_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Trigger sync of users   Action can be "triggerFullSync" or "triggerChangedUsersSync"
    /// POST /{realm}/user-storage/{id}/sync
    pub async fn user_storage_sync_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Unlink imported users from a storage provider
    /// POST /{realm}/user-storage/{id}/unlink-users
    pub async fn user_storage_unlink_users_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Trigger sync of mapper data related to ldap mapper (roles, groups, …​)   direction is "fedToKeycloak" or "keycloakToFed"
    /// POST /{realm}/user-storage/{parentId}/mappers/{id}/sync
    pub async fn user_storage_mappers_sync_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Create a new user   Username must be unique.
    /// POST /{realm}/users
    pub async fn users_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get users   Returns a list of users, filtered according to query parameters
    /// GET /{realm}/users
    pub async fn users_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Returns the number of users that match the given criteria.
    /// It can be called in three different ways.  1. Don’t specify any criteria and pass {@code null}. The number of all  users within that realm will be returned.  <p>  2. If {@code search} is specified other criteria such as {@code last} will  be ignored even though you set them. The {@code search} string will be  matched against the first and last name, the username and the email of a  user.  <p>  3. If {@code search} is unspecified but any of {@code last}, {@code first},  {@code email} or {@code username} those criteria are matched against their  respective fields on a user entity. Combined with a logical and.
    /// GET /{realm}/users/count
    pub async fn users_count_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get representation of the user
    /// GET /{realm}/users/{id}
    pub async fn user_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update the user
    /// PUT /{realm}/users/{id}
    pub async fn users_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Delete the user
    /// DELETE /{realm}/users/{id}
    pub async fn users_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    /// Returned values can contain for example "password", "otp" etc.  This will always return empty list for "local" users, which are not backed by any user storage
    /// GET /{realm}/users/{id}/configured-user-storage-credential-types
    pub async fn users_configured_user_storage_credential_types_get(
        &self,
    ) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get consents granted by the user
    /// GET /{realm}/users/{id}/consents
    pub async fn users_consents_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Revoke consent and offline tokens for particular client from user
    /// DELETE /{realm}/users/{id}/consents/{client}
    pub async fn users_consents_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/users/{id}/credentials
    pub async fn users_credentials_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove a credential for a user
    /// DELETE /{realm}/users/{id}/credentials/{credentialId}
    pub async fn users_credentials_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Move a credential to a position behind another credential
    /// POST /{realm}/users/{id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}
    pub async fn users_credentials_move_after_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Move a credential to a first position in the credentials list of the user
    /// POST /{realm}/users/{id}/credentials/{credentialId}/moveToFirst
    pub async fn users_credentials_move_to_first_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Update a credential label for a user
    /// PUT /{realm}/users/{id}/credentials/{credentialId}/userLabel
    pub async fn users_credentials_user_label_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Disable all credentials for a user of a specific type
    /// PUT /{realm}/users/{id}/disable-credential-types
    pub async fn users_disable_credential_types_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Send a update account email to the user   An email contains a link the user can click to perform a set of required actions.
    /// The redirectUri and clientId parameters are optional. If no redirect is given, then there will  be no link back to click after actions have completed. Redirect uri must be a valid uri for the  particular clientId.
    /// PUT /{realm}/users/{id}/execute-actions-email
    pub async fn users_execute_actions_email_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get social logins associated with the user
    /// GET /{realm}/users/{id}/federated-identity
    pub async fn users_federated_identity_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Add a social login provider to the user
    /// POST /{realm}/users/{id}/federated-identity/{provider}
    pub async fn users_federated_identity_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove a social login provider from user
    /// DELETE /{realm}/users/{id}/federated-identity/{provider}
    pub async fn users_federated_identity_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/users/{id}/groups
    pub async fn users_groups_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// GET /{realm}/users/{id}/groups/count
    pub async fn users_groups_count_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// PUT /{realm}/users/{id}/groups/{groupId}
    pub async fn users_groups_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// DELETE /{realm}/users/{id}/groups/{groupId}
    pub async fn users_groups_delete(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Impersonate the user
    /// POST /{realm}/users/{id}/impersonation
    pub async fn users_impersonation_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Remove all user sessions associated with the user   Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    /// POST /{realm}/users/{id}/logout
    pub async fn users_logout_post(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get offline sessions associated with the user and client
    /// GET /{realm}/users/{id}/offline-sessions/{clientId}
    pub async fn users_offline_sessions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Set up a new password for the user.
    /// PUT /{realm}/users/{id}/reset-password
    pub async fn users_reset_password_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Send an email-verification email to the user   An email contains a link the user can click to verify their email address.
    /// The redirectUri and clientId parameters are optional. The default for the  redirect is the account client.
    /// PUT /{realm}/users/{id}/send-verify-email
    pub async fn users_send_verify_email_put(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get sessions associated with the user
    /// GET /{realm}/users/{id}/sessions
    pub async fn users_sessions_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// Get themes, social providers, auth providers, and event listeners available on this server
    /// GET /
    pub async fn root_get(&self) -> Result<(), KeycloakError> {
        Ok(())
    }

    /// CORS preflight
    /// OPTIONS /{any}
    pub async fn any_options(&self) -> Result<(), KeycloakError> {
        Ok(())
    }
}
