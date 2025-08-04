use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Roles (by ID)</h4>
    /// Get a specific role's representation
    pub fn roles_by_id_with_role_id_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<RoleRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_get(self.realm, role_id)
    }

    /// Update the role
    pub fn roles_by_id_with_role_id_put(
        &'a self,
        role_id: &'a str,
        body: RoleRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_put(self.realm, role_id, body)
    }

    /// Delete the role
    pub fn roles_by_id_with_role_id_delete(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_delete(self.realm, role_id)
    }

    /// Get role's children Returns a set of role's children provided the role is a composite.
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
    pub fn roles_by_id_with_role_id_composites_post(
        &'a self,
        role_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_composites_post(self.realm, role_id, body)
    }

    /// Remove a set of roles from the role's composite
    pub fn roles_by_id_with_role_id_composites_delete(
        &'a self,
        role_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_roles_by_id_with_role_id_composites_delete(self.realm, role_id, body)
    }

    /// Get client-level roles for the client that are in the role's composite
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
    pub fn roles_by_id_with_role_id_composites_realm_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_composites_realm_get(self.realm, role_id)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    pub fn roles_by_id_with_role_id_management_permissions_get(
        &'a self,
        role_id: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_management_permissions_get(self.realm, role_id)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    pub fn roles_by_id_with_role_id_management_permissions_put(
        &'a self,
        role_id: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_roles_by_id_with_role_id_management_permissions_put(self.realm, role_id, body)
    }
}

// <h4>Roles (by ID)</h4>
pub struct RealmRolesByIdWithRoleIdCompositesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub role_id: &'a str,
}

#[derive(Default)]
pub struct RealmRolesByIdWithRoleIdCompositesGetArgs {
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

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

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Roles (by ID)</h4>
    impl<'a, TS> RealmRolesByIdWithRoleIdCompositesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
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

    impl<TS> Builder<'_, RealmRolesByIdWithRoleIdCompositesGet<'_, TS>>
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
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }
}
