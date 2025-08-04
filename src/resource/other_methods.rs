use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>default</h4>
    pub fn clients_with_client_uuid_authz_resource_server_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ResourceServerRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_get(self.realm, client_uuid)
    }

    pub fn clients_with_client_uuid_authz_resource_server_put(
        &'a self,
        client_uuid: &'a str,
        body: ResourceServerRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_authz_resource_server_put(self.realm, client_uuid, body)
    }

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

    pub fn clients_with_client_uuid_authz_resource_server_permission_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPermissionGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPermissionGet {
            realm_admin: self,
            client_uuid,
        }
    }

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

    pub fn clients_with_client_uuid_authz_resource_server_permission_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPermissionSearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

    pub fn clients_with_client_uuid_authz_resource_server_policy_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPolicyGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPolicyGet {
            realm_admin: self,
            client_uuid,
        }
    }

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

    pub fn clients_with_client_uuid_authz_resource_server_policy_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerPolicySearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

    pub fn clients_with_client_uuid_authz_resource_server_resource_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceGet {
            realm_admin: self,
            client_uuid,
        }
    }

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

    pub fn clients_with_client_uuid_authz_resource_server_resource_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerResourceSearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

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

    pub fn clients_with_client_uuid_authz_resource_server_scope_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerScopeGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerScopeGet {
            realm_admin: self,
            client_uuid,
        }
    }

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

    pub fn clients_with_client_uuid_authz_resource_server_scope_search_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet<'a, TS> {
        RealmClientsWithClientUuidAuthzResourceServerScopeSearchGet {
            realm_admin: self,
            client_uuid,
        }
    }

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
