use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Realms Admin</h4>
    /// Get the top-level representation of the realm It will not include nested information like User and Client representations.
    pub fn get(
        &'a self,
    ) -> impl Future<Output = Result<RealmRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_get(self.realm)
    }

    /// Update the top-level information of the realm Any user, roles or client information in the representation will be ignored.
    ///
    /// This will only update top-level attributes of the realm.
    pub fn put(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_put(self.realm, body)
    }

    /// Delete the realm
    pub fn delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_delete(self.realm)
    }

    /// Get admin events Returns all admin events, or filters events based on URL query parameters listed here
    pub fn admin_events_get(&'a self) -> RealmAdminEventsGet<'a, TS> {
        RealmAdminEventsGet { realm_admin: self }
    }

    /// Delete all admin events
    pub fn admin_events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_admin_events_delete(self.realm)
    }

    /// Base path for importing clients under this realm.
    pub fn client_description_converter_post(
        &'a self,
        body: String,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_description_converter_post(self.realm, body)
    }

    pub fn client_policies_policies_get(&'a self) -> RealmClientPoliciesPoliciesGet<'a, TS> {
        RealmClientPoliciesPoliciesGet { realm_admin: self }
    }

    pub fn client_policies_policies_put(
        &'a self,
        body: ClientPoliciesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_policies_policies_put(self.realm, body)
    }

    pub fn client_policies_profiles_get(&'a self) -> RealmClientPoliciesProfilesGet<'a, TS> {
        RealmClientPoliciesProfilesGet { realm_admin: self }
    }

    pub fn client_policies_profiles_put(
        &'a self,
        body: ClientProfilesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_policies_profiles_put(self.realm, body)
    }

    /// Get client session stats Returns a JSON map.
    ///
    /// The key is the client id, the value is the number of sessions that currently are active with that client. Only clients that actually have a session associated with them will be in this map.
    pub fn client_session_stats_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, String>>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_session_stats_get(self.realm)
    }

    /// List all client types available in the current realm
    ///
    /// This endpoint returns a list of both global and realm level client types and the attributes they set
    pub fn client_types_get(
        &'a self,
    ) -> impl Future<Output = Result<ClientTypesRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_get(self.realm)
    }

    /// Update a client type
    ///
    /// This endpoint allows you to update a realm level client type
    pub fn client_types_put(
        &'a self,
        body: ClientTypesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_put(self.realm, body)
    }

    pub fn credential_registrators_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin.realm_credential_registrators_get(self.realm)
    }

    /// Get realm default client scopes. Only name and ids are returned.
    pub fn default_default_client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_default_default_client_scopes_get(self.realm)
    }

    pub fn default_default_client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_default_client_scopes_with_client_scope_id_put(
                self.realm,
                client_scope_id,
            )
    }

    pub fn default_default_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_default_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_scope_id,
            )
    }

    /// Get group hierarchy.  Only name and ids are returned.
    pub fn default_groups_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<GroupRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_default_groups_get(self.realm)
    }

    pub fn default_groups_with_group_id_put(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_groups_with_group_id_put(self.realm, group_id)
    }

    pub fn default_groups_with_group_id_delete(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_groups_with_group_id_delete(self.realm, group_id)
    }

    /// Get realm optional client scopes. Only name and ids are returned.
    pub fn default_optional_client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_default_optional_client_scopes_get(self.realm)
    }

    pub fn default_optional_client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_optional_client_scopes_with_client_scope_id_put(
                self.realm,
                client_scope_id,
            )
    }

    pub fn default_optional_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_optional_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_scope_id,
            )
    }

    /// Get events Returns all events, or filters them based on URL query parameters listed here
    pub fn events_get(&'a self) -> RealmEventsGet<'a, TS> {
        RealmEventsGet { realm_admin: self }
    }

    /// Delete all events
    pub fn events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_delete(self.realm)
    }

    /// Get the events provider configuration Returns JSON object with events provider configuration
    pub fn events_config_get(
        &'a self,
    ) -> impl Future<Output = Result<RealmEventsConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_events_config_get(self.realm)
    }

    /// Update the events provider Change the events provider and/or its configuration
    pub fn events_config_put(
        &'a self,
        body: RealmEventsConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_config_put(self.realm, body)
    }

    pub fn group_by_path_with_path_get(
        &'a self,
        path: &'a str,
    ) -> impl Future<Output = Result<GroupRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_group_by_path_with_path_get(self.realm, path)
    }

    pub fn localization_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin.realm_localization_get(self.realm)
    }

    pub fn localization_with_locale_get(
        &'a self,
        locale: &'a str,
    ) -> RealmLocalizationWithLocaleGet<'a, TS> {
        RealmLocalizationWithLocaleGet {
            realm_admin: self,
            locale,
        }
    }

    /// Import localization from uploaded JSON file
    pub fn localization_with_locale_post(
        &'a self,
        locale: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_post(self.realm, locale, body)
    }

    pub fn localization_with_locale_delete(
        &'a self,
        locale: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_delete(self.realm, locale)
    }

    pub fn localization_with_locale_with_key_get(
        &'a self,
        key: &'a str,
        locale: &'a str,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_get(self.realm, key, locale)
    }

    pub fn localization_with_locale_with_key_put(
        &'a self,
        key: &'a str,
        locale: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_put(self.realm, key, locale, body)
    }

    pub fn localization_with_locale_with_key_delete(
        &'a self,
        key: &'a str,
        locale: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_delete(self.realm, key, locale)
    }

    /// Removes all user sessions.
    ///
    /// Any client that has an admin url will also be told to invalidate any sessions they have.
    pub fn logout_all_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_logout_all_post(self.realm)
    }

    /// Partial export of existing realm into a JSON file.
    pub fn partial_export_post(&'a self) -> RealmPartialExportPost<'a, TS> {
        RealmPartialExportPost { realm_admin: self }
    }

    /// Partial import from a JSON file to an existing realm.
    pub fn partial_import_post(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<Value, KeycloakError>> + use<'a, TS> {
        self.admin.realm_partial_import_post(self.realm, body)
    }

    /// Push the realm's revocation policy to any client that has an admin url associated with it.
    pub fn push_revocation_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_push_revocation_post(self.realm)
    }

    /// Remove a specific user session.
    ///
    /// Any client that has an admin url will also be told to invalidate this particular session.
    pub fn sessions_with_session_delete(
        &'a self,
        session: &'a str,
    ) -> RealmSessionsWithSessionDelete<'a, TS> {
        RealmSessionsWithSessionDelete {
            realm_admin: self,
            session,
        }
    }

    /// Test SMTP connection with current logged in user
    #[deprecated]
    pub fn test_smtp_connection_post(
        &'a self,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_test_smtp_connection_post(self.realm, body)
    }

    pub fn users_management_permissions_get(
        &'a self,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_management_permissions_get(self.realm)
    }

    pub fn users_management_permissions_put(
        &'a self,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_management_permissions_put(self.realm, body)
    }
}

