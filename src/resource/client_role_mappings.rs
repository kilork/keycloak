use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Role Mappings</h4>
    /// Get client-level role mappings for the user or group, and the app
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
