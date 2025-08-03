use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Attack Detection</h4>
    #[cfg(feature = "tag-attack-detection")]
    pub fn attack_detection_brute_force_users_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_delete(self.realm)
    }

    #[cfg(feature = "tag-attack-detection")]
    pub fn attack_detection_brute_force_users_with_user_id_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_get(self.realm, user_id)
    }

    #[cfg(feature = "tag-attack-detection")]
    pub fn attack_detection_brute_force_users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_delete(self.realm, user_id)
    }

    // <h4>Authentication Management</h4>
    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_authenticator_providers_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_client_authenticator_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_client_authenticator_providers_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    #[deprecated]
    pub fn authentication_config_post(
        &'a self,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_post(self.realm, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_description_with_provider_id_get(
        &'a self,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigInfoRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_description_with_provider_id_get(self.realm, provider_id)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticatorConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_config_with_id_get(self.realm, id)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticatorConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_put(self.realm, id, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_config_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_config_with_id_delete(self.realm, id)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_executions_post(
        &'a self,
        body: AuthenticationExecutionRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_post(self.realm, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_executions_with_execution_id_get(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationExecutionRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_executions_with_execution_id_get(self.realm, execution_id)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_executions_with_execution_id_delete(
        &'a self,
        execution_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_executions_with_execution_id_delete(self.realm, execution_id)
    }

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

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<AuthenticationFlowRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin.realm_authentication_flows_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_post(
        &'a self,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_authentication_flows_post(self.realm, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_flow_alias_copy_post(
        &'a self,
        flow_alias: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_copy_post(self.realm, flow_alias, body)
    }

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

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_flow_alias_executions_put(
        &'a self,
        flow_alias: &'a str,
        body: AuthenticationExecutionInfoRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_flow_alias_executions_put(self.realm, flow_alias, body)
    }

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

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<AuthenticationFlowRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_flows_with_id_get(self.realm, id)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_id_put(
        &'a self,
        id: &'a str,
        body: AuthenticationFlowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_put(self.realm, id, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_flows_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_flows_with_id_delete(self.realm, id)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_form_action_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_action_providers_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_form_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_form_providers_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_per_client_config_description_get(
        &'a self,
    ) -> impl Future<
        Output = Result<TypeMap<String, TypeVec<ConfigPropertyRepresentation>>, KeycloakError>,
    > + use<'a, TS> {
        self.admin
            .realm_authentication_per_client_config_description_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_register_required_action_post(
        &'a self,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_register_required_action_post(self.realm, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_get(self.realm)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_get(self.realm, alias)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_put(self.realm, alias, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_delete(self.realm, alias)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_config_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<RequiredActionConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_authentication_required_actions_with_alias_config_get(self.realm, alias)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_config_put(
        &'a self,
        alias: &'a str,
        body: RequiredActionConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_put(self.realm, alias, body)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_config_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_config_delete(self.realm, alias)
    }

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

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_lower_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_lower_priority_post(self.realm, alias)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_required_actions_with_alias_raise_priority_post(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_authentication_required_actions_with_alias_raise_priority_post(self.realm, alias)
    }

    #[cfg(feature = "tag-authentication-management")]
    pub fn authentication_unregistered_required_actions_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<RequiredActionProviderRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_authentication_unregistered_required_actions_get(self.realm)
    }

    // <h4>Client Attribute Certificate</h4>
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
    #[cfg(feature = "tag-client-registration-policy")]
    pub fn client_registration_policy_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_registration_policy_providers_get(self.realm)
    }

    // <h4>Client Role Mappings</h4>
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
    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_scopes_get(self.realm)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_scopes_post(self.realm, body)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_get(self.realm, client_scope_id)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_put(self.realm, client_scope_id, body)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_delete(self.realm, client_scope_id)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_templates_get(self.realm)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_post(
        &'a self,
        body: ClientScopeRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_templates_post(self.realm, body)
    }

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_with_client_scope_id_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<ClientScopeRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_get(self.realm, client_scope_id)
    }

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

    #[cfg(feature = "tag-client-scopes")]
    pub fn client_templates_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_delete(self.realm, client_scope_id)
    }

    // <h4>Clients</h4>
    #[cfg(feature = "tag-clients")]
    pub fn clients_get(&'a self) -> RealmClientsGet<'a, TS> {
        RealmClientsGet { realm_admin: self }
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_post(
        &'a self,
        body: ClientRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_clients_post(self.realm, body)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_put(
        &'a self,
        client_uuid: &'a str,
        body: ClientRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_put(self.realm, client_uuid, body)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_delete(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_delete(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_post(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_rotated_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_rotated_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_client_secret_rotated_delete(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_rotated_delete(self.realm, client_uuid)
    }

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

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_management_permissions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_management_permissions_get(self.realm, client_uuid)
    }

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

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_nodes_post(
        &'a self,
        client_uuid: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_nodes_post(self.realm, client_uuid, body)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_nodes_with_node_delete(
        &'a self,
        client_uuid: &'a str,
        node: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_nodes_with_node_delete(self.realm, client_uuid, node)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_offline_session_count_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, i64>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_offline_session_count_get(self.realm, client_uuid)
    }

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

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_push_revocation_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_push_revocation_post(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_registration_access_token_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_registration_access_token_post(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_service_account_user_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<UserRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_service_account_user_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_session_count_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, i64>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_session_count_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-clients")]
    pub fn clients_with_client_uuid_test_nodes_available_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_test_nodes_available_get(self.realm, client_uuid)
    }

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
    #[cfg(feature = "tag-groups")]
    pub fn groups_get(&'a self) -> RealmGroupsGet<'a, TS> {
        RealmGroupsGet { realm_admin: self }
    }

    #[cfg(feature = "tag-groups")]
    pub fn groups_post(
        &'a self,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_groups_post(self.realm, body)
    }

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

    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_children_post(
        &'a self,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_children_post(self.realm, group_id, body)
    }

    #[cfg(feature = "tag-groups")]
    pub fn groups_with_group_id_management_permissions_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_management_permissions_get(self.realm, group_id)
    }

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
    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_import_config_post(
        &'a self,
        body: TypeMap<String, Value>,
    ) -> impl Future<Output = Result<TypeMap<String, TypeString>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_import_config_post(self.realm, body)
    }

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_get(&'a self) -> RealmIdentityProviderInstancesGet<'a, TS> {
        RealmIdentityProviderInstancesGet { realm_admin: self }
    }

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_post(
        &'a self,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_post(self.realm, body)
    }

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_get(self.realm, alias)
    }

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_put(
        &'a self,
        alias: &'a str,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_put(self.realm, alias, body)
    }

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_delete(self.realm, alias)
    }

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

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_get(self.realm, alias)
    }

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_mappers_post(
        &'a self,
        alias: &'a str,
        body: IdentityProviderMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_post(self.realm, alias, body)
    }

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

    #[cfg(feature = "tag-identity-providers")]
    pub fn identity_provider_instances_with_alias_reload_keys_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<bool, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_reload_keys_get(self.realm, alias)
    }

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
    #[cfg(feature = "tag-organizations")]
    pub fn organizations_get(&'a self) -> RealmOrganizationsGet<'a, TS> {
        RealmOrganizationsGet { realm_admin: self }
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_post(
        &'a self,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_organizations_post(self.realm, body)
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_count_get(&'a self) -> RealmOrganizationsCountGet<'a, TS> {
        RealmOrganizationsCountGet { realm_admin: self }
    }

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

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<OrganizationRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_get(self.realm, org_id)
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_put(
        &'a self,
        org_id: &'a str,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_put(self.realm, org_id, body)
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_delete(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_delete(self.realm, org_id)
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_identity_providers_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_get(self.realm, org_id)
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_identity_providers_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_post(self.realm, org_id, body)
    }

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

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_post(self.realm, org_id, body)
    }

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_count_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<i64, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_count_get(self.realm, org_id)
    }

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

    #[cfg(feature = "tag-organizations")]
    pub fn organizations_with_org_id_members_invite_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_user_post(self.realm, org_id, body)
    }

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

    #[cfg(feature = "tag-protocol-mappers")]
    pub fn clients_with_client_uuid_protocol_mappers_models_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_get(self.realm, client_uuid)
    }

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
    #[cfg(feature = "tag-realms-admin")]
    pub fn get(
        &'a self,
    ) -> impl Future<Output = Result<RealmRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn put(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_put(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_delete(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn admin_events_get(&'a self) -> RealmAdminEventsGet<'a, TS> {
        RealmAdminEventsGet { realm_admin: self }
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn admin_events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_admin_events_delete(self.realm)
    }

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

    #[cfg(feature = "tag-realms-admin")]
    pub fn client_session_stats_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, String>>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_session_stats_get(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn client_types_get(
        &'a self,
    ) -> impl Future<Output = Result<ClientTypesRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_get(self.realm)
    }

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

    #[cfg(feature = "tag-realms-admin")]
    pub fn events_get(&'a self) -> RealmEventsGet<'a, TS> {
        RealmEventsGet { realm_admin: self }
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_delete(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn events_config_get(
        &'a self,
    ) -> impl Future<Output = Result<RealmEventsConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_events_config_get(self.realm)
    }

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

    #[cfg(feature = "tag-realms-admin")]
    pub fn logout_all_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_logout_all_post(self.realm)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn partial_export_post(&'a self) -> RealmPartialExportPost<'a, TS> {
        RealmPartialExportPost { realm_admin: self }
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn partial_import_post(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<Value, KeycloakError>> + use<'a, TS> {
        self.admin.realm_partial_import_post(self.realm, body)
    }

    #[cfg(feature = "tag-realms-admin")]
    pub fn push_revocation_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_push_revocation_post(self.realm)
    }

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
    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_get(self.realm, group_id)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_get(self.realm, group_id)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_post(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_post(self.realm, group_id, body)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_delete(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_delete(self.realm, group_id, body)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn groups_with_group_id_role_mappings_realm_available_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_available_get(self.realm, group_id)
    }

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

    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_get(self.realm, user_id)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_get(self.realm, user_id)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_post(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_post(self.realm, user_id, body)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_delete(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_delete(self.realm, user_id, body)
    }

    #[cfg(feature = "tag-role-mapper")]
    pub fn users_with_user_id_role_mappings_realm_available_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_available_get(self.realm, user_id)
    }

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

    #[cfg(feature = "tag-roles")]
    pub fn clients_with_client_uuid_roles_post(
        &'a self,
        client_uuid: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_post(self.realm, client_uuid, body)
    }

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

    #[cfg(feature = "tag-roles")]
    pub fn roles_get(&'a self) -> RealmRolesGet<'a, TS> {
        RealmRolesGet { realm_admin: self }
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_post(
        &'a self,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_roles_post(self.realm, body)
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_get(self.realm, role_name)
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_put(
        &'a self,
        role_name: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_put(self.realm, role_name, body)
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_delete(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_delete(self.realm, role_name)
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_get(self.realm, role_name)
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_post(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_post(self.realm, role_name, body)
    }

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_delete(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_delete(self.realm, role_name, body)
    }

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

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_composites_realm_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_realm_get(self.realm, role_name)
    }

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

    #[cfg(feature = "tag-roles")]
    pub fn roles_with_role_name_management_permissions_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_management_permissions_get(self.realm, role_name)
    }

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
    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_get(self.realm, role_id)
    }

    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_put(
        &'a self,
        role_id: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_put(self.realm, role_id, body)
    }

    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_delete(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_delete(self.realm, role_id)
    }

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

    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_post(
        &'a self,
        role_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_composites_post(self.realm, role_id, body)
    }

    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_delete(
        &'a self,
        role_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_composites_delete(self.realm, role_id, body)
    }

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

    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_composites_realm_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_composites_realm_get(self.realm, role_id)
    }

    #[cfg(feature = "tag-roles-by-id")]
    pub fn roles_by_id_with_role_id_management_permissions_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_management_permissions_get(self.realm, role_id)
    }

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

    #[cfg(feature = "tag-scope-mappings")]
    #[deprecated]
    pub fn clients_with_client_uuid_scope_mappings_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_get(self.realm, client_uuid)
    }

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

    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_get(self.realm, client_uuid)
    }

    #[cfg(feature = "tag-scope-mappings")]
    pub fn clients_with_client_uuid_scope_mappings_realm_post(
        &'a self,
        client_uuid: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_post(self.realm, client_uuid, body)
    }

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
    #[cfg(feature = "tag-users")]
    pub fn users_get(&'a self) -> RealmUsersGet<'a, TS> {
        RealmUsersGet { realm_admin: self }
    }

    #[cfg(feature = "tag-users")]
    pub fn users_post(
        &'a self,
        body: UserRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_post(self.realm, body)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_count_get(&'a self) -> RealmUsersCountGet<'a, TS> {
        RealmUsersCountGet { realm_admin: self }
    }

    #[cfg(feature = "tag-users")]
    pub fn users_profile_get(
        &'a self,
    ) -> impl Future<Output = Result<UPConfig, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_get(self.realm)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_profile_put(
        &'a self,
        body: UPConfig,
    ) -> impl Future<Output = Result<UPConfig, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_put(self.realm, body)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_profile_metadata_get(
        &'a self,
    ) -> impl Future<Output = Result<UserProfileMetadata, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_metadata_get(self.realm)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_get(&'a self, user_id: &'a str) -> RealmUsersWithUserIdGet<'a, TS> {
        RealmUsersWithUserIdGet {
            realm_admin: self,
            user_id,
        }
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_put(
        &'a self,
        user_id: &'a str,
        body: UserRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_put(self.realm, user_id, body)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_delete(self.realm, user_id)
    }

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

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_consents_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_consents_get(self.realm, user_id)
    }

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

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_disable_credential_types_put(
        &'a self,
        user_id: &'a str,
        body: Vec<String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_disable_credential_types_put(self.realm, user_id, body)
    }

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

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_federated_identity_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<FederatedIdentityRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_get(self.realm, user_id)
    }

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

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_impersonation_post(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_impersonation_post(self.realm, user_id)
    }

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_logout_post(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_logout_post(self.realm, user_id)
    }

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

    #[cfg(feature = "tag-users")]
    pub fn users_with_user_id_reset_password_put(
        &'a self,
        user_id: &'a str,
        body: CredentialRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_reset_password_put(self.realm, user_id, body)
    }

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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    group_id: &'a str,
    client_id: &'a str,
}

#[cfg(feature = "tag-client-role-mappings")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
    client_id: &'a str,
}

#[cfg(feature = "tag-client-role-mappings")]
#[derive(Default)]
pub struct RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsGetArgs {
    client_id: Option<String>,
    first: Option<i32>,
    max: Option<i32>,
    q: Option<String>,
    search: Option<bool>,
    viewable_only: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGetArgs {
    audience: Option<String>,
    scope: Option<String>,
    user_id: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGetArgs {
    audience: Option<String>,
    scope: Option<String>,
    user_id: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGetArgs {
    scope: Option<String>,
    user_id: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesProtocolMappersGetArgs {
    scope: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    role_container_id: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGetArgs {
    scope: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    role_container_id: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGetArgs
{
    scope: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidOfflineSessionsGetArgs {
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-clients")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidUserSessionsGetArgs {
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-component")]
#[derive(Default)]
pub struct RealmComponentsGetArgs {
    name: Option<String>,
    parent: Option<String>,
    type_: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    id: &'a str,
}

#[cfg(feature = "tag-component")]
#[derive(Default)]
pub struct RealmComponentsWithIdSubComponentTypesGetArgs {
    type_: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsGetArgs {
    brief_representation: Option<bool>,
    exact: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    populate_hierarchy: Option<bool>,
    q: Option<String>,
    search: Option<String>,
    sub_groups_count: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsCountGetArgs {
    search: Option<String>,
    top: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    group_id: &'a str,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdChildrenGetArgs {
    brief_representation: Option<bool>,
    exact: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    search: Option<String>,
    sub_groups_count: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    group_id: &'a str,
}

#[cfg(feature = "tag-groups")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdMembersGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-identity-providers")]
#[derive(Default)]
pub struct RealmIdentityProviderInstancesGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    realm_only: Option<bool>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    alias: &'a str,
}

#[cfg(feature = "tag-identity-providers")]
#[derive(Default)]
pub struct RealmIdentityProviderInstancesWithAliasExportGetArgs {
    format: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsGetArgs {
    brief_representation: Option<bool>,
    exact: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    q: Option<String>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsCountGetArgs {
    exact: Option<bool>,
    q: Option<String>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    member_id: &'a str,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    org_id: &'a str,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersGetArgs {
    exact: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    membership_type: Option<String>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    org_id: &'a str,
    member_id: &'a str,
}

#[cfg(feature = "tag-organizations")]
#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmAdminEventsGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmClientPoliciesPoliciesGetArgs {
    include_global_policies: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmClientPoliciesProfilesGetArgs {
    include_global_profiles: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmEventsGetArgs {
    client: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    direction: Option<String>,
    first: Option<i32>,
    ip_address: Option<String>,
    max: Option<i32>,
    type_: Option<Vec<String>>,
    user: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    locale: &'a str,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmLocalizationWithLocaleGetArgs {
    use_realm_default_locale_fallback: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmPartialExportPostArgs {
    export_clients: Option<bool>,
    export_groups_and_roles: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    session: &'a str,
}

#[cfg(feature = "tag-realms-admin")]
#[derive(Default)]
pub struct RealmSessionsWithSessionDeleteArgs {
    is_offline: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    group_id: &'a str,
}

#[cfg(feature = "tag-role-mapper")]
#[derive(Default)]
pub struct RealmGroupsWithGroupIdRoleMappingsRealmCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
}

#[cfg(feature = "tag-role-mapper")]
#[derive(Default)]
pub struct RealmUsersWithUserIdRoleMappingsRealmCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesWithRoleNameGroupsGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesWithRoleNameUsersGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmRolesGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmRolesWithRoleNameGroupsGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    role_name: &'a str,
}

#[cfg(feature = "tag-roles")]
#[derive(Default)]
pub struct RealmRolesWithRoleNameUsersGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    role_id: &'a str,
}

#[cfg(feature = "tag-roles-by-id")]
#[derive(Default)]
pub struct RealmRolesByIdWithRoleIdCompositesGetArgs {
    first: Option<i32>,
    max: Option<i32>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_scope_id: &'a str,
    client: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_scope_id: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_scope_id: &'a str,
    client: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_scope_id: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    client: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-scope-mappings")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidScopeMappingsRealmCompositeGetArgs {
    brief_representation: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersCountGetArgs {
    email: Option<String>,
    email_verified: Option<bool>,
    enabled: Option<bool>,
    first_name: Option<String>,
    last_name: Option<String>,
    q: Option<String>,
    search: Option<String>,
    username: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdGetArgs {
    user_profile_metadata: Option<bool>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
    body: Vec<String>,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdExecuteActionsEmailPutArgs {
    client_id: Option<String>,
    lifespan: Option<i32>,
    redirect_uri: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdGroupsGetArgs {
    brief_representation: Option<bool>,
    first: Option<i32>,
    max: Option<i32>,
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdGroupsCountGetArgs {
    search: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdResetPasswordEmailPutArgs {
    client_id: Option<String>,
    redirect_uri: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    user_id: &'a str,
}

#[cfg(feature = "tag-users")]
#[derive(Default)]
pub struct RealmUsersWithUserIdSendVerifyEmailPutArgs {
    client_id: Option<String>,
    lifespan: Option<i32>,
    redirect_uri: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGetArgs {
    fields: Option<String>,
    name: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicyGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicySearchGetArgs {
    fields: Option<String>,
    name: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    body: ResourceRepresentation,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourcePostArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceSearchGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    resource_id: &'a str,
    body: ResourceRepresentation,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPutArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDeleteArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
    resource_id: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGetArgs {
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeGetArgs {
    first: Option<i32>,
    max: Option<i32>,
    name: Option<String>,
    scope_id: Option<String>,
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
    realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    client_uuid: &'a str,
}

#[cfg(feature = "tag-none")]
#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeSearchGetArgs {
    name: Option<String>,
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
