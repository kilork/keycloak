use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Identity Providers</h4>
    /// Import identity provider from JSON body
    ///
    /// Import identity provider from uploaded JSON file
    pub fn identity_provider_import_config_post(
        &'a self,
        body: TypeMap<String, Value>,
    ) -> impl Future<Output = Result<TypeMap<String, TypeString>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_import_config_post(self.realm, body)
    }

    /// List identity providers
    pub fn identity_provider_instances_get(&'a self) -> RealmIdentityProviderInstancesGet<'a, TS> {
        RealmIdentityProviderInstancesGet { realm_admin: self }
    }

    /// Create a new identity provider
    pub fn identity_provider_instances_post(
        &'a self,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_post(self.realm, body)
    }

    /// Get the identity provider
    pub fn identity_provider_instances_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_get(self.realm, alias)
    }

    /// Update the identity provider
    pub fn identity_provider_instances_with_alias_put(
        &'a self,
        alias: &'a str,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_put(self.realm, alias, body)
    }

    /// Delete the identity provider
    pub fn identity_provider_instances_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_delete(self.realm, alias)
    }

    /// Export public broker configuration for identity provider
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
    pub fn identity_provider_instances_with_alias_mappers_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_get(self.realm, alias)
    }

    /// Add a mapper to identity provider
    pub fn identity_provider_instances_with_alias_mappers_post(
        &'a self,
        alias: &'a str,
        body: IdentityProviderMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_post(self.realm, alias, body)
    }

    /// Get mapper by id for the identity provider
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
    pub fn identity_provider_instances_with_alias_reload_keys_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<bool, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_reload_keys_get(self.realm, alias)
    }

    /// Get the identity provider factory for that provider id
    pub fn identity_provider_providers_with_provider_id_get(
        &'a self,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_providers_with_provider_id_get(self.realm, provider_id)
    }
}

// <h4>Identity Providers</h4>
pub struct RealmIdentityProviderInstancesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

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

pub struct RealmIdentityProviderInstancesWithAliasExportGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub alias: &'a str,
}

#[derive(Default)]
pub struct RealmIdentityProviderInstancesWithAliasExportGetArgs {
    /// Format to use
    pub format: Option<String>,
}

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
