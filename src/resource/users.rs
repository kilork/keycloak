use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Users</h4>
    /// Get users Returns a stream of users, filtered according to query parameters.
    pub fn users_get(&'a self) -> RealmUsersGet<'a, TS> {
        RealmUsersGet { realm_admin: self }
    }

    /// Create a new user Username must be unique.
    pub fn users_post(
        &'a self,
        body: UserRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_post(self.realm, body)
    }

    /// Returns the number of users that match the given criteria.
    ///
    /// It can be called in three different ways. 1. Don’t specify any criteria and pass {@code null}. The number of all users within that realm will be returned. <p> 2. If {@code search} is specified other criteria such as {@code last} will be ignored even though you set them. The {@code search} string will be matched against the first and last name, the username and the email of a user. <p> 3. If {@code search} is unspecified but any of {@code last}, {@code first}, {@code email} or {@code username} those criteria are matched against their respective fields on a user entity. Combined with a logical and.
    pub fn users_count_get(&'a self) -> RealmUsersCountGet<'a, TS> {
        RealmUsersCountGet { realm_admin: self }
    }

    /// Get the configuration for the user profile
    pub fn users_profile_get(
        &'a self,
    ) -> impl Future<Output = Result<UPConfig, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_get(self.realm)
    }

    /// Set the configuration for the user profile
    pub fn users_profile_put(
        &'a self,
        body: UPConfig,
    ) -> impl Future<Output = Result<UPConfig, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_put(self.realm, body)
    }

    /// Get the UserProfileMetadata from the configuration
    pub fn users_profile_metadata_get(
        &'a self,
    ) -> impl Future<Output = Result<UserProfileMetadata, KeycloakError>> + use<'a, TS> {
        self.admin.realm_users_profile_metadata_get(self.realm)
    }

    /// Get representation of the user
    pub fn users_with_user_id_get(&'a self, user_id: &'a str) -> RealmUsersWithUserIdGet<'a, TS> {
        RealmUsersWithUserIdGet {
            realm_admin: self,
            user_id,
        }
    }

    /// Update the user
    pub fn users_with_user_id_put(
        &'a self,
        user_id: &'a str,
        body: UserRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_put(self.realm, user_id, body)
    }

    /// Delete the user
    pub fn users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_delete(self.realm, user_id)
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    ///
    /// Returned values can contain for example "password", "otp" etc. This will always return empty list for "local" users, which are not backed by any user storage
    pub fn users_with_user_id_configured_user_storage_credential_types_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_configured_user_storage_credential_types_get(
                self.realm, user_id,
            )
    }

    /// Get consents granted by the user
    pub fn users_with_user_id_consents_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, Value>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_consents_get(self.realm, user_id)
    }

    /// Revoke consent and offline tokens for particular client from user
    pub fn users_with_user_id_consents_with_client_delete(
        &'a self,
        user_id: &'a str,
        client: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_consents_with_client_delete(self.realm, user_id, client)
    }

    pub fn users_with_user_id_credentials_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<CredentialRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_credentials_get(self.realm, user_id)
    }

    /// Remove a credential for a user
    pub fn users_with_user_id_credentials_with_credential_id_delete(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_delete(
                self.realm,
                user_id,
                credential_id,
            )
    }

    /// Move a credential to a position behind another credential
    pub fn users_with_user_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
        new_previous_credential_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
                self.realm,
                user_id,
                credential_id,
                new_previous_credential_id,
            )
    }

    /// Move a credential to a first position in the credentials list of the user
    pub fn users_with_user_id_credentials_with_credential_id_move_to_first_post(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_move_to_first_post(
                self.realm,
                user_id,
                credential_id,
            )
    }

    /// Update a credential label for a user
    pub fn users_with_user_id_credentials_with_credential_id_user_label_put(
        &'a self,
        user_id: &'a str,
        credential_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_credentials_with_credential_id_user_label_put(
                self.realm,
                user_id,
                credential_id,
                body,
            )
    }

    /// Disable all credentials for a user of a specific type
    pub fn users_with_user_id_disable_credential_types_put(
        &'a self,
        user_id: &'a str,
        body: Vec<String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_disable_credential_types_put(self.realm, user_id, body)
    }

    /// Send an email to the user with a link they can click to execute particular actions.
    ///
    /// An email contains a link the user can click to perform a set of required actions. The redirectUri and clientId parameters are optional. If no redirect is given, then there will be no link back to click after actions have completed. Redirect uri must be a valid uri for the particular clientId.
    pub fn users_with_user_id_execute_actions_email_put(
        &'a self,
        user_id: &'a str,
        body: Vec<String>,
    ) -> RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS> {
        RealmUsersWithUserIdExecuteActionsEmailPut {
            realm_admin: self,
            user_id,
            body,
        }
    }

    /// Get social logins associated with the user
    pub fn users_with_user_id_federated_identity_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<FederatedIdentityRepresentation>, KeycloakError>>
           + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_get(self.realm, user_id)
    }

    /// Add a social login provider to the user
    pub fn users_with_user_id_federated_identity_with_provider_post(
        &'a self,
        user_id: &'a str,
        provider: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_with_provider_post(
                self.realm, user_id, provider,
            )
    }

    /// Remove a social login provider from user
    pub fn users_with_user_id_federated_identity_with_provider_delete(
        &'a self,
        user_id: &'a str,
        provider: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_federated_identity_with_provider_delete(
                self.realm, user_id, provider,
            )
    }

    pub fn users_with_user_id_groups_get(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdGroupsGet<'a, TS> {
        RealmUsersWithUserIdGroupsGet {
            realm_admin: self,
            user_id,
        }
    }

    pub fn users_with_user_id_groups_count_get(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdGroupsCountGet<'a, TS> {
        RealmUsersWithUserIdGroupsCountGet {
            realm_admin: self,
            user_id,
        }
    }

    pub fn users_with_user_id_groups_with_group_id_put(
        &'a self,
        user_id: &'a str,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_groups_with_group_id_put(self.realm, user_id, group_id)
    }

    pub fn users_with_user_id_groups_with_group_id_delete(
        &'a self,
        user_id: &'a str,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_groups_with_group_id_delete(self.realm, user_id, group_id)
    }

    /// Impersonate the user
    pub fn users_with_user_id_impersonation_post(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_impersonation_post(self.realm, user_id)
    }

    /// Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    pub fn users_with_user_id_logout_post(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_logout_post(self.realm, user_id)
    }

    /// Get offline sessions associated with the user and client
    pub fn users_with_user_id_offline_sessions_with_client_uuid_get(
        &'a self,
        user_id: &'a str,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_offline_sessions_with_client_uuid_get(
                self.realm,
                user_id,
                client_uuid,
            )
    }

    /// Set up a new password for the user.
    pub fn users_with_user_id_reset_password_put(
        &'a self,
        user_id: &'a str,
        body: CredentialRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_users_with_user_id_reset_password_put(self.realm, user_id, body)
    }

    /// Send an email to the user with a link they can click to reset their password.
    ///
    /// The redirectUri and clientId parameters are optional. The default for the redirect is the account client. This endpoint has been deprecated.  Please use the execute-actions-email passing a list with UPDATE_PASSWORD within it.
    #[deprecated]
    pub fn users_with_user_id_reset_password_email_put(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdResetPasswordEmailPut<'a, TS> {
        RealmUsersWithUserIdResetPasswordEmailPut {
            realm_admin: self,
            user_id,
        }
    }

    /// Send an email-verification email to the user An email contains a link the user can click to verify their email address.
    ///
    /// The redirectUri, clientId and lifespan parameters are optional. The default for the redirect is the account client. The default for the lifespan is 12 hours
    pub fn users_with_user_id_send_verify_email_put(
        &'a self,
        user_id: &'a str,
    ) -> RealmUsersWithUserIdSendVerifyEmailPut<'a, TS> {
        RealmUsersWithUserIdSendVerifyEmailPut {
            realm_admin: self,
            user_id,
        }
    }

    /// Get sessions associated with the user
    pub fn users_with_user_id_sessions_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<UserSessionRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_sessions_get(self.realm, user_id)
    }

    pub fn users_with_user_id_unmanaged_attributes_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, TypeVec<String>>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_with_user_id_unmanaged_attributes_get(self.realm, user_id)
    }
}

