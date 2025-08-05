use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Roles</h4>
    /// Get all roles for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidroles>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidroles>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/roles`
    pub fn clients_with_client_uuid_roles_post(
        &'a self,
        client_uuid: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_post(self.realm, client_uuid, body)
    }

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecompositesclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites/clients/{client-uuid}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites/realm`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/groups`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/management/permissions`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/management/permissions`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/users`
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmroles>
    pub fn roles_get(&'a self) -> RealmRolesGet<'a, TS> {
        RealmRolesGet { realm_admin: self }
    }

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmroles>
    pub fn roles_post(
        &'a self,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_roles_post(self.realm, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}`
    pub fn roles_with_role_name_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_get(self.realm, role_name)
    }

    /// Update a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}`
    pub fn roles_with_role_name_put(
        &'a self,
        role_name: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_put(self.realm, role_name, body)
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}`
    pub fn roles_with_role_name_delete(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_delete(self.realm, role_name)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites`
    pub fn roles_with_role_name_composites_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_get(self.realm, role_name)
    }

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles/{role-name}/composites`
    pub fn roles_with_role_name_composites_post(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_post(self.realm, role_name, body)
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}/composites`
    pub fn roles_with_role_name_composites_delete(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_delete(self.realm, role_name, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/clients/{client-uuid}`
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/realm`
    pub fn roles_with_role_name_composites_realm_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_realm_get(self.realm, role_name)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/groups`
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/management/permissions`
    pub fn roles_with_role_name_management_permissions_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_management_permissions_get(self.realm, role_name)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}/management/permissions`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/users`
    pub fn roles_with_role_name_users_get(
        &'a self,
        role_name: &'a str,
    ) -> RealmRolesWithRoleNameUsersGet<'a, TS> {
        RealmRolesWithRoleNameUsersGet {
            realm_admin: self,
            role_name,
        }
    }
}

// <h4>Roles</h4>
pub struct RealmClientsWithClientUuidRolesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

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

pub struct RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// the role name.
    pub role_name: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesWithRoleNameGroupsGetArgs {
    /// if false, return a full representation of the {@code GroupRepresentation} objects.
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

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

pub struct RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// the role name.
    pub role_name: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidRolesWithRoleNameUsersGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

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

pub struct RealmRolesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmRolesGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

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

pub struct RealmRolesWithRoleNameGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// the role name.
    pub role_name: &'a str,
}

#[derive(Default)]
pub struct RealmRolesWithRoleNameGroupsGetArgs {
    /// if false, return a full representation of the {@code GroupRepresentation} objects.
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

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

pub struct RealmRolesWithRoleNameUsersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// the role name.
    pub role_name: &'a str,
}

#[derive(Default)]
pub struct RealmRolesWithRoleNameUsersGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// first result to return. Ignored if negative or {@code null}.
    pub first: Option<i32>,
    /// maximum number of results to return. Ignored if negative or {@code null}.
    pub max: Option<i32>,
}

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

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Roles</h4>
    impl<'a, TS> RealmClientsWithClientUuidRolesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidRolesGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
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
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return a full representation of the {@code GroupRepresentation} objects.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidRolesWithRoleNameGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return a full representation of the {@code GroupRepresentation} objects.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidRolesWithRoleNameUsersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }

    impl<'a, TS> RealmRolesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmRolesGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
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
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmRolesWithRoleNameGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return a full representation of the {@code GroupRepresentation} objects.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmRolesWithRoleNameGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return a full representation of the {@code GroupRepresentation} objects.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }

    impl<'a, TS> RealmRolesWithRoleNameUsersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmRolesWithRoleNameUsersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// first result to return. Ignored if negative or {@code null}.
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// maximum number of results to return. Ignored if negative or {@code null}.
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }
}