// <h4>Realms Admin</h4>
pub struct RealmAdminEventsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmAdminEventsGetArgs {
    pub auth_client: Option<String>,
    pub auth_ip_address: Option<String>,
    pub auth_realm: Option<String>,
    /// user id
    pub auth_user: Option<String>,
    /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_from: Option<String>,
    /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_to: Option<String>,
    /// The direction to sort events by (asc or desc)
    pub direction: Option<String>,
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    pub operation_types: Option<Vec<String>>,
    pub resource_path: Option<String>,
    pub resource_types: Option<Vec<String>>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmAdminEventsGet<'a, TS> {
    type Output = TypeVec<AdminEventRepresentation>;
    type Args = RealmAdminEventsGetArgs;

    fn opts(
        self,
        Self::Args {
            auth_client,
            auth_ip_address,
            auth_realm,
            auth_user,
            date_from,
            date_to,
            direction,
            first,
            max,
            operation_types,
            resource_path,
            resource_types,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_admin_events_get(
            self.realm_admin.realm,
            auth_client,
            auth_ip_address,
            auth_realm,
            auth_user,
            date_from,
            date_to,
            direction,
            first,
            max,
            operation_types,
            resource_path,
            resource_types,
        )
    }
}

impl<'a, TS> IntoFuture for RealmAdminEventsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<AdminEventRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientPoliciesPoliciesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmClientPoliciesPoliciesGetArgs {
    pub include_global_policies: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientPoliciesPoliciesGet<'a, TS>
{
    type Output = ClientPoliciesRepresentation;
    type Args = RealmClientPoliciesPoliciesGetArgs;

    fn opts(
        self,
        Self::Args {
            include_global_policies,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_policies_policies_get(self.realm_admin.realm, include_global_policies)
    }
}

impl<'a, TS> IntoFuture for RealmClientPoliciesPoliciesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ClientPoliciesRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientPoliciesProfilesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmClientPoliciesProfilesGetArgs {
    pub include_global_profiles: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientPoliciesProfilesGet<'a, TS>
{
    type Output = ClientProfilesRepresentation;
    type Args = RealmClientPoliciesProfilesGetArgs;

    fn opts(
        self,
        Self::Args {
            include_global_profiles,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_policies_profiles_get(self.realm_admin.realm, include_global_profiles)
    }
}

impl<'a, TS> IntoFuture for RealmClientPoliciesProfilesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ClientProfilesRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmEventsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmEventsGetArgs {
    /// App or oauth client name
    pub client: Option<String>,
    /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_from: Option<String>,
    /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_to: Option<String>,
    /// The direction to sort events by (asc or desc)
    pub direction: Option<String>,
    /// Paging offset
    pub first: Option<i32>,
    /// IP Address
    pub ip_address: Option<String>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    /// The types of events to return
    pub type_: Option<Vec<String>>,
    /// User id
    pub user: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmEventsGet<'a, TS> {
    type Output = TypeVec<EventRepresentation>;
    type Args = RealmEventsGetArgs;

    fn opts(
        self,
        Self::Args {
            client,
            date_from,
            date_to,
            direction,
            first,
            ip_address,
            max,
            type_,
            user,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_events_get(
            self.realm_admin.realm,
            client,
            date_from,
            date_to,
            direction,
            first,
            ip_address,
            max,
            type_,
            user,
        )
    }
}

impl<'a, TS> IntoFuture for RealmEventsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<EventRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmLocalizationWithLocaleGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub locale: &'a str,
}

#[derive(Default)]
pub struct RealmLocalizationWithLocaleGetArgs {
    pub use_realm_default_locale_fallback: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmLocalizationWithLocaleGet<'a, TS>
{
    type Output = TypeMap<String, TypeString>;
    type Args = RealmLocalizationWithLocaleGetArgs;

    fn opts(
        self,
        Self::Args {
            use_realm_default_locale_fallback,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_localization_with_locale_get(
            self.realm_admin.realm,
            self.locale,
            use_realm_default_locale_fallback,
        )
    }
}

impl<'a, TS> IntoFuture for RealmLocalizationWithLocaleGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, TypeString>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmPartialExportPost<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmPartialExportPostArgs {
    pub export_clients: Option<bool>,
    pub export_groups_and_roles: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmPartialExportPost<'a, TS> {
    type Output = RealmRepresentation;
    type Args = RealmPartialExportPostArgs;

    fn opts(
        self,
        Self::Args {
            export_clients,
            export_groups_and_roles,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_partial_export_post(
            self.realm_admin.realm,
            export_clients,
            export_groups_and_roles,
        )
    }
}

impl<'a, TS> IntoFuture for RealmPartialExportPost<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<RealmRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmSessionsWithSessionDelete<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub session: &'a str,
}

#[derive(Default)]
pub struct RealmSessionsWithSessionDeleteArgs {
    pub is_offline: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmSessionsWithSessionDelete<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmSessionsWithSessionDeleteArgs;

    fn opts(
        self,
        Self::Args { is_offline }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_sessions_with_session_delete(
            self.realm_admin.realm,
            self.session,
            is_offline,
        )
    }
}

impl<'a, TS> IntoFuture for RealmSessionsWithSessionDelete<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}