// <h4>Users</h4>
pub struct RealmUsersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmUsersGetArgs {
    /// Boolean which defines whether brief representations are returned (default: false)
    pub brief_representation: Option<bool>,
    /// A String contained in email, or the complete email, if param "exact" is true
    pub email: Option<String>,
    /// whether the email has been verified
    pub email_verified: Option<bool>,
    /// Boolean representing if user is enabled or not
    pub enabled: Option<bool>,
    /// Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
    pub exact: Option<bool>,
    /// Pagination offset
    pub first: Option<i32>,
    /// A String contained in firstName, or the complete firstName, if param "exact" is true
    pub first_name: Option<String>,
    /// The alias of an Identity Provider linked to the user
    pub idp_alias: Option<String>,
    /// The userId at an Identity Provider linked to the user
    pub idp_user_id: Option<String>,
    /// A String contained in lastName, or the complete lastName, if param "exact" is true
    pub last_name: Option<String>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    pub search: Option<String>,
    /// A String contained in username, or the complete username, if param "exact" is true
    pub username: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmUsersGet<'a, TS> {
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmUsersGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            email,
            email_verified,
            enabled,
            exact,
            first,
            first_name,
            idp_alias,
            idp_user_id,
            last_name,
            max,
            q,
            search,
            username,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_get(
            self.realm_admin.realm,
            brief_representation,
            email,
            email_verified,
            enabled,
            exact,
            first,
            first_name,
            idp_alias,
            idp_user_id,
            last_name,
            max,
            q,
            search,
            username,
        )
    }
}

