use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
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
    pub fn authentication_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_authenticator_providers_get(self.realm)
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
    pub fn authentication_client_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_client_authenticator_providers_get(self.realm)
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
    pub fn authentication_config_post(
        &'a self,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_post(self.realm, body)
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
    pub fn authentication_config_description_with_provider_id_get(
        &'a self,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigInfoRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_description_with_provider_id_get(self.realm, provider_id)
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
    pub fn authentication_config_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_with_id_get(self.realm, id)
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
    pub fn authentication_config_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_put(self.realm, id, body)
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
    pub fn authentication_config_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_delete(self.realm, id)
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
    pub fn authentication_executions_post(
        &'a self,
        body: AuthenticationExecutionRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_post(self.realm, body)
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
    pub fn authentication_executions_with_execution_id_get(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationExecutionRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_executions_with_execution_id_get(self.realm, execution_id)
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
    pub fn authentication_executions_with_execution_id_delete(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_delete(self.realm, execution_id)
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
    pub fn authentication_executions_with_execution_id_config_post(
        &'a self,
        execution_id: &'a str,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_config_post(
                self.realm,
                execution_id,
                body,
            )
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
    pub fn authentication_executions_with_execution_id_config_with_id_get(
        &'a self,
        execution_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_executions_with_execution_id_config_with_id_get(
                self.realm,
                execution_id,
                id,
            )
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
    pub fn authentication_executions_with_execution_id_lower_priority_post(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_lower_priority_post(
                self.realm,
                execution_id,
            )
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
    pub fn authentication_executions_with_execution_id_raise_priority_post(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_raise_priority_post(
                self.realm,
                execution_id,
            )
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
    pub fn authentication_flows_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<AuthenticationFlowRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_authentication_flows_get(self.realm)
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
    pub fn authentication_flows_post(
        &'a self,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_authentication_flows_post(self.realm, body)
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
    pub fn authentication_flows_with_flow_alias_copy_post(
        &'a self,
        flow_alias: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_copy_post(self.realm, flow_alias, body)
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
    pub fn authentication_flows_with_flow_alias_executions_get(
        &'a self,
        flow_alias: &'a str,
    ) -> impl Future<
        Output = Result<TypeVec<AuthenticationExecutionInfoRepresentation>, KeycloakError>,
    > + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_get(self.realm, flow_alias)
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
    pub fn authentication_flows_with_flow_alias_executions_put(
        &'a self,
        flow_alias: &'a str,
        body: AuthenticationExecutionInfoRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_put(self.realm, flow_alias, body)
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
    pub fn authentication_flows_with_flow_alias_executions_execution_post(
        &'a self,
        flow_alias: &'a str,
        body: TypeMap<String, Value>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_execution_post(
                self.realm, flow_alias, body,
            )
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
    pub fn authentication_flows_with_flow_alias_executions_flow_post(
        &'a self,
        flow_alias: &'a str,
        body: TypeMap<String, Value>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_flow_post(
                self.realm, flow_alias, body,
            )
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
    pub fn authentication_flows_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationFlowRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_flows_with_id_get(self.realm, id)
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
    pub fn authentication_flows_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_put(self.realm, id, body)
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
    pub fn authentication_flows_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_delete(self.realm, id)
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
    pub fn authentication_form_action_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_action_providers_get(self.realm)
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
    pub fn authentication_form_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_providers_get(self.realm)
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
    pub fn authentication_per_client_config_description_get(
        &'a self,
    ) -> impl Future<
        Output = Result<TypeMap<String, TypeVec<ConfigPropertyRepresentation>>, KeycloakError>,
    > + use<'a, TS> {
        self.admin
            .realm_authentication_per_client_config_description_get(self.realm)
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
    pub fn authentication_register_required_action_post(
        &'a self,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_register_required_action_post(self.realm, body)
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
    pub fn authentication_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_get(self.realm)
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
    pub fn authentication_required_actions_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_get(self.realm, alias)
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
    pub fn authentication_required_actions_with_alias_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_put(self.realm, alias, body)
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
    pub fn authentication_required_actions_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_delete(self.realm, alias)
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
    pub fn authentication_required_actions_with_alias_config_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_config_get(self.realm, alias)
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
    pub fn authentication_required_actions_with_alias_config_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_put(self.realm, alias, body)
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
    pub fn authentication_required_actions_with_alias_config_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_delete(self.realm, alias)
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
    pub fn authentication_required_actions_with_alias_config_description_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionConfigInfoRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_config_description_get(
                self.realm, alias,
            )
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
    pub fn authentication_required_actions_with_alias_lower_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_lower_priority_post(self.realm, alias)
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
    pub fn authentication_required_actions_with_alias_raise_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_raise_priority_post(self.realm, alias)
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
    pub fn authentication_unregistered_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_unregistered_required_actions_get(self.realm)
    }
}
