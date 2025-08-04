use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Clients</h4>
    /// Get clients belonging to the realm.
    ///
    /// If a client can’t be retrieved from the storage due to a problem with the underlying storage, it is silently removed from the returned list. This ensures that concurrent modifications to the list don’t prevent callers from retrieving this list.
    pub fn clients_get(&'a self) -> RealmClientsGet<'a, TS> {
        RealmClientsGet { realm_admin: self }
    }

    /// Create a new client Client’s client_id must be unique!
    pub fn clients_post(
        &'a self,
        body: ClientRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_clients_post(self.realm, body)
    }

    /// Get representation of the client
    pub fn clients_with_client_uuid_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_get(self.realm, client_uuid)
    }

    /// Update the client
    pub fn clients_with_client_uuid_put(
        &'a self,
        client_uuid: &'a str,
        body: ClientRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_put(self.realm, client_uuid, body)
    }

    /// Delete the client
    pub fn clients_with_client_uuid_delete(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_delete(self.realm, client_uuid)
    }

    /// Get the client secret
    pub fn clients_with_client_uuid_client_secret_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_get(self.realm, client_uuid)
    }

    /// Generate a new secret for the client
    pub fn clients_with_client_uuid_client_secret_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_post(self.realm, client_uuid)
    }

    /// Get the rotated client secret
    pub fn clients_with_client_uuid_client_secret_rotated_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<CredentialRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_rotated_get(self.realm, client_uuid)
    }

    /// Invalidate the rotated secret for the client
    pub fn clients_with_client_uuid_client_secret_rotated_delete(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_client_secret_rotated_delete(self.realm, client_uuid)
    }

    /// Get default client scopes.  Only name and ids are returned.
    pub fn clients_with_client_uuid_default_client_scopes_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_default_client_scopes_get(self.realm, client_uuid)
    }

    pub fn clients_with_client_uuid_default_client_scopes_with_client_scope_id_put(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_default_client_scopes_with_client_scope_id_put(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    pub fn clients_with_client_uuid_default_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_default_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    /// Create JSON with payload of example access token
    pub fn clients_with_client_uuid_evaluate_scopes_generate_example_access_token_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Create JSON with payload of example id token
    pub fn clients_with_client_uuid_evaluate_scopes_generate_example_id_token_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Create JSON with payload of example user info
    pub fn clients_with_client_uuid_evaluate_scopes_generate_example_userinfo_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Return list of all protocol mappers, which will be used when generating tokens issued for particular client.
    ///
    /// This means protocol mappers assigned to this client directly and protocol mappers assigned to all client scopes of this client.
    pub fn clients_with_client_uuid_evaluate_scopes_protocol_mappers_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS> {
        RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Get effective scope mapping of all roles of particular role container, which this client is defacto allowed to have in the accessToken issued for him.
    ///
    /// This contains scope mappings, which this client has directly, as well as scope mappings, which are granted to all client scopes, which are linked with this client.
    pub fn clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
        &'a self,
        client_uuid: &'a str,
        role_container_id: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
    {
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet {
            realm_admin: self,
            client_uuid,
            role_container_id,
        }
    }

    /// Get roles, which this client doesn't have scope for and can't have them in the accessToken issued for him.
    ///
    /// Defacto all the other roles of particular role container, which are not in {@link #getGrantedScopeMappings()}
    pub fn clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
        &'a self,
        client_uuid: &'a str,
        role_container_id: &'a str,
    ) -> RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<'a, TS>
    {
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet {
            realm_admin: self,
            client_uuid,
            role_container_id,
        }
    }

    pub fn clients_with_client_uuid_installation_providers_with_provider_id_get(
        &'a self,
        client_uuid: &'a str,
        provider_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_installation_providers_with_provider_id_get(
                self.realm,
                client_uuid,
                provider_id,
            )
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    pub fn clients_with_client_uuid_management_permissions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_management_permissions_get(self.realm, client_uuid)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    pub fn clients_with_client_uuid_management_permissions_put(
        &'a self,
        client_uuid: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_management_permissions_put(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Register a cluster node with the client Manually register cluster node to this client - usually it’s not needed to call this directly as adapter should handle by sending registration request to Keycloak
    pub fn clients_with_client_uuid_nodes_post(
        &'a self,
        client_uuid: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_nodes_post(self.realm, client_uuid, body)
    }

    /// Unregister a cluster node from the client
    pub fn clients_with_client_uuid_nodes_with_node_delete(
        &'a self,
        client_uuid: &'a str,
        node: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_nodes_with_node_delete(self.realm, client_uuid, node)
    }

    /// Get application offline session count Returns a number of offline user sessions associated with this client { "count": number }
    pub fn clients_with_client_uuid_offline_session_count_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, i64>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_offline_session_count_get(self.realm, client_uuid)
    }

    /// Get offline sessions for client Returns a list of offline user sessions associated with this client
    pub fn clients_with_client_uuid_offline_sessions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidOfflineSessionsGet<'a, TS> {
        RealmClientsWithClientUuidOfflineSessionsGet {
            realm_admin: self,
            client_uuid,
        }
    }

    /// Get optional client scopes.  Only name and ids are returned.
    pub fn clients_with_client_uuid_optional_client_scopes_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_optional_client_scopes_get(self.realm, client_uuid)
    }

    pub fn clients_with_client_uuid_optional_client_scopes_with_client_scope_id_put(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_optional_client_scopes_with_client_scope_id_put(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    pub fn clients_with_client_uuid_optional_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_optional_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_uuid,
                client_scope_id,
            )
    }

    /// Push the client's revocation policy to its admin URL If the client has an admin URL, push revocation policy to it.
    pub fn clients_with_client_uuid_push_revocation_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_push_revocation_post(self.realm, client_uuid)
    }

    /// Generate a new registration access token for the client
    pub fn clients_with_client_uuid_registration_access_token_post(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_registration_access_token_post(self.realm, client_uuid)
    }

    /// Get a user dedicated to the service account
    pub fn clients_with_client_uuid_service_account_user_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<UserRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_service_account_user_get(self.realm, client_uuid)
    }

    /// Get application session count Returns a number of user sessions associated with this client { "count": number }
    pub fn clients_with_client_uuid_session_count_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, i64>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_session_count_get(self.realm, client_uuid)
    }

    /// Test if registered cluster nodes are available Tests availability by sending 'ping' request to all cluster nodes.
    pub fn clients_with_client_uuid_test_nodes_available_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_test_nodes_available_get(self.realm, client_uuid)
    }

    /// Get user sessions for client Returns a list of user sessions associated with this client
    ///
    pub fn clients_with_client_uuid_user_sessions_get(
        &'a self,
        client_uuid: &'a str,
    ) -> RealmClientsWithClientUuidUserSessionsGet<'a, TS> {
        RealmClientsWithClientUuidUserSessionsGet {
            realm_admin: self,
            client_uuid,
        }
    }
}