impl<'a, TS> IntoFuture for RealmUsersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmUsersCountGetArgs {
    /// email filter
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    /// Boolean representing if user is enabled or not
    pub enabled: Option<bool>,
    /// first name filter
    pub first_name: Option<String>,
    /// last name filter
    pub last_name: Option<String>,
    pub q: Option<String>,
    /// arbitrary search string for all the fields below. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    pub search: Option<String>,
    /// username filter
    pub username: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmUsersCountGet<'a, TS> {
    type Output = i32;
    type Args = RealmUsersCountGetArgs;

    fn opts(
        self,
        Self::Args {
            email,
            email_verified,
            enabled,
            first_name,
            last_name,
            q,
            search,
            username,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_count_get(
            self.realm_admin.realm,
            email,
            email_verified,
            enabled,
            first_name,
            last_name,
            q,
            search,
            username,
        )
    }
}

impl<'a, TS> IntoFuture for RealmUsersCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<i32, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersWithUserIdGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdGetArgs {
    /// Indicates if the user profile metadata should be added to the response
    pub user_profile_metadata: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmUsersWithUserIdGet<'a, TS> {
    type Output = UserRepresentation;
    type Args = RealmUsersWithUserIdGetArgs;

    fn opts(
        self,
        Self::Args {
            user_profile_metadata,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_with_user_id_get(
            self.realm_admin.realm,
            self.user_id,
            user_profile_metadata,
        )
    }
}

impl<'a, TS> IntoFuture for RealmUsersWithUserIdGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<UserRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
    pub body: Vec<String>,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdExecuteActionsEmailPutArgs {
    /// Client id
    pub client_id: Option<String>,
    /// Number of seconds after which the generated token expires
    pub lifespan: Option<i32>,
    /// Redirect uri
    pub redirect_uri: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmUsersWithUserIdExecuteActionsEmailPutArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            lifespan,
            redirect_uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_execute_actions_email_put(
                self.realm_admin.realm,
                self.user_id,
                client_id,
                lifespan,
                redirect_uri,
                self.body,
            )
    }
}

impl<'a, TS> IntoFuture for RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersWithUserIdGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdGroupsGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdGroupsGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmUsersWithUserIdGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_users_with_user_id_groups_get(
            self.realm_admin.realm,
            self.user_id,
            brief_representation,
            first,
            max,
            search,
        )
    }
}

impl<'a, TS> IntoFuture for RealmUsersWithUserIdGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersWithUserIdGroupsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdGroupsCountGetArgs {
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdGroupsCountGet<'a, TS>
{
    type Output = TypeMap<String, i64>;
    type Args = RealmUsersWithUserIdGroupsCountGetArgs;

    fn opts(
        self,
        Self::Args { search }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_groups_count_get(self.realm_admin.realm, self.user_id, search)
    }
}

impl<'a, TS> IntoFuture for RealmUsersWithUserIdGroupsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, i64>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersWithUserIdResetPasswordEmailPut<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdResetPasswordEmailPutArgs {
    /// client id
    pub client_id: Option<String>,
    /// redirect uri
    pub redirect_uri: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdResetPasswordEmailPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmUsersWithUserIdResetPasswordEmailPutArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            redirect_uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_reset_password_email_put(
                self.realm_admin.realm,
                self.user_id,
                client_id,
                redirect_uri,
            )
    }
}

impl<'a, TS> IntoFuture for RealmUsersWithUserIdResetPasswordEmailPut<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmUsersWithUserIdSendVerifyEmailPut<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct RealmUsersWithUserIdSendVerifyEmailPutArgs {
    /// Client id
    pub client_id: Option<String>,
    /// Number of seconds after which the generated token expires
    pub lifespan: Option<i32>,
    /// Redirect uri
    pub redirect_uri: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmUsersWithUserIdSendVerifyEmailPut<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmUsersWithUserIdSendVerifyEmailPutArgs;

    fn opts(
        self,
        Self::Args {
            client_id,
            lifespan,
            redirect_uri,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_users_with_user_id_send_verify_email_put(
                self.realm_admin.realm,
                self.user_id,
                client_id,
                lifespan,
                redirect_uri,
            )
    }
}

