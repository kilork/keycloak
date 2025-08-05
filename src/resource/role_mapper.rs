use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Role Mapper</h4>
    /// Get role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings`
    pub fn groups_with_group_id_role_mappings_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_get(self.realm, group_id)
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    pub fn groups_with_group_id_role_mappings_realm_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_get(self.realm, group_id)
    }

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    pub fn groups_with_group_id_role_mappings_realm_post(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_post(self.realm, group_id, body)
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    pub fn groups_with_group_id_role_mappings_realm_delete(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_delete(self.realm, group_id, body)
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm/available`
    pub fn groups_with_group_id_role_mappings_realm_available_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_available_get(self.realm, group_id)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm/composite`
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings`
    pub fn users_with_user_id_role_mappings_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_get(self.realm, user_id)
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    pub fn users_with_user_id_role_mappings_realm_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_get(self.realm, user_id)
    }

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_post_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    pub fn users_with_user_id_role_mappings_realm_post(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_post(self.realm, user_id, body)
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_delete_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    pub fn users_with_user_id_role_mappings_realm_delete(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_delete(self.realm, user_id, body)
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm/available`
    pub fn users_with_user_id_role_mappings_realm_available_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_available_get(self.realm, user_id)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm/composite`
    pub fn users_with_user_id_role_mappings_realm_composite_get(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS> {
        RealmUsersWithUserIdRoleMappingsRealmCompositeGet {
            realm_admin: self,
            user_id,
        }
    }
}

// <h4>Role Mapper</h4>
pub struct RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
}

#[derive(Default)]
pub struct RealmGroupsWithGroupIdRoleMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

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

pub struct RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdRoleMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

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

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Role Mapper</h4>
    impl<'a, TS> RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmGroupsWithGroupIdRoleMappingsRealmCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdRoleMappingsRealmCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }
}
