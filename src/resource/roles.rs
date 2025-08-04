use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Roles</h4>
    /// Get all roles for the realm or client
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
    pub fn clients_with_client_uuid_roles_post(
        &'a self,
        client_uuid: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_roles_post(self.realm, client_uuid, body)
    }

    /// Get a role by name
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
    pub fn roles_get(&'a self) -> RealmRolesGet<'a, TS> {
        RealmRolesGet { realm_admin: self }
    }

    /// Create a new role for the realm or client
    pub fn roles_post(
        &'a self,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_roles_post(self.realm, body)
    }

    /// Get a role by name
    pub fn roles_with_role_name_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_get(self.realm, role_name)
    }

    /// Update a role by name
    pub fn roles_with_role_name_put(
        &'a self,
        role_name: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_put(self.realm, role_name, body)
    }

    /// Delete a role by name
    pub fn roles_with_role_name_delete(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_delete(self.realm, role_name)
    }

    /// Get composites of the role
    pub fn roles_with_role_name_composites_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_get(self.realm, role_name)
    }

    /// Add a composite to the role
    pub fn roles_with_role_name_composites_post(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_post(self.realm, role_name, body)
    }

    /// Remove roles from the role's composite
    pub fn roles_with_role_name_composites_delete(
        &'a self,
        role_name: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_with_role_name_composites_delete(self.realm, role_name, body)
    }

    /// Get client-level roles for the client that are in the role's composite
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
    pub fn roles_with_role_name_composites_realm_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_composites_realm_get(self.realm, role_name)
    }

    /// Returns a stream of groups that have the specified role name
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
    pub fn roles_with_role_name_management_permissions_get(
        &'a self,
        role_name: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_with_role_name_management_permissions_get(self.realm, role_name)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
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