// <h4>Clients</h4>
pub struct RealmClientsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmClientsGetArgs {
    /// filter by clientId
    pub client_id: Option<String>,
    /// the first result
    pub first: Option<i32>,
    /// the max results to return
    pub max: Option<i32>,
    pub q: Option<String>,
    /// whether this is a search query or a getClientById query
    pub search: Option<bool>,
    /// filter clients that cannot be viewed in full by admin
    pub viewable_only: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmClientsGet<'a, TS> {
    type Output = TypeVec<ClientRepresentation>;
    type Args = RealmClientsGetArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            first,
            max,
            q,
            search,
            viewable_only,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_clients_get(
            self.realm_admin.realm,
            client_id,
            first,
            max,
            q,
            search,
            viewable_only,
        )
    }
}

impl<'a, TS> IntoFuture for RealmClientsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ClientRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGetArgs {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub user_id: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS>
{
    type Output = AccessToken;
    type Args = RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGetArgs;

    fn opts(
        self,
        Self::Args {
            audience,
            scope,
            user_id,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_generate_example_access_token_get(
                self.realm_admin.realm,
                self.client_uuid,
                audience,
                scope,
                user_id,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<AccessToken, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGetArgs {
    pub audience: Option<String>,
    pub scope: Option<String>,
    pub user_id: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS>
{
    type Output = IDToken;
    type Args = RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGetArgs;

    fn opts(
        self,
        Self::Args {
            audience,
            scope,
            user_id,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_generate_example_id_token_get(
                self.realm_admin.realm,
                self.client_uuid,
                audience,
                scope,
                user_id,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<IDToken, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGetArgs {
    pub scope: Option<String>,
    pub user_id: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS>
{
    type Output = Value;
    type Args = RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGetArgs;

    fn opts(
        self,
        Self::Args { scope, user_id }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_generate_example_userinfo_get(
                self.realm_admin.realm,
                self.client_uuid,
                scope,
                user_id,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<Value, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS: KeycloakTokenSupplier>
{
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesProtocolMappersGetArgs {
    pub scope: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS>
{
    type Output = TypeVec<ProtocolMapperEvaluationRepresentation>;
    type Args = RealmClientsWithClientUuidEvaluateScopesProtocolMappersGetArgs;

    fn opts(
        self,
        Self::Args { scope }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_protocol_mappers_get(
                self.realm_admin.realm,
                self.client_uuid,
                scope,
            )
    }
}

impl<'a, TS> IntoFuture for RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ProtocolMapperEvaluationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// either realm name OR client UUID
    pub role_container_id: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGetArgs {
    pub scope: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
{
    type Output = TypeVec<RoleRepresentation>;
    type Args =
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGetArgs;

    fn opts(
        self,
        Self::Args { scope }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_granted_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.role_container_id,
                scope,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
    /// either realm name OR client UUID
    pub role_container_id: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGetArgs
{
    pub scope: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
        'a,
        TS,
    >
{
    type Output = TypeVec<RoleRepresentation>;
    type Args =
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGetArgs;

    fn opts(
        self,
        Self::Args { scope }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_evaluate_scopes_scope_mappings_with_role_container_id_not_granted_get(
                self.realm_admin.realm,
                self.client_uuid,
                self.role_container_id,
                scope,
            )
    }
}

impl<'a, TS> IntoFuture
    for RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
        'a,
        TS,
    >
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<RoleRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidOfflineSessionsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidOfflineSessionsGetArgs {
    /// Paging offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidOfflineSessionsGet<'a, TS>
{
    type Output = TypeVec<UserSessionRepresentation>;
    type Args = RealmClientsWithClientUuidOfflineSessionsGetArgs;

    fn opts(
        self,
        Self::Args { first, max }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_offline_sessions_get(
                self.realm_admin.realm,
                self.client_uuid,
                first,
                max,
            )
    }
}

impl<'a, TS> IntoFuture for RealmClientsWithClientUuidOfflineSessionsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientsWithClientUuidUserSessionsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// id of client (not client-id!)
    pub client_uuid: &'a str,
}

#[derive(Default)]
pub struct RealmClientsWithClientUuidUserSessionsGetArgs {
    /// Paging offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientsWithClientUuidUserSessionsGet<'a, TS>
{
    type Output = TypeVec<UserSessionRepresentation>;
    type Args = RealmClientsWithClientUuidUserSessionsGetArgs;

    fn opts(
        self,
        Self::Args { first, max }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_clients_with_client_uuid_user_sessions_get(
                self.realm_admin.realm,
                self.client_uuid,
                first,
                max,
            )
    }
}

impl<'a, TS> IntoFuture for RealmClientsWithClientUuidUserSessionsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Clients</h4>
    impl<'a, TS> RealmClientsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// filter by clientId
        pub fn client_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().client_id(value)
        }
        /// the first result
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// the max results to return
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// whether this is a search query or a getClientById query
        pub fn search(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// filter clients that cannot be viewed in full by admin
        pub fn viewable_only(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().viewable_only(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// filter by clientId
        pub fn client_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.client_id = value.into();
            self
        }
        /// the first result
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// the max results to return
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// whether this is a search query or a getClientById query
        pub fn search(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// filter clients that cannot be viewed in full by admin
        pub fn viewable_only(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.viewable_only = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn audience(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().audience(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn user_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().user_id(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidEvaluateScopesGenerateExampleAccessTokenGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn audience(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.audience = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn user_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.user_id = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn audience(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().audience(value)
        }
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn user_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().user_id(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidEvaluateScopesGenerateExampleIdTokenGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn audience(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.audience = value.into();
            self
        }
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn user_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.user_id = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
        pub fn user_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().user_id(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidEvaluateScopesGenerateExampleUserinfoGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
        pub fn user_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.user_id = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidEvaluateScopesProtocolMappersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
    }

    impl<'a, TS>
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdGrantedGet<
                '_,
                TS,
            >,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
    }

    impl<'a, TS>
        RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
            'a,
            TS,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().scope(value)
        }
    }

    impl<TS>
        Builder<
            '_,
            RealmClientsWithClientUuidEvaluateScopesScopeMappingsWithRoleContainerIdNotGrantedGet<
                '_,
                TS,
            >,
        >
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn scope(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.scope = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidOfflineSessionsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Paging offset
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidOfflineSessionsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Paging offset
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientsWithClientUuidUserSessionsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Paging offset
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmClientsWithClientUuidUserSessionsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Paging offset
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }
}
