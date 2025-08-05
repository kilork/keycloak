use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Authentication Management</h4>
    /// Get authenticator providers Returns a stream of authenticator providers.
    pub fn authentication_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_authenticator_providers_get(self.realm)
    }

    /// Get client authenticator providers Returns a stream of client authenticator providers.
    pub fn authentication_client_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_client_authenticator_providers_get(self.realm)
    }

    /// Create new authenticator configuration
    #[deprecated]
    pub fn authentication_config_post(
        &'a self,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_post(self.realm, body)
    }

    /// Get authenticator provider's configuration description
    pub fn authentication_config_description_with_provider_id_get(
        &'a self,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigInfoRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_description_with_provider_id_get(self.realm, provider_id)
    }

    /// Get authenticator configuration
    pub fn authentication_config_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_with_id_get(self.realm, id)
    }

    /// Update authenticator configuration
    pub fn authentication_config_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_put(self.realm, id, body)
    }

    /// Delete authenticator configuration
    pub fn authentication_config_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_delete(self.realm, id)
    }

    /// Add new authentication execution
    pub fn authentication_executions_post(
        &'a self,
        body: AuthenticationExecutionRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_post(self.realm, body)
    }

    /// Get Single Execution
    pub fn authentication_executions_with_execution_id_get(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationExecutionRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_executions_with_execution_id_get(self.realm, execution_id)
    }

    /// Delete execution
    pub fn authentication_executions_with_execution_id_delete(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_delete(self.realm, execution_id)
    }

    /// Update execution with new configuration
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
    pub fn authentication_flows_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<AuthenticationFlowRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_authentication_flows_get(self.realm)
    }

    /// Create a new authentication flow
    pub fn authentication_flows_post(
        &'a self,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_authentication_flows_post(self.realm, body)
    }

    /// Copy existing authentication flow under a new name The new name is given as 'newName' attribute of the passed JSON object
    pub fn authentication_flows_with_flow_alias_copy_post(
        &'a self,
        flow_alias: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_copy_post(self.realm, flow_alias, body)
    }

    /// Get authentication executions for a flow
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
    pub fn authentication_flows_with_flow_alias_executions_put(
        &'a self,
        flow_alias: &'a str,
        body: AuthenticationExecutionInfoRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_put(self.realm, flow_alias, body)
    }

    /// Add new authentication execution to a flow
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
    pub fn authentication_flows_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationFlowRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_flows_with_id_get(self.realm, id)
    }

    /// Update an authentication flow
    pub fn authentication_flows_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_put(self.realm, id, body)
    }

    /// Delete an authentication flow
    pub fn authentication_flows_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_delete(self.realm, id)
    }

    /// Get form action providers Returns a stream of form action providers.
    pub fn authentication_form_action_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_action_providers_get(self.realm)
    }

    /// Get form providers Returns a stream of form providers.
    pub fn authentication_form_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_providers_get(self.realm)
    }

    /// Get configuration descriptions for all clients
    pub fn authentication_per_client_config_description_get(
        &'a self,
    ) -> impl Future<
        Output = Result<TypeMap<String, TypeVec<ConfigPropertyRepresentation>>, KeycloakError>,
    > + use<'a, TS> {
        self.admin
            .realm_authentication_per_client_config_description_get(self.realm)
    }

    /// Register a new required actions
    pub fn authentication_register_required_action_post(
        &'a self,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_register_required_action_post(self.realm, body)
    }

    /// Get required actions Returns a stream of required actions.
    pub fn authentication_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_get(self.realm)
    }

    /// Get required action for alias
    pub fn authentication_required_actions_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_get(self.realm, alias)
    }

    /// Update required action
    pub fn authentication_required_actions_with_alias_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_put(self.realm, alias, body)
    }

    /// Delete required action
    pub fn authentication_required_actions_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_delete(self.realm, alias)
    }

    /// Get RequiredAction configuration
    pub fn authentication_required_actions_with_alias_config_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_config_get(self.realm, alias)
    }

    /// Update RequiredAction configuration
    pub fn authentication_required_actions_with_alias_config_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_put(self.realm, alias, body)
    }

    /// Delete RequiredAction configuration
    pub fn authentication_required_actions_with_alias_config_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_delete(self.realm, alias)
    }

    /// Get RequiredAction provider configuration description
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
    pub fn authentication_required_actions_with_alias_lower_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_lower_priority_post(self.realm, alias)
    }

    /// Raise required action's priority
    pub fn authentication_required_actions_with_alias_raise_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_raise_priority_post(self.realm, alias)
    }

    /// Get unregistered required actions Returns a stream of unregistered required actions.
    pub fn authentication_unregistered_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_unregistered_required_actions_get(self.realm)
    }
}
