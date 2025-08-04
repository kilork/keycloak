use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Attack Detection</h4>
    /// Clear any user login failures for all users This can release temporary disabled users
    #[cfg(feature = "tag-attack-detection")]
    pub fn attack_detection_brute_force_users_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_delete(self.realm)
    }

    /// Get status of a username in brute force detection
    #[cfg(feature = "tag-attack-detection")]
    pub fn attack_detection_brute_force_users_with_user_id_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_get(self.realm, user_id)
    }

    /// Clear any user login failures for the user This can release temporary disabled user
    #[cfg(feature = "tag-attack-detection")]
    pub fn attack_detection_brute_force_users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_delete(self.realm, user_id)
    }

    // <h4>Authentication Management</h4>
    /// Get authenticator providers Returns a stream of authenticator providers.
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_authenticator_providers_get(self.realm)
    }

    /// Get client authenticator providers Returns a stream of client authenticator providers.
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_client_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_client_authenticator_providers_get(self.realm)
    }

    /// Create new authenticator configuration
    #[cfg(feature = "tag-authentication-management")]
    #[deprecated]
    pub fn authentication_config_post(
        &'a self,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_post(self.realm, body)
    }

    /// Get authenticator provider's configuration description
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_description_with_provider_id_get(
        &'a self,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigInfoRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_description_with_provider_id_get(self.realm, provider_id)
    }

    /// Get authenticator configuration
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_with_id_get(self.realm, id)
    }

    /// Update authenticator configuration
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_put(self.realm, id, body)
    }

    /// Delete authenticator configuration
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_delete(self.realm, id)
    }

    /// Add new authentication execution
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_executions_post(
        &'a self,
        body: AuthenticationExecutionRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_post(self.realm, body)
    }

    /// Get Single Execution
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_executions_with_execution_id_get(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationExecutionRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_executions_with_execution_id_get(self.realm, execution_id)
    }

    /// Delete execution
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_executions_with_execution_id_delete(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_delete(self.realm, execution_id)
    }

    /// Update execution with new configuration
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<AuthenticationFlowRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_authentication_flows_get(self.realm)
    }

    /// Create a new authentication flow
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_post(
        &'a self,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_authentication_flows_post(self.realm, body)
    }

    /// Copy existing authentication flow under a new name The new name is given as 'newName' attribute of the passed JSON object
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_flow_alias_copy_post(
        &'a self,
        flow_alias: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_copy_post(self.realm, flow_alias, body)
    }

    /// Get authentication executions for a flow
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_flow_alias_executions_put(
        &'a self,
        flow_alias: &'a str,
        body: AuthenticationExecutionInfoRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_put(self.realm, flow_alias, body)
    }

    /// Add new authentication execution to a flow
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationFlowRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_flows_with_id_get(self.realm, id)
    }

    /// Update an authentication flow
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_put(self.realm, id, body)
    }

    /// Delete an authentication flow
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_delete(self.realm, id)
    }

    /// Get form action providers Returns a stream of form action providers.
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_form_action_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_action_providers_get(self.realm)
    }

    /// Get form providers Returns a stream of form providers.
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_form_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_providers_get(self.realm)
    }

    /// Get configuration descriptions for all clients
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_per_client_config_description_get(
        &'a self,
    ) -> impl Future<
        Output = Result<TypeMap<String, TypeVec<ConfigPropertyRepresentation>>, KeycloakError>,
    > + use<'a, TS> {
        self.admin
            .realm_authentication_per_client_config_description_get(self.realm)
    }

    /// Register a new required actions
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_register_required_action_post(
        &'a self,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_register_required_action_post(self.realm, body)
    }

    /// Get required actions Returns a stream of required actions.
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_get(self.realm)
    }

    /// Get required action for alias
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_get(self.realm, alias)
    }

    /// Update required action
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_put(self.realm, alias, body)
    }

    /// Delete required action
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_delete(self.realm, alias)
    }

    /// Get RequiredAction configuration
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_config_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_config_get(self.realm, alias)
    }

    /// Update RequiredAction configuration
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_config_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_put(self.realm, alias, body)
    }

    /// Delete RequiredAction configuration
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_config_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_delete(self.realm, alias)
    }

    /// Get RequiredAction provider configuration description
    #[cfg(feature = "tag-authentication-management")]
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
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_lower_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_lower_priority_post(self.realm, alias)
    }

    /// Raise required action's priority
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_raise_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_raise_priority_post(self.realm, alias)
    }

    /// Get unregistered required actions Returns a stream of unregistered required actions.
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_unregistered_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_unregistered_required_actions_get(self.realm)
    }

    // <h4>Client Attribute Certificate</h4>
    /// Get key info
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub fn clients_with_client_uuid_certificates_with_attr_get(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_get(
                self.realm,
                client_uuid,
                attr,
            )
    }

    /// Get a keystore file for the client, containing private key and public certificate
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub fn clients_with_client_uuid_certificates_with_attr_download_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
        body: KeyStoreConfig,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_download_post(
                self.realm,
                client_uuid,
                attr,
                body,
            )
    }

    /// Generate a new certificate with new key pair
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub fn clients_with_client_uuid_certificates_with_attr_generate_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_generate_post(
                self.realm,
                client_uuid,
                attr,
            )
    }

    /// Generate a new keypair and certificate, and get the private key file
    ///
    /// Generates a keypair and certificate and serves the private key in a specified keystore format.
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub fn clients_with_client_uuid_certificates_with_attr_generate_and_download_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
        body: KeyStoreConfig,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_generate_and_download_post(
                self.realm,
                client_uuid,
                attr,
                body,
            )
    }

    /// Upload certificate and eventually private key
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub fn clients_with_client_uuid_certificates_with_attr_upload_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_upload_post(
                self.realm,
                client_uuid,
                attr,
            )
    }

    /// Upload only certificate, not private key
    #[cfg(feature = "tag-client-attribute-certificate")]
    pub fn clients_with_client_uuid_certificates_with_attr_upload_certificate_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_upload_certificate_post(
                self.realm,
                client_uuid,
                attr,
            )
    }

    // <h4>Client Initial Access</h4>
    #[cfg(feature = "tag-client-initial-access")]
    pub fn clients_initial_access_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientInitialAccessPresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_clients_initial_access_get(self.realm)
    }

    /// Create a new initial access token.
    #[cfg(feature = "tag-client-initial-access")]
    pub fn clients_initial_access_post(
        &'a self,
        body: ClientInitialAccessCreatePresentation,
    ) -> impl Future<Output = Result<ClientInitialAccessCreatePresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_initial_access_post(self.realm, body)
    }

    #[cfg(feature = "tag-client-initial-access")]
    pub fn clients_initial_access_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_initial_access_with_id_delete(self.realm, id)
    }

    // <h4>Client Registration Policy</h4>
    /// Base path for retrieve providers with the configProperties properly filled
    #[cfg(feature = "tag-client-registration-policy")]
    pub fn client_registration_policy_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_registration_policy_providers_get(self.realm)
    }

    // <h4>Client Role Mappings</h4>
    /// Get client-level role mappings for the user or group, and the app
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn groups_with_group_id_role_mappings_clients_with_client_id_get(
        &'a self,
        group_id: &'a str,
        client_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_clients_with_client_id_get(
                self.realm, group_id, client_id,
            )
    }

    /// Add client-level roles to the user or group role mapping
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn groups_with_group_id_role_mappings_clients_with_client_id_post(
        &'a self,
        group_id: &'a str,
        client_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_clients_with_client_id_post(
                self.realm, group_id, client_id, body,
            )
    }

    /// Delete client-level roles from user or group role mapping
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn groups_with_group_id_role_mappings_clients_with_client_id_delete(
        &'a self,
        group_id: &'a str,
        client_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_clients_with_client_id_delete(
                self.realm, group_id, client_id, body,
            )
    }

    /// Get available client-level roles that can be mapped to the user or group
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn groups_with_group_id_role_mappings_clients_with_client_id_available_get(
        &'a self,
        group_id: &'a str,
        client_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_clients_with_client_id_available_get(
                self.realm, group_id, client_id,
            )
    }

    /// Get effective client-level role mappings This recurses any composite roles
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn groups_with_group_id_role_mappings_clients_with_client_id_composite_get(
        &'a self,
        group_id: &'a str,
        client_id: &'a str,
    ) -> RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS> {
        RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet {
            realm_admin: self,
            group_id,
            client_id,
        }
    }

    /// Get client-level role mappings for the user or group, and the app
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn users_with_user_id_role_mappings_clients_with_client_id_get(
        &'a self,
        user_id: &'a str,
        client_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_clients_with_client_id_get(
                self.realm, user_id, client_id,
            )
    }

    /// Add client-level roles to the user or group role mapping
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn users_with_user_id_role_mappings_clients_with_client_id_post(
        &'a self,
        user_id: &'a str,
        client_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_clients_with_client_id_post(
                self.realm, user_id, client_id, body,
            )
    }

    /// Delete client-level roles from user or group role mapping
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn users_with_user_id_role_mappings_clients_with_client_id_delete(
        &'a self,
        user_id: &'a str,
        client_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_clients_with_client_id_delete(
                self.realm, user_id, client_id, body,
            )
    }

    /// Get available client-level roles that can be mapped to the user or group
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn users_with_user_id_role_mappings_clients_with_client_id_available_get(
        &'a self,
        user_id: &'a str,
        client_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_clients_with_client_id_available_get(
                self.realm, user_id, client_id,
            )
    }

    /// Get effective client-level role mappings This recurses any composite roles
    #[cfg(feature = "tag-client-role-mappings")]
    pub fn users_with_user_id_role_mappings_clients_with_client_id_composite_get(
        &'a self,
        user_id: &'a str,
        client_id: &'a str,
    ) -> RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS> {
        RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet {
            realm_admin: self,
            user_id,
            client_id,
        }
    }

    // <h4>Client Scopes</h4>
    /// Get client scopes belonging to the realm Returns a list of client scopes belonging to the realm
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_scopes_get(self.realm)
    }

    /// Create a new client scope Client Scope’s name must be unique!
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_scopes_post(self.realm, body)
    }

    /// Get representation of the client scope
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_get(self.realm, client_scope_id)
    }

    /// Update the client scope
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_put(self.realm, client_scope_id, body)
    }

    /// Delete the client scope
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_delete(self.realm, client_scope_id)
    }

    /// Get client scopes belonging to the realm Returns a list of client scopes belonging to the realm
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_templates_get(self.realm)
    }

    /// Create a new client scope Client Scope’s name must be unique!
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_templates_post(self.realm, body)
    }

    /// Get representation of the client scope
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_get(self.realm, client_scope_id)
    }

    /// Update the client scope
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_templates_with_client_scope_id_put(
            self.realm,
            client_scope_id,
            body,
        )
    }

    /// Delete the client scope
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_delete(self.realm, client_scope_id)
    }

    // <h4>Clients</h4>
    /// Get clients belonging to the realm.
    ///
    /// If a client can’t be retrieved from the storage due to a problem with the underlying storage, it is silently removed from the returned list. This ensures that concurrent modifications to the list don’t prevent callers from retrieving this list.
    #[cfg(feature = "tag-clients")]
    pub fn clients_get(&'a self) -> RealmClientsGet<'a, TS> {
        RealmClientsGet { realm_admin: self }
    }

    /// Create a new client Client’s client_id must be unique!
    #[cfg(feature = "tag-clients")]
    pub fn clients_post(
        &'a self,
        body: ClientRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_clients_post(self.realm, body)
    }

    /// Get representation of the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_get(self.realm, client_uuid)
    }

    /// Update the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_put(
        &'a self,
        client_uuid: &'a str,
        body: ClientRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_put(self.realm, client_uuid, body)
    }

    /// Delete the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_delete(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_delete(self.realm, client_uuid)
    }

    /// Get the client secret
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_get(self.realm, client_uuid)
    }

    /// Generate a new secret for the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_post(self.realm, client_uuid)
    }

    /// Get the rotated client secret
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_rotated_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_rotated_get(self.realm, client_uuid)
    }

    /// Invalidate the rotated secret for the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_rotated_delete(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_rotated_delete(self.realm, client_uuid)
    }

    /// Get default client scopes.  Only name and ids are returned.
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_default_client_scopes_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_default_client_scopes_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_default_client_scopes_with_client_scope_id_put(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_default_client_scopes_with_client_scope_id_put(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_default_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_default_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    /// Create JSON with payload of example access token
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_evaluate_scopes_generate_example_access_token_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Create JSON with payload of example id token
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_evaluate_scopes_generate_example_id_token_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Create JSON with payload of example user info
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_evaluate_scopes_generate_example_userinfo_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Return list of all protocol mappers, which will be used when generating tokens issued for particular client.
    ///
    /// This means protocol mappers assigned to this client directly and protocol mappers assigned to all client scopes of this client.
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_evaluate_scopes_protocol_mappers_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Get effective scope mapping of all roles of particular role container, which this client is defacto allowed to have in the accessToken issued for him.
    ///
    /// This contains scope mappings, which this client has directly, as well as scope mappings, which are granted to all client scopes, which are linked with this client.
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
        &'a self,
        client_uuid: &'a str,
        role_container_id: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
    {
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet {
            realm_admin: self,
            client_uuid,
            role_container_id,
        }
    }

    /// Get roles, which this client doesn't have scope for and can't have them in the accessToken issued for him.
    ///
    /// Defacto all the other roles of particular role container, which are not in {@link #getGrantedScopeMappings()}
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
        &'a self,
        client_uuid: &'a str,
        role_container_id: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<'a, TS>
    {
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet {
            realm_admin: self,
            client_uuid,
            role_container_id,
        }
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_installation_providers_with_provider_id_get(
        &'a self,
        client_uuid: &'a str,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_installation_providers_with_provider_id_get(
                self.realm,
                client_uuid,
                provider_id,
            )
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_management_permissions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_management_permissions_get(self.realm, client_uuid)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_management_permissions_put(
        &'a self,
        client_uuid: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_management_permissions_put(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Register a cluster node with the client Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle by sending registration request to Keycloak
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_nodes_post(
        &'a self,
        client_uuid: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_nodes_post(self.realm, client_uuid, body)
    }

    /// Unregister a cluster node from the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_nodes_with_node_delete(
        &'a self,
        client_uuid: &'a str,
        node: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_nodes_with_node_delete(self.realm, client_uuid, node)
    }

    /// Get application offline session count Returns a number of offline user sessions associated with this client { "count": number }
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_offline_session_count_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, i64>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_offline_session_count_get(self.realm, client_uuid)
    }

    /// Get offline sessions for client Returns a list of offline user sessions associated with this client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_offline_sessions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidOfflineSessionsGet<'a, TS> {
        RealmClientsWithClientUuidOfflineSessionsGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Get optional client scopes.  Only name and ids are returned.
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_optional_client_scopes_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_optional_client_scopes_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_optional_client_scopes_with_client_scope_id_put(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_optional_client_scopes_with_client_scope_id_put(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_optional_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_optional_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    /// Push the client's revocation policy to its admin URL If the client has an admin URL, push revocation policy to it.
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_push_revocation_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_push_revocation_post(self.realm, client_uuid)
    }

    /// Generate a new registration access token for the client
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_registration_access_token_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_registration_access_token_post(self.realm, client_uuid)
    }

    /// Get a user dedicated to the service account
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_service_account_user_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<UserRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_service_account_user_get(self.realm, client_uuid)
    }

    /// Get application session count Returns a number of user sessions associated with this client { "count": number }
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_session_count_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, i64>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_session_count_get(self.realm, client_uuid)
    }

    /// Test if registered cluster nodes are available Tests availability by sending 'ping' request to all cluster nodes.
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_test_nodes_available_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_test_nodes_available_get(self.realm, client_uuid)
    }

    /// Get user sessions for client Returns a list of user sessions associated with this client
    ///
    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_user_sessions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidUserSessionsGet<'a, TS> {
        RealmClientsWithClientUuidUserSessionsGet {
            realm_admin: self,
            client_uuid,
        }
    }

    // <h4>Component</h4>
    #[cfg(feature = "tag-component")]
    pub fn components_get(&'a self) -> RealmComponentsGet<'a, TS> {
        RealmComponentsGet { realm_admin: self }
    }

    #[cfg(feature = "tag-component")]
    pub fn components_post(
        &'a self,
        body: ComponentRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_post(self.realm, body)
    }

    #[cfg(feature = "tag-component")]
    pub fn components_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<ComponentRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_with_id_get(self.realm, id)
    }

    #[cfg(feature = "tag-component")]
    pub fn components_with_id_put(
        &'a self,
        id: &'a str,
        body: ComponentRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_components_with_id_put(self.realm, id, body)
    }

    #[cfg(feature = "tag-component")]
    pub fn components_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_with_id_delete(self.realm, id)
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
    #[cfg(feature = "tag-component")]
    pub fn components_with_id_sub_component_types_get(
        &'a self,
        id: &'a str,
    ) -> RealmComponentsWithIdSubComponentTypesGet<'a, TS> {
        RealmComponentsWithIdSubComponentTypesGet {
            realm_admin: self,
            id,
        }
    }

    // <h4>Groups</h4>
    /// Get group hierarchy.  Only `name` and `id` are returned.  `subGroups` are only returned when using the `search` or `q` parameter. If none of these parameters is provided, the top-level groups are returned without `subGroups` being filled.
    #[cfg(feature = "tag-groups")]
    pub fn groups_get(&'a self) -> RealmGroupsGet<'a, TS> {
        RealmGroupsGet { realm_admin: self }
    }

    /// create or add a top level realm groupSet or create child.
    ///
    /// This will update the group and set the parent if it exists. Create it and set the parent if the group doesn’t exist.
    #[cfg(feature = "tag-groups")]
    pub fn groups_post(
        &'a self,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_groups_post(self.realm, body)
    }

    /// Returns the groups counts.
    #[cfg(feature = "tag-groups")]
    pub fn groups_count_get(&'a self) -> RealmGroupsCountGet<'a, TS> {
        RealmGroupsCountGet { realm_admin: self }
    }

    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<GroupRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_get(self.realm, group_id)
    }

    /// Update group, ignores subgroups.
    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_put(
        &'a self,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_put(self.realm, group_id, body)
    }

    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_delete(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_delete(self.realm, group_id)
    }

    /// Return a paginated list of subgroups that have a parent group corresponding to the group on the URL
    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_children_get(
        &'a self,
        group_id: &'a str,
    ) -> RealmGroupsWithGroupIdChildrenGet<'a, TS> {
        RealmGroupsWithGroupIdChildrenGet {
            realm_admin: self,
            group_id,
        }
    }

    /// Set or create child.
    ///
    /// This will just set the parent if it exists. Create it and set the parent if the group doesn’t exist.
    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_children_post(
        &'a self,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_children_post(self.realm, group_id, body)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_management_permissions_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_management_permissions_get(self.realm, group_id)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_management_permissions_put(
        &'a self,
        group_id: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_management_permissions_put(self.realm, group_id, body)
    }

    /// Get users Returns a stream of users, filtered according to query parameters
    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_members_get(
        &'a self,
        group_id: &'a str,
    ) -> RealmGroupsWithGroupIdMembersGet<'a, TS> {
        RealmGroupsWithGroupIdMembersGet {
            realm_admin: self,
            group_id,
        }
    }

    // <h4>Identity Providers</h4>
    /// Import identity provider from JSON body
    ///
    /// Import identity provider from uploaded JSON file
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_import_config_post(
        &'a self,
        body: TypeMap<String, Value>,
    ) -> impl Future<Output = Result<TypeMap<String, TypeString>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_import_config_post(self.realm, body)
    }

    /// List identity providers
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_get(&'a self) -> RealmIdentityProviderInstancesGet<'a, TS> {
        RealmIdentityProviderInstancesGet { realm_admin: self }
    }

    /// Create a new identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_post(
        &'a self,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_post(self.realm, body)
    }

    /// Get the identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_get(self.realm, alias)
    }

    /// Update the identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_put(
        &'a self,
        alias: &'a str,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_put(self.realm, alias, body)
    }

    /// Delete the identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_delete(self.realm, alias)
    }

    /// Export public broker configuration for identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_export_get(
        &'a self,
        alias: &'a str,
    ) -> RealmIdentityProviderInstancesWithAliasExportGet<'a, TS> {
        RealmIdentityProviderInstancesWithAliasExportGet {
            realm_admin: self,
            alias,
        }
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_management_permissions_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_management_permissions_get(
                self.realm, alias,
            )
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_management_permissions_put(
        &'a self,
        alias: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_management_permissions_put(
                self.realm, alias, body,
            )
    }

    /// Get mapper types for identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mapper_types_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<
        Output = Result<TypeMap<String, IdentityProviderMapperTypeRepresentation>, KeycloakError>,
    > + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mapper_types_get(self.realm, alias)
    }

    /// Get mappers for identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_get(self.realm, alias)
    }

    /// Add a mapper to identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_post(
        &'a self,
        alias: &'a str,
        body: IdentityProviderMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_post(self.realm, alias, body)
    }

    /// Get mapper by id for the identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_with_id_get(
        &'a self,
        alias: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_with_id_get(self.realm, alias, id)
    }

    /// Update a mapper for the identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_with_id_put(
        &'a self,
        alias: &'a str,
        id: &'a str,
        body: IdentityProviderMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_with_id_put(
                self.realm, alias, id, body,
            )
    }

    /// Delete a mapper for the identity provider
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_with_id_delete(
        &'a self,
        alias: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_with_id_delete(
                self.realm, alias, id,
            )
    }

    /// Reaload keys for the identity provider if the provider supports it, "true" is returned if reload was performed, "false" if not.
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_reload_keys_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<bool, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_reload_keys_get(self.realm, alias)
    }

    /// Get the identity provider factory for that provider id
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_providers_with_provider_id_get(
        &'a self,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_providers_with_provider_id_get(self.realm, provider_id)
    }

    // <h4>Key</h4>
    #[cfg(feature = "tag-key")]
    pub fn keys_get(
        &'a self,
    ) -> impl Future<Output = Result<KeysMetadataRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_keys_get(self.realm)
    }

    // <h4>Organizations</h4>
    /// Returns a paginated list of organizations filtered according to the specified parameters
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_get(&'a self) -> RealmOrganizationsGet<'a, TS> {
        RealmOrganizationsGet { realm_admin: self }
    }

    /// Creates a new organization
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_post(
        &'a self,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_organizations_post(self.realm, body)
    }

    /// Returns the organizations counts.
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_count_get(&'a self) -> RealmOrganizationsCountGet<'a, TS> {
        RealmOrganizationsCountGet { realm_admin: self }
    }

    /// Returns the organizations associated with the user that has the specified id
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_members_with_member_id_organizations_get(
        &'a self,
        member_id: &'a str,
    ) -> RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS> {
        RealmOrganizationsMembersWithMemberIdOrganizationsGet {
            realm_admin: self,
            member_id,
        }
    }

    /// Returns the organization representation
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<OrganizationRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_get(self.realm, org_id)
    }

    /// Updates the organization
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_put(
        &'a self,
        org_id: &'a str,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_put(self.realm, org_id, body)
    }

    /// Deletes the organization
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_delete(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_delete(self.realm, org_id)
    }

    /// Returns all identity providers associated with the organization
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_identity_providers_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_get(self.realm, org_id)
    }

    /// Adds the identity provider with the specified id to the organization
    ///
    /// Adds, or associates, an existing identity provider with the organization. If no identity provider is found, or if it is already associated with the organization, an error response is returned
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_identity_providers_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_post(self.realm, org_id, body)
    }

    /// Returns the identity provider associated with the organization that has the specified alias
    ///
    /// Searches for an identity provider with the given alias. If one is found and is associated with the organization, it is returned. Otherwise, an error response with status NOT_FOUND is returned
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_identity_providers_with_alias_get(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_with_alias_get(
                self.realm, org_id, alias,
            )
    }

    /// Removes the identity provider with the specified alias from the organization
    ///
    /// Breaks the association between the identity provider and the organization. The provider itself is not deleted. If no provider is found, or if it is not currently associated with the org, an error response is returned
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_identity_providers_with_alias_delete(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_with_alias_delete(
                self.realm, org_id, alias,
            )
    }

    /// Returns a paginated list of organization members filtered according to the specified parameters
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_get(
        &'a self,
        org_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersGet {
            realm_admin: self,
            org_id,
        }
    }

    /// Adds the user with the specified id as a member of the organization
    ///
    /// Adds, or associates, an existing user with the organization. If no user is found, or if it is already associated with the organization, an error response is returned
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_post(self.realm, org_id, body)
    }

    /// Returns number of members in the organization.
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_count_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<i64, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_count_get(self.realm, org_id)
    }

    /// Invites an existing user to the organization, using the specified user id
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_invite_existing_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_existing_user_post(
                self.realm, org_id, body,
            )
    }

    /// Invites an existing user or sends a registration link to a new user, based on the provided e-mail address.
    ///
    /// If the user with the given e-mail address exists, it sends an invitation link, otherwise it sends a registration link.
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_invite_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_user_post(self.realm, org_id, body)
    }

    /// Returns the member of the organization with the specified id
    ///
    /// Searches for auser with the given id. If one is found, and is currently a member of the organization, returns it. Otherwise,an error response with status NOT_FOUND is returned
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_with_member_id_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> impl Future<Output = Result<MemberRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_with_member_id_get(
                self.realm, org_id, member_id,
            )
    }

    /// Removes the user with the specified id from the organization
    ///
    /// Breaks the association between the user and organization. The user itself is deleted in case the membership is managed, otherwise the user is not deleted. If no user is found, or if they are not a member of the organization, an error response is returned
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_with_member_id_delete(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_with_member_id_delete(
                self.realm, org_id, member_id,
            )
    }

    /// Returns the organizations associated with the user that has the specified id
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_with_member_id_organizations_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet {
            realm_admin: self,
            org_id,
            member_id,
        }
    }

    // <h4>Protocol Mappers</h4>
    /// Create multiple mappers
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_add_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_add_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mappers
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Create a mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mapper by id
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_with_id_get(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<ProtocolMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_get(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Update the mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_with_id_put(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_put(
                self.realm,
                client_scope_id,
                id,
                body,
            )
    }

    /// Delete the mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_with_id_delete(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_delete(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Get mappers by name for a specific protocol
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_scopes_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
        &'a self,
        client_scope_id: &'a str,
        protocol: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
                self.realm,
                client_scope_id,
                protocol,
            )
    }

    /// Create multiple mappers
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_add_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_add_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mappers
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Create a mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mapper by id
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_with_id_get(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<ProtocolMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_get(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Update the mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_with_id_put(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_put(
                self.realm,
                client_scope_id,
                id,
                body,
            )
    }

    /// Delete the mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_with_id_delete(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_delete(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Get mappers by name for a specific protocol
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn client_templates_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
        &'a self,
        client_scope_id: &'a str,
        protocol: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
                self.realm,
                client_scope_id,
                protocol,
            )
    }

    /// Create multiple mappers
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_add_models_post(
        &'a self,
        client_uuid: &'a str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_add_models_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Get mappers
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_models_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_get(self.realm, client_uuid)
    }

    /// Create a mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_models_post(
        &'a self,
        client_uuid: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Get mapper by id
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_models_with_id_get(
        &'a self,
        client_uuid: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<ProtocolMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_with_id_get(
                self.realm,
                client_uuid,
                id,
            )
    }

    /// Update the mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_models_with_id_put(
        &'a self,
        client_uuid: &'a str,
        id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_with_id_put(
                self.realm,
                client_uuid,
                id,
                body,
            )
    }

    /// Delete the mapper
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_models_with_id_delete(
        &'a self,
        client_uuid: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_with_id_delete(
                self.realm,
                client_uuid,
                id,
            )
    }

    /// Get mappers by name for a specific protocol
    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_protocol_with_protocol_get(
        &'a self,
        client_uuid: &'a str,
        protocol: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_protocol_with_protocol_get(
                self.realm,
                client_uuid,
                protocol,
            )
    }

    // <h4>Realms Admin</h4>
    /// Get the top-level representation of the realm It will not include nested information like User and Client representations.
    #[cfg(feature = "tag-realms-admin")]
    pub fn get(
        &'a self,
    ) -> impl Future<Output = Result<RealmRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_get(self.realm)
    }

    /// Update the top-level information of the realm Any user, roles or client information in the representation will be ignored.
    ///
    /// This will only update top-level attributes of the realm.
    #[cfg(feature = "tag-realms-admin")]
    pub fn put(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_put(self.realm, body)
    }

    /// Delete the realm
    #[cfg(feature = "tag-realms-admin")]
    pub fn delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_delete(self.realm)
    }

    /// Get admin events Returns all admin events, or filters events based on URL query parameters listed here
    #[cfg(feature = "tag-realms-admin")]
    pub fn admin_events_get(&'a self) -> RealmAdminEventsGet<'a, TS> {
        RealmAdminEventsGet { realm_admin: self }
    }

    /// Delete all admin events
    #[cfg(feature = "tag-realms-admin")]
    pub fn admin_events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_admin_events_delete(self.realm)
    }

    /// Base path for importing clients under this realm.
    #[cfg(feature = "tag-realms-admin")]
    pub fn client_description_converter_post(
        &'a self,
        body: String,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_description_converter_post(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn client_policies_policies_get(&'a self) -> RealmClientPoliciesPoliciesGet<'a, TS> {
        RealmClientPoliciesPoliciesGet { realm_admin: self }
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn client_policies_policies_put(
        &'a self,
        body: ClientPoliciesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_policies_policies_put(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn client_policies_profiles_get(&'a self) -> RealmClientPoliciesProfilesGet<'a, TS> {
        RealmClientPoliciesProfilesGet { realm_admin: self }
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn client_policies_profiles_put(
        &'a self,
        body: ClientProfilesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_policies_profiles_put(self.realm, body)
    }

    /// Get client session stats Returns a JSON map.
    ///
    /// The key is the client id, the value is the number of sessions that currently are active with that client. Only clients that actually have a session associated with them will be in this map.
    #[cfg(feature = "tag-realms-admin")]
    pub fn client_session_stats_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, String>>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_session_stats_get(self.realm)
    }

    /// List all client types available in the current realm
    ///
    /// This endpoint returns a list of both global and realm level client types and the attributes they set
    #[cfg(feature = "tag-realms-admin")]
    pub fn client_types_get(
        &'a self,
    ) -> impl Future<Output = Result<ClientTypesRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_get(self.realm)
    }

    /// Update a client type
    ///
    /// This endpoint allows you to update a realm level client type
    #[cfg(feature = "tag-realms-admin")]
    pub fn client_types_put(
        &'a self,
        body: ClientTypesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_put(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn credential_registrators_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin.realm_credential_registrators_get(self.realm)
    }

    /// Get realm default client scopes. Only name and ids are returned.
    #[cfg(feature = "tag-realms-admin")]
    pub fn default_default_client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_default_default_client_scopes_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn default_default_client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_default_client_scopes_with_client_scope_id_put(
                self.realm,
                client_scope_id,
            )
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn default_default_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_default_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_scope_id,
            )
    }

    /// Get group hierarchy.  Only name and ids are returned.
    #[cfg(feature = "tag-realms-admin")]
    pub fn default_groups_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<GroupRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_default_groups_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn default_groups_with_group_id_put(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_groups_with_group_id_put(self.realm, group_id)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn default_groups_with_group_id_delete(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_groups_with_group_id_delete(self.realm, group_id)
    }

    /// Get realm optional client scopes. Only name and ids are returned.
    #[cfg(feature = "tag-realms-admin")]
    pub fn default_optional_client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_default_optional_client_scopes_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn default_optional_client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_optional_client_scopes_with_client_scope_id_put(
                self.realm,
                client_scope_id,
            )
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn default_optional_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_optional_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_scope_id,
            )
    }

    /// Get events Returns all events, or filters them based on URL query parameters listed here
    #[cfg(feature = "tag-realms-admin")]
    pub fn events_get(&'a self) -> RealmEventsGet<'a, TS> {
        RealmEventsGet { realm_admin: self }
    }

    /// Delete all events
    #[cfg(feature = "tag-realms-admin")]
    pub fn events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_delete(self.realm)
    }

    /// Get the events provider configuration Returns JSON object with events provider configuration
    #[cfg(feature = "tag-realms-admin")]
    pub fn events_config_get(
        &'a self,
    ) -> impl Future<Output = Result<RealmEventsConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_events_config_get(self.realm)
    }

    /// Update the events provider Change the events provider and/or its configuration
    #[cfg(feature = "tag-realms-admin")]
    pub fn events_config_put(
        &'a self,
        body: RealmEventsConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_config_put(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn group_by_path_with_path_get(
        &'a self,
        path: &'a str,
    ) -> impl Future<Output = Result<GroupRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_group_by_path_with_path_get(self.realm, path)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin.realm_localization_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_with_locale_get(
        &'a self,
        locale: &'a str,
    ) -> RealmLocalizationWithLocaleGet<'a, TS> {
        RealmLocalizationWithLocaleGet {
            realm_admin: self,
            locale,
        }
    }

    /// Import localization from uploaded JSON file
    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_with_locale_post(
        &'a self,
        locale: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_post(self.realm, locale, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_with_locale_delete(
        &'a self,
        locale: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_delete(self.realm, locale)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_with_locale_with_key_get(
        &'a self,
        key: &'a str,
        locale: &'a str,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_get(self.realm, key, locale)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_with_locale_with_key_put(
        &'a self,
        key: &'a str,
        locale: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_put(self.realm, key, locale, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn localization_with_locale_with_key_delete(
        &'a self,
        key: &'a str,
        locale: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_delete(self.realm, key, locale)
    }

    /// Removes all user sessions.
    ///
    /// Any client that has an admin url will also be told to invalidate any sessions they have.
    #[cfg(feature = "tag-realms-admin")]
    pub fn logout_all_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_logout_all_post(self.realm)
    }

    /// Partial export of existing realm into a JSON file.
    #[cfg(feature = "tag-realms-admin")]
    pub fn partial_export_post(&'a self) -> RealmPartialExportPost<'a, TS> {
        RealmPartialExportPost { realm_admin: self }
    }

    /// Partial import from a JSON file to an existing realm.
    #[cfg(feature = "tag-realms-admin")]
    pub fn partial_import_post(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<Value, KeycloakError>> + use<'a, TS> {
        self.admin.realm_partial_import_post(self.realm, body)
    }

    /// Push the realm's revocation policy to any client that has an admin url associated with it.
    #[cfg(feature = "tag-realms-admin")]
    pub fn push_revocation_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_push_revocation_post(self.realm)
    }

    /// Remove a specific user session.
    ///
    /// Any client that has an admin url will also be told to invalidate this particular session.
    #[cfg(feature = "tag-realms-admin")]
    pub fn sessions_with_session_delete(
        &'a self,
        session: &'a str,
    ) -> RealmSessionsWithSessionDelete<'a, TS> {
        RealmSessionsWithSessionDelete {
            realm_admin: self,
            session,
        }
    }

    /// Test SMTP connection with current logged in user
    #[cfg(feature = "tag-realms-admin")]
    #[deprecated]
    pub fn test_smtp_connection_post(
        &'a self,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_test_smtp_connection_post(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn users_management_permissions_get(
        &'a self,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_management_permissions_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn users_management_permissions_put(
        &'a self,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_management_permissions_put(self.realm, body)
    }

    // <h4>Role Mapper</h4>
    /// Get role mappings
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_get(self.realm, group_id)
    }

    /// Get realm-level role mappings
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_get(self.realm, group_id)
    }

    /// Add realm-level role mappings to the user
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_post(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_post(self.realm, group_id, body)
    }

    /// Delete realm-level role mappings
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_delete(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_delete(self.realm, group_id, body)
    }

    /// Get realm-level roles that can be mapped
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_available_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_available_get(self.realm, group_id)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_composite_get(
        &'a self,
        group_id: &'a str,
    ) -> RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'a, TS> {
        RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet {
            realm_admin: self,
            group_id,
        }
    }

    /// Get role mappings
    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_get(self.realm, user_id)
    }

    /// Get realm-level role mappings
    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_get(self.realm, user_id)
    }

    /// Add realm-level role mappings to the user
    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_post(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_post(self.realm, user_id, body)
    }

    /// Delete realm-level role mappings
    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_delete(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_delete(self.realm, user_id, body)
    }

    /// Get realm-level roles that can be mapped
    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_available_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_available_get(self.realm, user_id)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_composite_get(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS> {
        RealmUsersWithUserIdRoleMappingsRealmCompositeGet {
            realm_admin: self,
            user_id,
        }
    }

    // <h4>Roles</h4>
    /// Get all roles for the realm or client
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidRolesGet<'a, TS> {
        RealmClientsWithClientUuidRolesGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Create a new role for the realm or client
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_post(
        &'a self,
        client_uuid: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_post(self.realm, client_uuid, body)
    }

    /// Get a role by name
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_get(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Update a role by name
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_put(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_put(
                self.realm,
                client_uuid,
                role_name,
                body,
            )
    }

    /// Delete a role by name
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_delete(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_delete(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Get composites of the role
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_composites_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_composites_get(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Add a composite to the role
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_composites_post(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_composites_post(
                self.realm,
                client_uuid,
                role_name,
                body,
            )
    }

    /// Remove roles from the role's composite
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_composites_delete(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_composites_delete(
                self.realm,
                client_uuid,
                role_name,
                body,
            )
    }

    /// Get client-level roles for the client that are in the role's composite
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_composites_clients_with_client_uuid_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_composites_clients_with_client_uuid_get(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Get realm-level roles of the role's composite
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_composites_realm_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_composites_realm_get(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Returns a stream of groups that have the specified role name
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_groups_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'a, TS> {
        RealmClientsWithClientUuidRolesWithRoleNameGroupsGet {
            realm_admin: self,
            client_uuid,
            role_name,
        }
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_management_permissions_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_management_permissions_get(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_management_permissions_put(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_roles_with_role_name_management_permissions_put(
                self.realm,
                client_uuid,
                role_name,
                body,
            )
    }

    /// Returns a stream of users that have the specified role name.
    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_with_role_name_users_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'a, TS> {
        RealmClientsWithClientUuidRolesWithRoleNameUsersGet {
            realm_admin: self,
            client_uuid,
            role_name,
        }
    }

    /// Get all roles for the realm or client
    #[cfg(feature = "tag-roles")]
    pub fn roles_get(&'a self) -> RealmRolesGet<'a, TS> {
        RealmRolesGet { realm_admin: self }
    }

    /// Create a new role for the realm or client
    #[cfg(feature = "tag-roles")]
    pub fn roles_post(
        &'a self,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_roles_post(self.realm, body)
    }

    /// Get a role by name
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_get(self.realm, role_name)
    }

    /// Update a role by name
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_put(
        &'a self,
        role_name: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_put(self.realm, role_name, body)
    }

    /// Delete a role by name
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_delete(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_delete(self.realm, role_name)
    }

    /// Get composites of the role
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_get(self.realm, role_name)
    }

    /// Add a composite to the role
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_post(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_post(self.realm, role_name, body)
    }

    /// Remove roles from the role's composite
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_delete(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_delete(self.realm, role_name, body)
    }

    /// Get client-level roles for the client that are in the role's composite
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_clients_with_client_uuid_get(
        &'a self,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_clients_with_client_uuid_get(
                self.realm,
                client_uuid,
                role_name,
            )
    }

    /// Get realm-level roles of the role's composite
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_realm_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_realm_get(self.realm, role_name)
    }

    /// Returns a stream of groups that have the specified role name
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_groups_get(
        &'a self,
        role_name: &'a str,
    ) -> RealmRolesWithRoleNameGroupsGet<'a, TS> {
        RealmRolesWithRoleNameGroupsGet {
            realm_admin: self,
            role_name,
        }
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_management_permissions_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_management_permissions_get(self.realm, role_name)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_management_permissions_put(
        &'a self,
        role_name: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_management_permissions_put(self.realm, role_name, body)
    }

    /// Returns a stream of users that have the specified role name.
    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_users_get(
        &'a self,
        role_name: &'a str,
    ) -> RealmRolesWithRoleNameUsersGet<'a, TS> {
        RealmRolesWithRoleNameUsersGet {
            realm_admin: self,
            role_name,
        }
    }

    // <h4>Roles (by ID)</h4>
    /// Get a specific role's representation
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_get(self.realm, role_id)
    }

    /// Update the role
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_put(
        &'a self,
        role_id: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_put(self.realm, role_id, body)
    }

    /// Delete the role
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_delete(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_delete(self.realm, role_id)
    }

    /// Get role's children Returns a set of role's children provided the role is a composite.
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_get(
        &'a self,
        role_id: &'a str,
    ) -> RealmRolesByIdWithRoleIdCompositesGet<'a, TS> {
        RealmRolesByIdWithRoleIdCompositesGet {
            realm_admin: self,
            role_id,
        }
    }

    /// Make the role a composite role by associating some child roles
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_post(
        &'a self,
        role_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_composites_post(self.realm, role_id, body)
    }

    /// Remove a set of roles from the role's composite
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_delete(
        &'a self,
        role_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_composites_delete(self.realm, role_id, body)
    }

    /// Get client-level roles for the client that are in the role's composite
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
        &'a self,
        client_uuid: &'a str,
        role_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
                self.realm,
                client_uuid,
                role_id,
            )
    }

    /// Get realm-level roles that are in the role's composite
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_realm_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_composites_realm_get(self.realm, role_id)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_management_permissions_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_management_permissions_get(self.realm, role_id)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_management_permissions_put(
        &'a self,
        role_id: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_management_permissions_put(self.realm, role_id, body)
    }

    // <h4>Scope Mappings</h4>
    /// Get all scope mappings for the client
    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub fn client_scopes_with_client_scope_id_scope_mappings_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Add client-level roles to the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_post(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_post(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// Remove client-level roles from the client's scope.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_delete(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_delete(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_available_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_available_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS> {
        RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet {
            realm_admin: self,
            client_scope_id,
            client,
        }
    }

    /// Get realm-level roles associated with the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Add a set of realm-level roles to the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Remove a set of realm-level roles from the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_delete(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_delete(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get realm-level roles that are available to attach to this client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_available_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_available_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really to show a comprehensive total view of realm-level roles associated with the client.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_composite_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS> {
        RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet {
            realm_admin: self,
            client_scope_id,
        }
    }

    /// Get all scope mappings for the client
    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub fn client_templates_with_client_scope_id_scope_mappings_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Add client-level roles to the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_post(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_post(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// Remove client-level roles from the client's scope.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_delete(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_delete(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_available_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_available_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
    {
        RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet {
            realm_admin: self,
            client_scope_id,
            client,
        }
    }

    /// Get realm-level roles associated with the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Add a set of realm-level roles to the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Remove a set of realm-level roles from the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_delete(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_delete(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get realm-level roles that are available to attach to this client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_available_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_available_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really to show a comprehensive total view of realm-level roles associated with the client.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_composite_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS> {
        RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet {
            realm_admin: self,
            client_scope_id,
        }
    }

    /// Get all scope mappings for the client
    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub fn clients_with_client_uuid_scope_mappings_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_get(self.realm, client_uuid)
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_get(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_get(
                self.realm,
                client_uuid,
                client,
            )
    }

    /// Add client-level roles to the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_post(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_post(
                self.realm,
                client_uuid,
                client,
                body,
            )
    }

    /// Remove client-level roles from the client's scope.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_delete(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_delete(
                self.realm,
                client_uuid,
                client,
                body,
            )
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_available_get(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_available_get(
                self.realm,
                client_uuid,
                client,
            )
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_composite_get(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
    ) -> RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS> {
        RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet {
            realm_admin: self,
            client_uuid,
            client,
        }
    }

    /// Get realm-level roles associated with the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_get(self.realm, client_uuid)
    }

    /// Add a set of realm-level roles to the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_post(
        &'a self,
        client_uuid: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_post(self.realm, client_uuid, body)
    }

    /// Remove a set of realm-level roles from the client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_delete(
        &'a self,
        client_uuid: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_delete(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Get realm-level roles that are available to attach to this client's scope
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_available_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_available_get(
                self.realm,
                client_uuid,
            )
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really to show a comprehensive total view of realm-level roles associated with the client.
    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_composite_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS> {
        RealmClientsWithClientUuidScopeMappingsRealmCompositeGet {
            realm_admin: self,
            client_uuid,
        }
    }

    // <h4>Users</h4>
    /// Get users Returns a stream of users, filtered according to query parameters.
    #[cfg(feature = "tag-users")]
    pub fn users_get(&'a self) -> RealmUsersGet<'a, TS> {
        RealmUsersGet { realm_admin: self }
    }

    /// Create a new user Username must be unique.
    #[cfg(feature = "tag-users")]
    pub fn users_post(
        &'a self,
        body: UserRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_post(self.realm, body)
    }

    /// Returns the number of users that match the given criteria.
    ///
    /// It can be called in three different ways. 1. Don’t specify any criteria and pass {@code null}. The number of all users within that realm will be returned. <p> 2. If {@code search} is specified other criteria such as {@code last} will be ignored even though you set them. The {@code search} string will be matched against the first and last name, the username and the email of a user. <p> 3. If {@code search} is unspecified but any of {@code last}, {@code first}, {@code email} or {@code username} those criteria are matched against their respective fields on a user entity. Combined with a logical and.
    #[cfg(feature = "tag-users")]
    pub fn users_count_get(&'a self) -> RealmUsersCountGet<'a, TS> {
        RealmUsersCountGet { realm_admin: self }
    }

    /// Get the configuration for the user profile
    #[cfg(feature = "tag-users")]
    pub fn users_profile_get(
        &'a self,
    ) -> impl Future<Output = Result<UPConfig, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_get(self.realm)
    }

    /// Set the configuration for the user profile
    #[cfg(feature = "tag-users")]
    pub fn users_profile_put(
        &'a self,
        body: UPConfig,
    ) -> impl Future<Output = Result<UPConfig, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_put(self.realm, body)
    }

    /// Get the UserProfileMetadata from the configuration
    #[cfg(feature = "tag-users")]
    pub fn users_profile_metadata_get(
        &'a self,
    ) -> impl Future<Output = Result<UserProfileMetadata, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_metadata_get(self.realm)
    }

    /// Get representation of the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_get(&'a self, user_id: &'a str) -> RealmUsersWithUserIdGet<'a, TS> {
        RealmUsersWithUserIdGet {
            realm_admin: self,
            user_id,
        }
    }

    /// Update the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_put(
        &'a self,
        user_id: &'a str,
        body: UserRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_put(self.realm, user_id, body)
    }

    /// Delete the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_delete(self.realm, user_id)
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    ///
    /// Returned values can contain for example "password", "otp" etc. This will always return empty list for "local" users, which are not backed by any user storage
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_configured_user_storage_credential_types_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_configured_user_storage_credential_types_get(
                self.realm, user_id,
            )
    }

    /// Get consents granted by the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_consents_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_consents_get(self.realm, user_id)
    }

    /// Revoke consent and offline tokens for particular client from user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_consents_with_client_delete(
        &'a self,
        user_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_consents_with_client_delete(self.realm, user_id, client)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_credentials_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<CredentialRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_credentials_get(self.realm, user_id)
    }

    /// Remove a credential for a user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_credentials_with_credential_id_delete(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_delete(
                self.realm,
                user_id,
                credential_id,
            )
    }

    /// Move a credential to a position behind another credential
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
        new_previous_credential_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
                self.realm,
                user_id,
                credential_id,
                new_previous_credential_id,
            )
    }

    /// Move a credential to a first position in the credentials list of the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_credentials_with_credential_id_move_to_first_post(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_move_to_first_post(
                self.realm,
                user_id,
                credential_id,
            )
    }

    /// Update a credential label for a user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_credentials_with_credential_id_user_label_put(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_user_label_put(
                self.realm,
                user_id,
                credential_id,
                body,
            )
    }

    /// Disable all credentials for a user of a specific type
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_disable_credential_types_put(
        &'a self,
        user_id: &'a str,
        body: Vec<String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_disable_credential_types_put(self.realm, user_id, body)
    }

    /// Send an email to the user with a link they can click to execute particular actions.
    ///
    /// An email contains a link the user can click to perform a set of required actions. The redirectUri and clientId parameters are optional. If no redirect is given, then there will be no link back to click after actions have completed. Redirect uri must be a valid uri for the particular clientId.
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_execute_actions_email_put(
        &'a self,
        user_id: &'a str,
        body: Vec<String>,
    ) -> RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS> {
        RealmUsersWithUserIdExecuteActionsEmailPut {
            realm_admin: self,
            user_id,
            body,
        }
    }

    /// Get social logins associated with the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_federated_identity_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<FederatedIdentityRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_get(self.realm, user_id)
    }

    /// Add a social login provider to the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_federated_identity_with_provider_post(
        &'a self,
        user_id: &'a str,
        provider: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_with_provider_post(
                self.realm, user_id, provider,
            )
    }

    /// Remove a social login provider from user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_federated_identity_with_provider_delete(
        &'a self,
        user_id: &'a str,
        provider: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_with_provider_delete(
                self.realm, user_id, provider,
            )
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_groups_get(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdGroupsGet<'a, TS> {
        RealmUsersWithUserIdGroupsGet {
            realm_admin: self,
            user_id,
        }
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_groups_count_get(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdGroupsCountGet<'a, TS> {
        RealmUsersWithUserIdGroupsCountGet {
            realm_admin: self,
            user_id,
        }
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_groups_with_group_id_put(
        &'a self,
        user_id: &'a str,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_groups_with_group_id_put(self.realm, user_id, group_id)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_groups_with_group_id_delete(
        &'a self,
        user_id: &'a str,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_groups_with_group_id_delete(self.realm, user_id, group_id)
    }

    /// Impersonate the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_impersonation_post(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_impersonation_post(self.realm, user_id)
    }

    /// Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_logout_post(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_logout_post(self.realm, user_id)
    }

    /// Get offline sessions associated with the user and client
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_offline_sessions_with_client_uuid_get(
        &'a self,
        user_id: &'a str,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_offline_sessions_with_client_uuid_get(
                self.realm,
                user_id,
                client_uuid,
            )
    }

    /// Set up a new password for the user.
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_reset_password_put(
        &'a self,
        user_id: &'a str,
        body: CredentialRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_reset_password_put(self.realm, user_id, body)
    }

    /// Send an email to the user with a link they can click to reset their password.
    ///
    /// The redirectUri and clientId parameters are optional. The default for the redirect is the account client. This endpoint has been deprecated.  Please use the execute-actions-email passing a list with UPDATE_PASSWORD within it.
    #[cfg(feature = "tag-users")]
    #[deprecated]
    pub fn users_with_user_id_reset_password_email_put(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdResetPasswordEmailPut<'a, TS> {
        RealmUsersWithUserIdResetPasswordEmailPut {
            realm_admin: self,
            user_id,
        }
    }

    /// Send an email-verification email to the user An email contains a link the user can click to verify their email address.
    ///
    /// The redirectUri, clientId and lifespan parameters are optional. The default for the redirect is the account client. The default for the lifespan is 12 hours
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_send_verify_email_put(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdSendVerifyEmailPut<'a, TS> {
        RealmUsersWithUserIdSendVerifyEmailPut {
            realm_admin: self,
            user_id,
        }
    }

    /// Get sessions associated with the user
    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_sessions_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_sessions_get(self.realm, user_id)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_unmanaged_attributes_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, TypeVec<String>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_unmanaged_attributes_get(self.realm, user_id)
    }

    // <h4>default</h4>
    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ResourceServerRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_put(
        &'a self,
        client_uuid: &'a str,
        body: ResourceServerRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_put(self.realm, client_uuid, body)
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_import_post(
        &'a self,
        client_uuid: &'a str,
        body: ResourceServerRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_import_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_permission_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPermissionGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_permission_post(
        &'a self,
        client_uuid: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_permission_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_permission_evaluate_post(
        &'a self,
        client_uuid: &'a str,
        body: PolicyEvaluationRequest,
    ) -> impl Future<Output = Result<PolicyEvaluationResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_permission_evaluate_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_permission_providers_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<PolicyProviderRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_permission_providers_get(
                self.realm,
                client_uuid,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_permission_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_policy_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPolicyGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_policy_post(
        &'a self,
        client_uuid: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_policy_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_policy_evaluate_post(
        &'a self,
        client_uuid: &'a str,
        body: PolicyEvaluationRequest,
    ) -> impl Future<Output = Result<PolicyEvaluationResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_policy_evaluate_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_policy_providers_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<PolicyProviderRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_policy_providers_get(
                self.realm,
                client_uuid,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_policy_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_post(
        &'a self,
        client_uuid: &'a str,
        body: ResourceRepresentation,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourcePost<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourcePost {
            realm_admin: self,
            client_uuid,
            body,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_with_resource_id_get(
        &'a self,
        client_uuid: &'a str,
        resource_id: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet {
            realm_admin: self,
            client_uuid,
            resource_id,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_with_resource_id_put(
        &'a self,
        client_uuid: &'a str,
        resource_id: &'a str,
        body: ResourceRepresentation,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut {
            realm_admin: self,
            client_uuid,
            resource_id,
            body,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_with_resource_id_delete(
        &'a self,
        client_uuid: &'a str,
        resource_id: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete {
            realm_admin: self,
            client_uuid,
            resource_id,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_with_resource_id_attributes_get(
        &'a self,
        client_uuid: &'a str,
        resource_id: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet<'a, TS>
    {
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet {
            realm_admin: self,
            client_uuid,
            resource_id,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_with_resource_id_permissions_get(
        &'a self,
        client_uuid: &'a str,
        resource_id: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet<'a, TS>
    {
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet {
            realm_admin: self,
            client_uuid,
            resource_id,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_resource_with_resource_id_scopes_get(
        &'a self,
        client_uuid: &'a str,
        resource_id: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet {
            realm_admin: self,
            client_uuid,
            resource_id,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerScopeGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_post(
        &'a self,
        client_uuid: &'a str,
        body: ScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_with_scope_id_get(
        &'a self,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> impl Future<Output = Result<ScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_get(
                self.realm,
                client_uuid,
                scope_id,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_with_scope_id_put(
        &'a self,
        client_uuid: &'a str,
        scope_id: &'a str,
        body: ScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_put(
                self.realm,
                client_uuid,
                scope_id,
                body,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_with_scope_id_delete(
        &'a self,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_delete(
                self.realm,
                client_uuid,
                scope_id,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_with_scope_id_permissions_get(
        &'a self,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<PolicyRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_permissions_get(
                self.realm,
                client_uuid,
                scope_id,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_scope_with_scope_id_resources_get(
        &'a self,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ResourceRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_resources_get(
                self.realm,
                client_uuid,
                scope_id,
            )
    }

    #[cfg(feature = "tag-none")]
    pub fn clients_with_client_uuid_authz_resource_server_settings_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ResourceServerRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_settings_get(
                self.realm,
                client_uuid,
            )
    }
}

// <h4>Attack Detection</h4>

// <h4>Authentication Management</h4>

// <h4>Client Attribute Certificate</h4>

// <h4>Client Initial Access</h4>

// <h4>Client Registration Policy</h4>

// <h4>Client Role Mappings</h4>
#[cfg(feature = "tag-client-role-mappings")]
pub struct RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
    /// client id (not clientId!)
    pub client_id: &'a str,
}

#[cfg(feature = "tag-client-role-mappings")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-client-role-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_with_group_id_role_mappings_clients_with_client_id_composite_get(
                self.realm_admin.realm,
                self.group_id,
                self.client_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-client-role-mappings")]
impl<'a, TS> IntoFuture
    for RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-client-role-mappings")]
pub struct RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
    /// client id (not clientId!)
    pub client_id: &'a str,
}

#[cfg(feature = "tag-client-role-mappings")]
#[derive(Default)]
pub struct RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-client-role-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_role_mappings_clients_with_client_id_composite_get(
                self.realm_admin.realm,
                self.user_id,
                self.client_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-client-role-mappings")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Client Scopes</h4>

// <h4>Clients</h4>
#[cfg(feature = "tag-clients")]
pub struct RealmClientsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsGetArgs {
    /// filter by clientId
    pub client_id: Option<String>,
    /// the first result
    pub first: Option<i32>,
    /// the max results to return
    pub max: Option<i32>,
    pub q: Option<String>,
    /// whether this is a search query or a getClientById query
    pub search: Option<bool>,
    /// filter clients that cannot be viewed in full by admin
    pub viewable_only: Option<bool>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmClientsGet<'a, TS> {
    type Output = TypeVec<ClientRepresentation>;
    type Args = RealmClientsGetArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            first,
            max,
            q,
            search,
            viewable_only,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_clients_get(
            self.realm_admin.realm,
            client_id,
            first,
            max,
            q,
            search,
            viewable_only,
        )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture for RealmClientsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ClientRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGetArgs {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub user_id: Option<String>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS>
{
    type Output = AccessToken;
    type Args = RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGetArgs;

    fn opts(
        self,
        Self::Args {
            audience,
            scope,
            user_id,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_generate_example_access_token_get(
                self.realm_admin.realm,
                self.client_uuid,
                audience,
                scope,
                user_id,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<AccessToken, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGetArgs {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub user_id: Option<String>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS>
{
    type Output = IDToken;
    type Args = RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGetArgs;

    fn opts(
        self,
        Self::Args {
            audience,
            scope,
            user_id,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_generate_example_id_token_get(
                self.realm_admin.realm,
                self.client_uuid,
                audience,
                scope,
                user_id,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<IDToken, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGetArgs {
    pub scope: Option<String>,
    pub user_id: Option<String>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS>
{
    type Output = Value;
    type Args = RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGetArgs;

    fn opts(
        self,
        Self::Args { scope, user_id }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_generate_example_userinfo_get(
                self.realm_admin.realm,
                self.client_uuid,
                scope,
                user_id,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<Value, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS: KeycloakTokenSupplier>
{
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesProtocolMappersGetArgs {
    pub scope: Option<String>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS>
{
    type Output = TypeVec<ProtocolMapperEvaluationRepresentation>;
    type Args = RealmClientsWithClientUuidEvaluateScopesProtocolMappersGetArgs;

    fn opts(
        self,
        Self::Args { scope }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_protocol_mappers_get(
                self.realm_admin.realm,
                self.client_uuid,
                scope,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ProtocolMapperEvaluationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// either realm name OR client UUID
    pub role_container_id: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGetArgs {
    pub scope: Option<String>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args =
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGetArgs;

    fn opts(
        self,
        Self::Args { scope }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.role_container_id,
                scope,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// either realm name OR client UUID
    pub role_container_id: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGetArgs
{
    pub scope: Option<String>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
        'a,
        TS,
    >
{
    type Output = TypeVec<RoleRepresentation>;
    type Args =
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGetArgs;

    fn opts(
        self,
        Self::Args { scope }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.role_container_id,
                scope,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
        'a,
        TS,
    >
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidOfflineSessionsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidOfflineSessionsGetArgs {
    /// Paging offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidOfflineSessionsGet<'a, TS>
{
    type Output = TypeVec<UserSessionRepresentation>;
    type Args = RealmClientsWithClientUuidOfflineSessionsGetArgs;

    fn opts(
        self,
        Self::Args { first, max }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_offline_sessions_get(
                self.realm_admin.realm,
                self.client_uuid,
                first,
                max,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidOfflineSessionsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-clients")]
pub struct RealmClientsWithClientUuidUserSessionsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidUserSessionsGetArgs {
    /// Paging offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

#[cfg(feature = "tag-clients")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidUserSessionsGet<'a, TS>
{
    type Output = TypeVec<UserSessionRepresentation>;
    type Args = RealmClientsWithClientUuidUserSessionsGetArgs;

    fn opts(
        self,
        Self::Args { first, max }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_user_sessions_get(
                self.realm_admin.realm,
                self.client_uuid,
                first,
                max,
            )
    }
}

#[cfg(feature = "tag-clients")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidUserSessionsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Component</h4>
#[cfg(feature = "tag-component")]
pub struct RealmComponentsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-component")]
#[derive(Default)]
pub struct RealmComponentsGetArgs {
    pub name: Option<String>,
    pub parent: Option<String>,
    pub type_: Option<String>,
}

#[cfg(feature = "tag-component")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmComponentsGet<'a, TS> {
    type Output = TypeVec<ComponentRepresentation>;
    type Args = RealmComponentsGetArgs;

    fn opts(
        self,
        Self::Args {
            name,
            parent,
            type_,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_components_get(self.realm_admin.realm, name, parent, type_)
    }
}

#[cfg(feature = "tag-component")]
impl<'a, TS> IntoFuture for RealmComponentsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ComponentRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-component")]
pub struct RealmComponentsWithIdSubComponentTypesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub id: &'a str,
}

#[cfg(feature = "tag-component")]
#[derive(Default)]
pub struct RealmComponentsWithIdSubComponentTypesGetArgs {
    pub type_: Option<String>,
}

#[cfg(feature = "tag-component")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmComponentsWithIdSubComponentTypesGet<'a, TS>
{
    type Output = TypeVec<ComponentTypeRepresentation>;
    type Args = RealmComponentsWithIdSubComponentTypesGetArgs;

    fn opts(
        self,
        Self::Args { type_ }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_components_with_id_sub_component_types_get(
                self.realm_admin.realm,
                self.id,
                type_,
            )
    }
}

#[cfg(feature = "tag-component")]
impl<'a, TS> IntoFuture for RealmComponentsWithIdSubComponentTypesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Groups</h4>
#[cfg(feature = "tag-groups")]
pub struct RealmGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsGetArgs {
    pub brief_representation: Option<bool>,
    pub exact: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub populate_hierarchy: Option<bool>,
    pub q: Option<String>,
    pub search: Option<String>,
    /// Boolean which defines whether to return the count of subgroups for each group (default: true
    pub sub_groups_count: Option<bool>,
}

#[cfg(feature = "tag-groups")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmGroupsGet<'a, TS> {
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            populate_hierarchy,
            q,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_groups_get(
            self.realm_admin.realm,
            brief_representation,
            exact,
            first,
            max,
            populate_hierarchy,
            q,
            search,
            sub_groups_count,
        )
    }
}

#[cfg(feature = "tag-groups")]
impl<'a, TS> IntoFuture for RealmGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-groups")]
pub struct RealmGroupsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsCountGetArgs {
    pub search: Option<String>,
    pub top: Option<bool>,
}

#[cfg(feature = "tag-groups")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmGroupsCountGet<'a, TS> {
    type Output = TypeMap<String, i64>;
    type Args = RealmGroupsCountGetArgs;

    fn opts(
        self,
        Self::Args { search, top }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_count_get(self.realm_admin.realm, search, top)
    }
}

#[cfg(feature = "tag-groups")]
impl<'a, TS> IntoFuture for RealmGroupsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, i64>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-groups")]
pub struct RealmGroupsWithGroupIdChildrenGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdChildrenGetArgs {
    /// Boolean which defines whether brief groups representations are returned or not (default: false)
    pub brief_representation: Option<bool>,
    /// Boolean which defines whether the params "search" must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be returned (pagination offset).
    pub first: Option<i32>,
    /// The maximum number of results that are to be returned. Defaults to 10
    pub max: Option<i32>,
    /// A String representing either an exact group name or a partial name
    pub search: Option<String>,
    /// Boolean which defines whether to return the count of subgroups for each subgroup of this group (default: true
    pub sub_groups_count: Option<bool>,
}

#[cfg(feature = "tag-groups")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmGroupsWithGroupIdChildrenGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmGroupsWithGroupIdChildrenGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_with_group_id_children_get(
                self.realm_admin.realm,
                self.group_id,
                brief_representation,
                exact,
                first,
                max,
                search,
                sub_groups_count,
            )
    }
}

#[cfg(feature = "tag-groups")]
impl<'a, TS> IntoFuture for RealmGroupsWithGroupIdChildrenGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-groups")]
pub struct RealmGroupsWithGroupIdMembersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdMembersGetArgs {
    /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    pub brief_representation: Option<bool>,
    /// Pagination offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

#[cfg(feature = "tag-groups")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmGroupsWithGroupIdMembersGet<'a, TS>
{
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmGroupsWithGroupIdMembersGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_with_group_id_members_get(
                self.realm_admin.realm,
                self.group_id,
                brief_representation,
                first,
                max,
            )
    }
}

#[cfg(feature = "tag-groups")]
impl<'a, TS> IntoFuture for RealmGroupsWithGroupIdMembersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Identity Providers</h4>
#[cfg(feature = "tag-identity-providers")]
pub struct RealmIdentityProviderInstancesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-identity-providers")]
#[derive(Default)]
pub struct RealmIdentityProviderInstancesGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// Pagination offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    /// Boolean which defines if only realm-level IDPs (not associated with orgs) should be returned (default: false)
    pub realm_only: Option<bool>,
    /// Filter specific providers by name. Search can be prefix (name*), contains (*name*) or exact ("name"). Default prefixed.
    pub search: Option<String>,
}

#[cfg(feature = "tag-identity-providers")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmIdentityProviderInstancesGet<'a, TS>
{
    type Output = TypeVec<IdentityProviderRepresentation>;
    type Args = RealmIdentityProviderInstancesGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
            realm_only,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_identity_provider_instances_get(
                self.realm_admin.realm,
                brief_representation,
                first,
                max,
                realm_only,
                search,
            )
    }
}

#[cfg(feature = "tag-identity-providers")]
impl<'a, TS> IntoFuture for RealmIdentityProviderInstancesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<IdentityProviderRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-identity-providers")]
pub struct RealmIdentityProviderInstancesWithAliasExportGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub alias: &'a str,
}

#[cfg(feature = "tag-identity-providers")]
#[derive(Default)]
pub struct RealmIdentityProviderInstancesWithAliasExportGetArgs {
    /// Format to use
    pub format: Option<String>,
}

#[cfg(feature = "tag-identity-providers")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmIdentityProviderInstancesWithAliasExportGet<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmIdentityProviderInstancesWithAliasExportGetArgs;

    fn opts(
        self,
        Self::Args { format }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_identity_provider_instances_with_alias_export_get(
                self.realm_admin.realm,
                self.alias,
                format,
            )
    }
}

#[cfg(feature = "tag-identity-providers")]
impl<'a, TS> IntoFuture for RealmIdentityProviderInstancesWithAliasExportGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Key</h4>

// <h4>Organizations</h4>
#[cfg(feature = "tag-organizations")]
pub struct RealmOrganizationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned - defaults to 10
    pub max: Option<i32>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String representing either an organization name or domain
    pub search: Option<String>,
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmOrganizationsGet<'a, TS> {
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_organizations_get(
            self.realm_admin.realm,
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
        )
    }
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS> IntoFuture for RealmOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-organizations")]
pub struct RealmOrganizationsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsCountGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String representing either an organization name or domain
    pub search: Option<String>,
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmOrganizationsCountGet<'a, TS>
{
    type Output = i64;
    type Args = RealmOrganizationsCountGetArgs;

    fn opts(
        self,
        Self::Args { exact, q, search }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_organizations_count_get(
            self.realm_admin.realm,
            exact,
            q,
            search,
        )
    }
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS> IntoFuture for RealmOrganizationsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<i64, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-organizations")]
pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub member_id: &'a str,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_members_with_member_id_organizations_get(
                self.realm_admin.realm,
                self.member_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS> IntoFuture for RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-organizations")]
pub struct RealmOrganizationsWithOrgIdMembersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned. Defaults to 10
    pub max: Option<i32>,
    /// The membership type
    pub membership_type: Option<String>,
    /// A String representing either a member's username, e-mail, first name, or last name.
    pub search: Option<String>,
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersGet<'a, TS>
{
    type Output = TypeVec<MemberRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersGetArgs;

    fn opts(
        self,
        Self::Args {
            exact,
            first,
            max,
            membership_type,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_get(
                self.realm_admin.realm,
                self.org_id,
                exact,
                first,
                max,
                membership_type,
                search,
            )
    }
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<MemberRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-organizations")]
pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub member_id: &'a str,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_with_member_id_organizations_get(
                self.realm_admin.realm,
                self.org_id,
                self.member_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-organizations")]
impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Protocol Mappers</h4>

// <h4>Realms Admin</h4>
#[cfg(feature = "tag-realms-admin")]
pub struct RealmAdminEventsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmAdminEventsGetArgs {
    pub auth_client: Option<String>,
    pub auth_ip_address: Option<String>,
    pub auth_realm: Option<String>,
    /// user id
    pub auth_user: Option<String>,
    /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_from: Option<String>,
    /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_to: Option<String>,
    /// The direction to sort events by (asc or desc)
    pub direction: Option<String>,
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    pub operation_types: Option<Vec<String>>,
    pub resource_path: Option<String>,
    pub resource_types: Option<Vec<String>>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmAdminEventsGet<'a, TS> {
    type Output = TypeVec<AdminEventRepresentation>;
    type Args = RealmAdminEventsGetArgs;

    fn opts(
        self,
        Self::Args {
            auth_client,
            auth_ip_address,
            auth_realm,
            auth_user,
            date_from,
            date_to,
            direction,
            first,
            max,
            operation_types,
            resource_path,
            resource_types,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_admin_events_get(
            self.realm_admin.realm,
            auth_client,
            auth_ip_address,
            auth_realm,
            auth_user,
            date_from,
            date_to,
            direction,
            first,
            max,
            operation_types,
            resource_path,
            resource_types,
        )
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmAdminEventsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<AdminEventRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-realms-admin")]
pub struct RealmClientPoliciesPoliciesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmClientPoliciesPoliciesGetArgs {
    pub include_global_policies: Option<bool>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientPoliciesPoliciesGet<'a, TS>
{
    type Output = ClientPoliciesRepresentation;
    type Args = RealmClientPoliciesPoliciesGetArgs;

    fn opts(
        self,
        Self::Args {
            include_global_policies,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_policies_policies_get(self.realm_admin.realm, include_global_policies)
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmClientPoliciesPoliciesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ClientPoliciesRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-realms-admin")]
pub struct RealmClientPoliciesProfilesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmClientPoliciesProfilesGetArgs {
    pub include_global_profiles: Option<bool>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientPoliciesProfilesGet<'a, TS>
{
    type Output = ClientProfilesRepresentation;
    type Args = RealmClientPoliciesProfilesGetArgs;

    fn opts(
        self,
        Self::Args {
            include_global_profiles,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_policies_profiles_get(self.realm_admin.realm, include_global_profiles)
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmClientPoliciesProfilesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ClientProfilesRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-realms-admin")]
pub struct RealmEventsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmEventsGetArgs {
    /// App or oauth client name
    pub client: Option<String>,
    /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_from: Option<String>,
    /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_to: Option<String>,
    /// The direction to sort events by (asc or desc)
    pub direction: Option<String>,
    /// Paging offset
    pub first: Option<i32>,
    /// IP Address
    pub ip_address: Option<String>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    /// The types of events to return
    pub type_: Option<Vec<String>>,
    /// User id
    pub user: Option<String>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmEventsGet<'a, TS> {
    type Output = TypeVec<EventRepresentation>;
    type Args = RealmEventsGetArgs;

    fn opts(
        self,
        Self::Args {
            client,
            date_from,
            date_to,
            direction,
            first,
            ip_address,
            max,
            type_,
            user,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_events_get(
            self.realm_admin.realm,
            client,
            date_from,
            date_to,
            direction,
            first,
            ip_address,
            max,
            type_,
            user,
        )
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmEventsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<EventRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-realms-admin")]
pub struct RealmLocalizationWithLocaleGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub locale: &'a str,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmLocalizationWithLocaleGetArgs {
    pub use_realm_default_locale_fallback: Option<bool>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmLocalizationWithLocaleGet<'a, TS>
{
    type Output = TypeMap<String, TypeString>;
    type Args = RealmLocalizationWithLocaleGetArgs;

    fn opts(
        self,
        Self::Args {
            use_realm_default_locale_fallback,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_localization_with_locale_get(
            self.realm_admin.realm,
            self.locale,
            use_realm_default_locale_fallback,
        )
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmLocalizationWithLocaleGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, TypeString>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-realms-admin")]
pub struct RealmPartialExportPost<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmPartialExportPostArgs {
    pub export_clients: Option<bool>,
    pub export_groups_and_roles: Option<bool>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmPartialExportPost<'a, TS> {
    type Output = RealmRepresentation;
    type Args = RealmPartialExportPostArgs;

    fn opts(
        self,
        Self::Args {
            export_clients,
            export_groups_and_roles,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_partial_export_post(
            self.realm_admin.realm,
            export_clients,
            export_groups_and_roles,
        )
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmPartialExportPost<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<RealmRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-realms-admin")]
pub struct RealmSessionsWithSessionDelete<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub session: &'a str,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmSessionsWithSessionDeleteArgs {
    pub is_offline: Option<bool>,
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmSessionsWithSessionDelete<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmSessionsWithSessionDeleteArgs;

    fn opts(
        self,
        Self::Args { is_offline }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_sessions_with_session_delete(
            self.realm_admin.realm,
            self.session,
            is_offline,
        )
    }
}

#[cfg(feature = "tag-realms-admin")]
impl<'a, TS> IntoFuture for RealmSessionsWithSessionDelete<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Role Mapper</h4>
#[cfg(feature = "tag-role-mapper")]
pub struct RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
}

#[cfg(feature = "tag-role-mapper")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdRoleMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-role-mapper")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmGroupsWithGroupIdRoleMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_with_group_id_role_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.group_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-role-mapper")]
impl<'a, TS> IntoFuture for RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-role-mapper")]
pub struct RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[cfg(feature = "tag-role-mapper")]
#[derive(Default)]
pub struct RealmUsersWithUserIdRoleMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-role-mapper")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmUsersWithUserIdRoleMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_role_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.user_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-role-mapper")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Roles</h4>
#[cfg(feature = "tag-roles")]
pub struct RealmClientsWithClientUuidRolesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

#[cfg(feature = "tag-roles")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidRolesGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientsWithClientUuidRolesGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_roles_get(
                self.realm_admin.realm,
                self.client_uuid,
                brief_representation,
                first,
                max,
                search,
            )
    }
}

#[cfg(feature = "tag-roles")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidRolesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-roles")]
pub struct RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// the role name.
    pub role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesWithRoleNameGroupsGetArgs {
    /// if false, return a full representation of the {@code GroupRepresentation} objects.
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

#[cfg(feature = "tag-roles")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'a, TS>
{
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmClientsWithClientUuidRolesWithRoleNameGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_roles_with_role_name_groups_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.role_name,
                brief_representation,
                first,
                max,
            )
    }
}

#[cfg(feature = "tag-roles")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-roles")]
pub struct RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// the role name.
    pub role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesWithRoleNameUsersGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

#[cfg(feature = "tag-roles")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'a, TS>
{
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmClientsWithClientUuidRolesWithRoleNameUsersGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_roles_with_role_name_users_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.role_name,
                brief_representation,
                first,
                max,
            )
    }
}

#[cfg(feature = "tag-roles")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-roles")]
pub struct RealmRolesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmRolesGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

#[cfg(feature = "tag-roles")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmRolesGet<'a, TS> {
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmRolesGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_roles_get(
            self.realm_admin.realm,
            brief_representation,
            first,
            max,
            search,
        )
    }
}

#[cfg(feature = "tag-roles")]
impl<'a, TS> IntoFuture for RealmRolesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-roles")]
pub struct RealmRolesWithRoleNameGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// the role name.
    pub role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmRolesWithRoleNameGroupsGetArgs {
    /// if false, return a full representation of the {@code GroupRepresentation} objects.
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

#[cfg(feature = "tag-roles")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmRolesWithRoleNameGroupsGet<'a, TS>
{
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmRolesWithRoleNameGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_roles_with_role_name_groups_get(
                self.realm_admin.realm,
                self.role_name,
                brief_representation,
                first,
                max,
            )
    }
}

#[cfg(feature = "tag-roles")]
impl<'a, TS> IntoFuture for RealmRolesWithRoleNameGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-roles")]
pub struct RealmRolesWithRoleNameUsersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// the role name.
    pub role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmRolesWithRoleNameUsersGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

#[cfg(feature = "tag-roles")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmRolesWithRoleNameUsersGet<'a, TS>
{
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmRolesWithRoleNameUsersGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_roles_with_role_name_users_get(
            self.realm_admin.realm,
            self.role_name,
            brief_representation,
            first,
            max,
        )
    }
}

#[cfg(feature = "tag-roles")]
impl<'a, TS> IntoFuture for RealmRolesWithRoleNameUsersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Roles (by ID)</h4>
#[cfg(feature = "tag-roles-by-id")]
pub struct RealmRolesByIdWithRoleIdCompositesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub role_id: &'a str,
}

#[cfg(feature = "tag-roles-by-id")]
#[derive(Default)]
pub struct RealmRolesByIdWithRoleIdCompositesGetArgs {
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

#[cfg(feature = "tag-roles-by-id")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmRolesByIdWithRoleIdCompositesGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmRolesByIdWithRoleIdCompositesGetArgs;

    fn opts(
        self,
        Self::Args { first, max, search }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_roles_by_id_with_role_id_composites_get(
                self.realm_admin.realm,
                self.role_id,
                first,
                max,
                search,
            )
    }
}

#[cfg(feature = "tag-roles-by-id")]
impl<'a, TS> IntoFuture for RealmRolesByIdWithRoleIdCompositesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Scope Mappings</h4>
#[cfg(feature = "tag-scope-mappings")]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
    pub client: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                self.client,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS> IntoFuture
    for RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-scope-mappings")]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS> IntoFuture for RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-scope-mappings")]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
    pub client: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                self.client,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS> IntoFuture
    for RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-scope-mappings")]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS> IntoFuture
    for RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-scope-mappings")]
pub struct RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub client: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_composite_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.client,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-scope-mappings")]
pub struct RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidScopeMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientsWithClientUuidScopeMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_scope_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.client_uuid,
                brief_representation,
            )
    }
}

#[cfg(feature = "tag-scope-mappings")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>Users</h4>
#[cfg(feature = "tag-users")]
pub struct RealmUsersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// A String contained in email, or the complete email, if param "exact" is true
    pub email: Option<String>,
    /// whether the email has been verified
    pub email_verified: Option<bool>,
    /// Boolean representing if user is enabled or not
    pub enabled: Option<bool>,
    /// Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
    pub exact: Option<bool>,
    /// Pagination offset
    pub first: Option<i32>,
    /// A String contained in firstName, or the complete firstName, if param "exact" is true
    pub first_name: Option<String>,
    /// The alias of an Identity Provider linked to the user
    pub idp_alias: Option<String>,
    /// The userId at an Identity Provider linked to the user
    pub idp_user_id: Option<String>,
    /// A String contained in lastName, or the complete lastName, if param "exact" is true
    pub last_name: Option<String>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    pub search: Option<String>,
    /// A String contained in username, or the complete username, if param "exact" is true
    pub username: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmUsersGet<'a, TS> {
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmUsersGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            email,
            email_verified,
            enabled,
            exact,
            first,
            first_name,
            idp_alias,
            idp_user_id,
            last_name,
            max,
            q,
            search,
            username,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_get(
            self.realm_admin.realm,
            brief_representation,
            email,
            email_verified,
            enabled,
            exact,
            first,
            first_name,
            idp_alias,
            idp_user_id,
            last_name,
            max,
            q,
            search,
            username,
        )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersCountGetArgs {
    /// email filter
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    /// Boolean representing if user is enabled or not
    pub enabled: Option<bool>,
    /// first name filter
    pub first_name: Option<String>,
    /// last name filter
    pub last_name: Option<String>,
    pub q: Option<String>,
    /// arbitrary search string for all the fields below. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    pub search: Option<String>,
    /// username filter
    pub username: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmUsersCountGet<'a, TS> {
    type Output = i32;
    type Args = RealmUsersCountGetArgs;

    fn opts(
        self,
        Self::Args {
            email,
            email_verified,
            enabled,
            first_name,
            last_name,
            q,
            search,
            username,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_count_get(
            self.realm_admin.realm,
            email,
            email_verified,
            enabled,
            first_name,
            last_name,
            q,
            search,
            username,
        )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<i32, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersWithUserIdGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdGetArgs {
    /// Indicates if the user profile metadata should be added to the response
    pub user_profile_metadata: Option<bool>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmUsersWithUserIdGet<'a, TS> {
    type Output = UserRepresentation;
    type Args = RealmUsersWithUserIdGetArgs;

    fn opts(
        self,
        Self::Args {
            user_profile_metadata,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_with_user_id_get(
            self.realm_admin.realm,
            self.user_id,
            user_profile_metadata,
        )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<UserRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
    pub body: Vec<String>,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdExecuteActionsEmailPutArgs {
    /// Client id
    pub client_id: Option<String>,
    /// Number of seconds after which the generated token expires
    pub lifespan: Option<i32>,
    /// Redirect uri
    pub redirect_uri: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmUsersWithUserIdExecuteActionsEmailPutArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            lifespan,
            redirect_uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_execute_actions_email_put(
                self.realm_admin.realm,
                self.user_id,
                client_id,
                lifespan,
                redirect_uri,
                self.body,
            )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersWithUserIdGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdGroupsGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdGroupsGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmUsersWithUserIdGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_with_user_id_groups_get(
            self.realm_admin.realm,
            self.user_id,
            brief_representation,
            first,
            max,
            search,
        )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersWithUserIdGroupsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdGroupsCountGetArgs {
    pub search: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdGroupsCountGet<'a, TS>
{
    type Output = TypeMap<String, i64>;
    type Args = RealmUsersWithUserIdGroupsCountGetArgs;

    fn opts(
        self,
        Self::Args { search }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_groups_count_get(self.realm_admin.realm, self.user_id, search)
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdGroupsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, i64>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersWithUserIdResetPasswordEmailPut<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdResetPasswordEmailPutArgs {
    /// client id
    pub client_id: Option<String>,
    /// redirect uri
    pub redirect_uri: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdResetPasswordEmailPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmUsersWithUserIdResetPasswordEmailPutArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            redirect_uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_reset_password_email_put(
                self.realm_admin.realm,
                self.user_id,
                client_id,
                redirect_uri,
            )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdResetPasswordEmailPut<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-users")]
pub struct RealmUsersWithUserIdSendVerifyEmailPut<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdSendVerifyEmailPutArgs {
    /// Client id
    pub client_id: Option<String>,
    /// Number of seconds after which the generated token expires
    pub lifespan: Option<i32>,
    /// Redirect uri
    pub redirect_uri: Option<String>,
}

#[cfg(feature = "tag-users")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdSendVerifyEmailPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmUsersWithUserIdSendVerifyEmailPutArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            lifespan,
            redirect_uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_send_verify_email_put(
                self.realm_admin.realm,
                self.user_id,
                client_id,
                lifespan,
                redirect_uri,
            )
    }
}

#[cfg(feature = "tag-users")]
impl<'a, TS> IntoFuture for RealmUsersWithUserIdSendVerifyEmailPut<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

// <h4>default</h4>
#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS: KeycloakTokenSupplier>
{
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionGetArgs {
    pub fields: Option<String>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub permission: Option<bool>,
    pub policy_id: Option<String>,
    pub resource: Option<String>,
    pub resource_type: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS>
{
    type Output = TypeVec<AbstractPolicyRepresentation>;
    type Args = RealmClientsWithClientUuidAuthzResourceServerPermissionGetArgs;

    fn opts(
        self,
        Self::Args {
            fields,
            first,
            max,
            name,
            owner,
            permission,
            policy_id,
            resource,
            resource_type,
            scope,
            type_,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_permission_get(
                self.realm_admin.realm,
                self.client_uuid,
                fields,
                first,
                max,
                name,
                owner,
                permission,
                policy_id,
                resource,
                resource_type,
                scope,
                type_,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<AbstractPolicyRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGetArgs {
    pub fields: Option<String>,
    pub name: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'a, TS>
{
    type Output = AbstractPolicyRepresentation;
    type Args = RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGetArgs;

    fn opts(
        self,
        Self::Args { fields, name }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_permission_search_get(
                self.realm_admin.realm,
                self.client_uuid,
                fields,
                name,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<AbstractPolicyRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicyGetArgs {
    pub fields: Option<String>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub permission: Option<bool>,
    pub policy_id: Option<String>,
    pub resource: Option<String>,
    pub resource_type: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS>
{
    type Output = TypeVec<AbstractPolicyRepresentation>;
    type Args = RealmClientsWithClientUuidAuthzResourceServerPolicyGetArgs;

    fn opts(
        self,
        Self::Args {
            fields,
            first,
            max,
            name,
            owner,
            permission,
            policy_id,
            resource,
            resource_type,
            scope,
            type_,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_policy_get(
                self.realm_admin.realm,
                self.client_uuid,
                fields,
                first,
                max,
                name,
                owner,
                permission,
                policy_id,
                resource,
                resource_type,
                scope,
                type_,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<AbstractPolicyRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicySearchGetArgs {
    pub fields: Option<String>,
    pub name: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'a, TS>
{
    type Output = AbstractPolicyRepresentation;
    type Args = RealmClientsWithClientUuidAuthzResourceServerPolicySearchGetArgs;

    fn opts(
        self,
        Self::Args { fields, name }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_policy_search_get(
                self.realm_admin.realm,
                self.client_uuid,
                fields,
                name,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<AbstractPolicyRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceGetArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS>
{
    type Output = TypeVec<ResourceRepresentation>;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourceGetArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_get(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ResourceRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourcePost<'a, TS: KeycloakTokenSupplier>
{
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub body: ResourceRepresentation,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourcePostArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourcePost<'a, TS>
{
    type Output = ResourceRepresentation;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourcePostArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_post(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.body,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerResourcePost<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ResourceRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceSearchGetArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'a, TS>
{
    type Output = ResourceRepresentation;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourceSearchGetArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_search_get(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ResourceRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGetArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet<'a, TS>
{
    type Output = ResourceRepresentation;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGetArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_get(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.resource_id,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ResourceRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub resource_id: &'a str,
    pub body: ResourceRepresentation,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPutArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPutArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_put(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.resource_id,
                self.body,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDeleteArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDeleteArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_delete(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.resource_id,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGetArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet<'a, TS>
{
    type Output = DefaultResponse;
    type Args =
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGetArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_attributes_get(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.resource_id,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGetArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet<'a, TS>
{
    type Output = TypeVec<PolicyRepresentation>;
    type Args =
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGetArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_permissions_get(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.resource_id,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<PolicyRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGetArgs {
    pub id: Option<String>,
    pub deep: Option<bool>,
    pub exact_name: Option<bool>,
    pub first: Option<i32>,
    pub matching_uri: Option<bool>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet<'a, TS>
{
    type Output = TypeVec<ScopeRepresentation>;
    type Args = RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGetArgs;

    fn opts(
        self,
        Self::Args {
            id,
            deep,
            exact_name,
            first,
            matching_uri,
            max,
            name,
            owner,
            scope,
            type_,
            uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_scopes_get(
                self.realm_admin.realm,
                self.client_uuid,
                id,
                deep,
                exact_name,
                first,
                matching_uri,
                max,
                name,
                owner,
                scope,
                type_,
                uri,
                self.resource_id,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ScopeRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeGetArgs {
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub scope_id: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS>
{
    type Output = TypeVec<ScopeRepresentation>;
    type Args = RealmClientsWithClientUuidAuthzResourceServerScopeGetArgs;

    fn opts(
        self,
        Self::Args {
            first,
            max,
            name,
            scope_id,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_get(
                self.realm_admin.realm,
                self.client_uuid,
                first,
                max,
                name,
                scope_id,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ScopeRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "tag-none")]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeSearchGetArgs {
    pub name: Option<String>,
}

#[cfg(feature = "tag-none")]
impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'a, TS>
{
    type Output = TypeVec<ScopeRepresentation>;
    type Args = RealmClientsWithClientUuidAuthzResourceServerScopeSearchGetArgs;

    fn opts(
        self,
        Self::Args { name }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_authz_resource_server_scope_search_get(
                self.realm_admin.realm,
                self.client_uuid,
                name,
            )
    }
}

#[cfg(feature = "tag-none")]
impl<'a, TS> IntoFuture for RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ScopeRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}
