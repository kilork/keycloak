use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Role Mappings</h4>
    /// Get client-level role mappings for the user or group, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_idavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}/available`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_idcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}/composite`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_idavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}/available`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_idcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}/composite`
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
}

// <h4>Client Role Mappings</h4>
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

#[derive(Default)]
pub struct RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
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

impl<'a, TS> IntoFuture
    for RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

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

#[derive(Default)]
pub struct RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
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

impl<'a, TS> IntoFuture for RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Client Role Mappings</h4>
    impl<'a, TS> RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmGroupsWithGroupIdRoleMappingsClientsWithClientIdCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdRoleMappingsClientsWithClientIdCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }
}