impl<'a, TS> IntoFuture for RealmUsersWithUserIdSendVerifyEmailPut<'a, TS>
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

    // <h4>Users</h4>
    impl<'a, TS> RealmUsersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// A String contained in email, or the complete email, if param "exact" is true
        pub fn email(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().email(value)
        }
        /// whether the email has been verified
        pub fn email_verified(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().email_verified(value)
        }
        /// Boolean representing if user is enabled or not
        pub fn enabled(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().enabled(value)
        }
        /// Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// Pagination offset
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// A String contained in firstName, or the complete firstName, if param "exact" is true
        pub fn first_name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().first_name(value)
        }
        /// The alias of an Identity Provider linked to the user
        pub fn idp_alias(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().idp_alias(value)
        }
        /// The userId at an Identity Provider linked to the user
        pub fn idp_user_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().idp_user_id(value)
        }
        /// A String contained in lastName, or the complete lastName, if param "exact" is true
        pub fn last_name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().last_name(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// A String contained in username, or the complete username, if param "exact" is true
        pub fn username(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().username(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief representations are returned (default: false)
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// A String contained in email, or the complete email, if param "exact" is true
        pub fn email(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.email = value.into();
            self
        }
        /// whether the email has been verified
        pub fn email_verified(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.email_verified = value.into();
            self
        }
        /// Boolean representing if user is enabled or not
        pub fn enabled(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.enabled = value.into();
            self
        }
        /// Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// Pagination offset
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// A String contained in firstName, or the complete firstName, if param "exact" is true
        pub fn first_name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.first_name = value.into();
            self
        }
        /// The alias of an Identity Provider linked to the user
        pub fn idp_alias(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.idp_alias = value.into();
            self
        }
        /// The userId at an Identity Provider linked to the user
        pub fn idp_user_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.idp_user_id = value.into();
            self
        }
        /// A String contained in lastName, or the complete lastName, if param "exact" is true
        pub fn last_name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.last_name = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// A String contained in username, or the complete username, if param "exact" is true
        pub fn username(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.username = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersCountGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// email filter
        pub fn email(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().email(value)
        }
        pub fn email_verified(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().email_verified(value)
        }
        /// Boolean representing if user is enabled or not
        pub fn enabled(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().enabled(value)
        }
        /// first name filter
        pub fn first_name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().first_name(value)
        }
        /// last name filter
        pub fn last_name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().last_name(value)
        }
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// arbitrary search string for all the fields below. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// username filter
        pub fn username(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().username(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersCountGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// email filter
        pub fn email(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.email = value.into();
            self
        }
        pub fn email_verified(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.email_verified = value.into();
            self
        }
        /// Boolean representing if user is enabled or not
        pub fn enabled(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.enabled = value.into();
            self
        }
        /// first name filter
        pub fn first_name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.first_name = value.into();
            self
        }
        /// last name filter
        pub fn last_name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.last_name = value.into();
            self
        }
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// arbitrary search string for all the fields below. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// username filter
        pub fn username(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.username = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Indicates if the user profile metadata should be added to the response
        pub fn user_profile_metadata(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().user_profile_metadata(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Indicates if the user profile metadata should be added to the response
        pub fn user_profile_metadata(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.user_profile_metadata = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdExecuteActionsEmailPut<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Client id
        pub fn client_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().client_id(value)
        }
        /// Number of seconds after which the generated token expires
        pub fn lifespan(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().lifespan(value)
        }
        /// Redirect uri
        pub fn redirect_uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().redirect_uri(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdExecuteActionsEmailPut<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Client id
        pub fn client_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.client_id = value.into();
            self
        }
        /// Number of seconds after which the generated token expires
        pub fn lifespan(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.lifespan = value.into();
            self
        }
        /// Redirect uri
        pub fn redirect_uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.redirect_uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
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

    impl<TS> Builder<'_, RealmUsersWithUserIdGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
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

    impl<'a, TS> RealmUsersWithUserIdGroupsCountGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdGroupsCountGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdResetPasswordEmailPut<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// client id
        pub fn client_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().client_id(value)
        }
        /// redirect uri
        pub fn redirect_uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().redirect_uri(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdResetPasswordEmailPut<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// client id
        pub fn client_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.client_id = value.into();
            self
        }
        /// redirect uri
        pub fn redirect_uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.redirect_uri = value.into();
            self
        }
    }

    impl<'a, TS> RealmUsersWithUserIdSendVerifyEmailPut<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Client id
        pub fn client_id(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().client_id(value)
        }
        /// Number of seconds after which the generated token expires
        pub fn lifespan(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().lifespan(value)
        }
        /// Redirect uri
        pub fn redirect_uri(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().redirect_uri(value)
        }
    }

    impl<TS> Builder<'_, RealmUsersWithUserIdSendVerifyEmailPut<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Client id
        pub fn client_id(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.client_id = value.into();
            self
        }
        /// Number of seconds after which the generated token expires
        pub fn lifespan(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.lifespan = value.into();
            self
        }
        /// Redirect uri
        pub fn redirect_uri(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.redirect_uri = value.into();
            self
        }
    }
}
