use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Identity Providers</h4>
    /// Import identity provider from JSON body
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/import-config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_post_adminrealmsrealmidentity_providerimport_config>
    pub fn identity_provider_import_config_post(
        &'a self,
        body: TypeMap<String, Value>,
    ) -> impl Future<Output = Result<TypeMap<String, TypeString>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_import_config_post(self.realm, body)
    }

    /// List identity providers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    /// - `realm_only`: Boolean which defines if only realm-level IDPs (not associated with orgs) should be returned (default: false)
    /// - `search`: Filter specific providers by name. Search can be prefix (name*), contains (*name*) or exact ("name"). Default prefixed.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstances>
    pub fn identity_provider_instances_get(&'a self) -> RealmIdentityProviderInstancesGet<'a, TS> {
        RealmIdentityProviderInstancesGet { realm_admin: self }
    }

    /// Create a new identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstances>
    pub fn identity_provider_instances_post(
        &'a self,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_post(self.realm, body)
    }

    /// Get the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesalias>
    pub fn identity_provider_instances_with_alias_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_identity_provider_instances_with_alias_get(self.realm, alias)
    }

    /// Update the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesalias>
    pub fn identity_provider_instances_with_alias_put(
        &'a self,
        alias: &'a str,
        body: IdentityProviderRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_put(self.realm, alias, body)
    }

    /// Delete the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesalias>
    pub fn identity_provider_instances_with_alias_delete(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_delete(self.realm, alias)
    }

    /// Export public broker configuration for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `format`: Format to use
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasexport>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmapper_types>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappers>
    pub fn identity_provider_instances_with_alias_mappers_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_get(self.realm, alias)
    }

    /// Add a mapper to identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstancesaliasmappers>
    pub fn identity_provider_instances_with_alias_mappers_post(
        &'a self,
        alias: &'a str,
        body: IdentityProviderMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_mappers_post(self.realm, alias, body)
    }

    /// Get mapper by id for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappersid>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmappersid>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesaliasmappersid>
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
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/reload-keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasreload_keys>
    pub fn identity_provider_instances_with_alias_reload_keys_get(
        &'a self,
        alias: &'a str,
    ) -> impl Future<Output = Result<bool, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_identity_provider_instances_with_alias_reload_keys_get(self.realm, alias)
    }

    /// Get the identity provider factory for that provider id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `provider_id`: The provider id to get the factory
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.1/rest-api/index.html#_get_adminrealmsrealmidentity_providerprovidersprovider_id>
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

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Identity Providers</h4>
    impl<'a, TS> RealmIdentityProviderInstancesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// Pagination offset
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// Boolean which defines if only realm-level IDPs (not associated with orgs) should be returned (default: false)
        pub fn realm_only(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().realm_only(value)
        }
        /// Filter specific providers by name. Search can be prefix (name*), contains (*name*) or exact ("name"). Default prefixed.
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmIdentityProviderInstancesGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// Pagination offset
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// Boolean which defines if only realm-level IDPs (not associated with orgs) should be returned (default: false)
        pub fn realm_only(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.realm_only = value.into();
            self
        }
        /// Filter specific providers by name. Search can be prefix (name*), contains (*name*) or exact ("name"). Default prefixed.
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmIdentityProviderInstancesWithAliasExportGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Format to use
        pub fn format(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().format(value)
        }
    }

    impl<TS> Builder<'_, RealmIdentityProviderInstancesWithAliasExportGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Format to use
        pub fn format(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.format = value.into();
            self
        }
    }
}
