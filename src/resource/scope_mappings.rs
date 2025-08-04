use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Scope Mappings</h4>
    /// Get all scope mappings for the client
    #[deprecated]
    pub fn client_scopes_with_client_scope_id_scope_mappings_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Add client-level roles to the client's scope
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_post(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_post(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// Remove client-level roles from the client's scope.
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_delete(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_delete(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_available_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_available_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    pub fn client_scopes_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS> {
        RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet {
            realm_admin: self,
            client_scope_id,
            client,
        }
    }

    /// Get realm-level roles associated with the client's scope
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Add a set of realm-level roles to the client's scope
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Remove a set of realm-level roles from the client's scope
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_delete(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_delete(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get realm-level roles that are available to attach to this client's scope
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_available_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_available_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really to show a comprehensive total view of realm-level roles associated with the client.
    pub fn client_scopes_with_client_scope_id_scope_mappings_realm_composite_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS> {
        RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet {
            realm_admin: self,
            client_scope_id,
        }
    }

    /// Get all scope mappings for the client
    #[deprecated]
    pub fn client_templates_with_client_scope_id_scope_mappings_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Add client-level roles to the client's scope
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_post(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_post(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// Remove client-level roles from the client's scope.
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_delete(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_delete(
                self.realm,
                client_scope_id,
                client,
                body,
            )
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_available_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_available_get(
                self.realm,
                client_scope_id,
                client,
            )
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    pub fn client_templates_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
        &'a self,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
    {
        RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet {
            realm_admin: self,
            client_scope_id,
            client,
        }
    }

    /// Get realm-level roles associated with the client's scope
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Add a set of realm-level roles to the client's scope
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Remove a set of realm-level roles from the client's scope
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_delete(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_delete(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get realm-level roles that are available to attach to this client's scope
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_available_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_available_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really to show a comprehensive total view of realm-level roles associated with the client.
    pub fn client_templates_with_client_scope_id_scope_mappings_realm_composite_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS> {
        RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet {
            realm_admin: self,
            client_scope_id,
        }
    }

    /// Get all scope mappings for the client
    #[deprecated]
    pub fn clients_with_client_uuid_scope_mappings_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<MappingsRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_get(self.realm, client_uuid)
    }

    /// Get the roles associated with a client's scope Returns roles for the client.
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_get(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_get(
                self.realm,
                client_uuid,
                client,
            )
    }

    /// Add client-level roles to the client's scope
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_post(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_post(
                self.realm,
                client_uuid,
                client,
                body,
            )
    }

    /// Remove client-level roles from the client's scope.
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_delete(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_delete(
                self.realm,
                client_uuid,
                client,
                body,
            )
    }

    /// The available client-level roles Returns the roles for the client that can be associated with the client's scope
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_available_get(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_available_get(
                self.realm,
                client_uuid,
                client,
            )
    }

    /// Get effective client roles Returns the roles for the client that are associated with the client's scope.
    pub fn clients_with_client_uuid_scope_mappings_clients_with_client_composite_get(
        &'a self,
        client_uuid: &'a str,
        client: &'a str,
    ) -> RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS> {
        RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet {
            realm_admin: self,
            client_uuid,
            client,
        }
    }

    /// Get realm-level roles associated with the client's scope
    pub fn clients_with_client_uuid_scope_mappings_realm_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_get(self.realm, client_uuid)
    }

    /// Add a set of realm-level roles to the client's scope
    pub fn clients_with_client_uuid_scope_mappings_realm_post(
        &'a self,
        client_uuid: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_post(self.realm, client_uuid, body)
    }

    /// Remove a set of realm-level roles from the client's scope
    pub fn clients_with_client_uuid_scope_mappings_realm_delete(
        &'a self,
        client_uuid: &'a str,
        body: Vec<RoleRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_delete(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Get realm-level roles that are available to attach to this client's scope
    pub fn clients_with_client_uuid_scope_mappings_realm_available_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<RoleRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_scope_mappings_realm_available_get(
                self.realm,
                client_uuid,
            )
    }

    /// Get effective realm-level roles associated with the client’s scope What this does is recurse any composite roles associated with the client’s scope and adds the roles to this lists.
    ///
    /// The method is really to show a comprehensive total view of realm-level roles associated with the client.
    pub fn clients_with_client_uuid_scope_mappings_realm_composite_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS> {
        RealmClientsWithClientUuidScopeMappingsRealmCompositeGet {
            realm_admin: self,
            client_uuid,
        }
    }
}

// <h4>Scope Mappings</h4>
pub struct RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
    pub client: &'a str,
}

#[derive(Default)]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                self.client,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
}

#[derive(Default)]
pub struct RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_scopes_with_client_scope_id_scope_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture for RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
    pub client: &'a str,
}

#[derive(Default)]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_templates_with_client_scope_id_scope_mappings_clients_with_client_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                self.client,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub client_scope_id: &'a str,
}

#[derive(Default)]
pub struct RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_templates_with_client_scope_id_scope_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.client_scope_id,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    pub client: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_scope_mappings_clients_with_client_composite_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.client,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidScopeMappingsRealmCompositeGetArgs {
    /// if false, return roles with their attributes
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args = RealmClientsWithClientUuidScopeMappingsRealmCompositeGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_scope_mappings_realm_composite_get(
                self.realm_admin.realm,
                self.client_uuid,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture for RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS>
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

    // <h4>Scope Mappings</h4>
    impl<'a, TS> RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientScopesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'_, TS>,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmClientScopesWithClientScopeIdScopeMappingsRealmCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientTemplatesWithClientScopeIdScopeMappingsClientsWithClientCompositeGet<'_, TS>,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmClientTemplatesWithClientScopeIdScopeMappingsRealmCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidScopeMappingsClientsWithClientCompositeGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// if false, return roles with their attributes
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidScopeMappingsRealmCompositeGet<'_, TS>>
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
