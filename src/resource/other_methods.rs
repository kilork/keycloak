use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
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
    pub fn clients_with_client_uuid_authz_resource_server_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ResourceServerRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_get(self.realm, client_uuid)
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
    pub fn clients_with_client_uuid_authz_resource_server_put(
        &'a self,
        client_uuid: &'a str,
        body: ResourceServerRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_put(self.realm, client_uuid, body)
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
    pub fn clients_with_client_uuid_authz_resource_server_permission_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPermissionGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_permission_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_policy_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPolicyGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_policy_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_resource_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_resource_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_scope_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerScopeGet {
            realm_admin: self,
            client_uuid,
        }
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
    pub fn clients_with_client_uuid_authz_resource_server_scope_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet {
            realm_admin: self,
            client_uuid,
        }
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

// <h4>default</h4>
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS: KeycloakTokenSupplier>
{
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGetArgs {
    pub fields: Option<String>,
    pub name: Option<String>,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerPolicySearchGetArgs {
    pub fields: Option<String>,
    pub name: Option<String>,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerResourcePost<'a, TS: KeycloakTokenSupplier>
{
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub body: ResourceRepresentation,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeGetArgs {
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub name: Option<String>,
    pub scope_id: Option<String>,
}

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

pub struct RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidAuthzResourceServerScopeSearchGetArgs {
    pub name: Option<String>,
}

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

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>default</h4>
    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().fields(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn permission(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().permission(value)
        }
        pub fn policy_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().policy_id(value)
        }
        pub fn resource(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().resource(value)
        }
        pub fn resource_type(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().resource_type(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.fields = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn permission(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.permission = value.into();
            self
        }
        pub fn policy_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.policy_id = value.into();
            self
        }
        pub fn resource(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.resource = value.into();
            self
        }
        pub fn resource_type(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.resource_type = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().fields(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.fields = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().fields(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn permission(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().permission(value)
        }
        pub fn policy_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().policy_id(value)
        }
        pub fn resource(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().resource(value)
        }
        pub fn resource_type(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().resource_type(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.fields = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn permission(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.permission = value.into();
            self
        }
        pub fn policy_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.policy_id = value.into();
            self
        }
        pub fn resource(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.resource = value.into();
            self
        }
        pub fn resource_type(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.resource_type = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().fields(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn fields(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.fields = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerResourceGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourcePost<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerResourcePost<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPut<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdDelete<'_, TS>,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS>
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdAttributesGet<
                '_,
                TS,
            >,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS>
        RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdPermissionsGet<
                '_,
                TS,
            >,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().id(value)
        }
        pub fn deep(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().deep(value)
        }
        pub fn exact_name(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact_name(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn matching_uri(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().matching_uri(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn owner(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().owner(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        pub fn uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().uri(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientsWithClientUuidAuthzResourceServerResourceWithResourceIdScopesGet<'_, TS>,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.id = value.into();
            self
        }
        pub fn deep(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.deep = value.into();
            self
        }
        pub fn exact_name(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact_name = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn matching_uri(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.matching_uri = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn owner(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.owner = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        pub fn uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn scope_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope_id(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerScopeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn scope_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope_id = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
    }
}
