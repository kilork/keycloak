use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Role Mapper</h4>
    /// Get role mappings
    pub fn groups_with_group_id_role_mappings_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_get(self.realm, group_id)
    }

    /// Get realm-level role mappings
    pub fn groups_with_group_id_role_mappings_realm_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_get(self.realm, group_id)
    }

    /// Add realm-level role mappings to the user
    pub fn groups_with_group_id_role_mappings_realm_post(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_post(self.realm, group_id, body)
    }

    /// Delete realm-level role mappings
    pub fn groups_with_group_id_role_mappings_realm_delete(
        &'a self,
        group_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_delete(self.realm, group_id, body)
    }

    /// Get realm-level roles that can be mapped
    pub fn groups_with_group_id_role_mappings_realm_available_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_role_mappings_realm_available_get(self.realm, group_id)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
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
    pub fn users_with_user_id_role_mappings_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_get(self.realm, user_id)
    }

    /// Get realm-level role mappings
    pub fn users_with_user_id_role_mappings_realm_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_get(self.realm, user_id)
    }

    /// Add realm-level role mappings to the user
    pub fn users_with_user_id_role_mappings_realm_post(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_post(self.realm, user_id, body)
    }

    /// Delete realm-level role mappings
    pub fn users_with_user_id_role_mappings_realm_delete(
        &'a self,
        user_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_delete(self.realm, user_id, body)
    }

    /// Get realm-level roles that can be mapped
    pub fn users_with_user_id_role_mappings_realm_available_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_role_mappings_realm_available_get(self.realm, user_id)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
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
