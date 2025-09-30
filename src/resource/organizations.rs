use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Organizations</h4>
    /// Returns a paginated list of organizations filtered according to the specified parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned - defaults to 10
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String representing either an organization name or domain
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizations>
    pub fn organizations_get(&'a self) -> RealmOrganizationsGet<'a, TS> {
        RealmOrganizationsGet { realm_admin: self }
    }

    /// Creates a new organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmorganizations>
    pub fn organizations_post(
        &'a self,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_organizations_post(self.realm, body)
    }

    /// Returns the organizations counts.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String representing either an organization name or domain
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationscount>
    pub fn organizations_count_get(&'a self) -> RealmOrganizationsCountGet<'a, TS> {
        RealmOrganizationsCountGet { realm_admin: self }
    }

    /// Returns the organizations associated with the user that has the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `member_id`
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/members/{member_id}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/members/{member-id}/organizations`
    pub fn organizations_members_with_member_id_organizations_get(
        &'a self,
        member_id: &'a str,
    ) -> RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS> {
        RealmOrganizationsMembersWithMemberIdOrganizationsGet {
            realm_admin: self,
            member_id,
        }
    }

    /// Returns the organization representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}`
    pub fn organizations_with_org_id_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<OrganizationRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_get(self.realm, org_id)
    }

    /// Updates the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `PUT /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/organizations/{org-id}`
    pub fn organizations_with_org_id_put(
        &'a self,
        org_id: &'a str,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_put(self.realm, org_id, body)
    }

    /// Deletes the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}`
    pub fn organizations_with_org_id_delete(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_delete(self.realm, org_id)
    }

    /// Returns all identity providers associated with the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    pub fn organizations_with_org_id_identity_providers_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_get(self.realm, org_id)
    }

    /// Adds the identity provider with the specified id to the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/identity-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    pub fn organizations_with_org_id_identity_providers_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_post(self.realm, org_id, body)
    }

    /// Returns the identity provider associated with the organization that has the specified alias
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    pub fn organizations_with_org_id_identity_providers_with_alias_get(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_with_alias_get(
                self.realm, org_id, alias,
            )
    }

    /// Removes the identity provider with the specified alias from the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    pub fn organizations_with_org_id_identity_providers_with_alias_delete(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_with_alias_delete(
                self.realm, org_id, alias,
            )
    }

    /// Returns a paginated list of organization members filtered according to the specified parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned. Defaults to 10
    /// - `membership_type`: The membership type
    /// - `search`: A String representing either a member's username, e-mail, first name, or last name.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members`
    pub fn organizations_with_org_id_members_get(
        &'a self,
        org_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersGet {
            realm_admin: self,
            org_id,
        }
    }

    /// Adds the user with the specified id as a member of the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members`
    pub fn organizations_with_org_id_members_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_post(self.realm, org_id, body)
    }

    /// Returns number of members in the organization.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmemberscount>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/count`
    pub fn organizations_with_org_id_members_count_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<i64, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_count_get(self.realm, org_id)
    }

    /// Invites an existing user to the organization, using the specified user id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members/invite-existing-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_existing_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-existing-user`
    pub fn organizations_with_org_id_members_invite_existing_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_existing_user_post(
                self.realm, org_id, body,
            )
    }

    /// Invites an existing user or sends a registration link to a new user, based on the provided e-mail address.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members/invite-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-user`
    pub fn organizations_with_org_id_members_invite_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_user_post(self.realm, org_id, body)
    }

    /// Returns the member of the organization with the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    pub fn organizations_with_org_id_members_with_member_id_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> impl Future<Output = Result<MemberRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_with_member_id_get(
                self.realm, org_id, member_id,
            )
    }

    /// Removes the user with the specified id from the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/members/{member_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    pub fn organizations_with_org_id_members_with_member_id_delete(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_with_member_id_delete(
                self.realm, org_id, member_id,
            )
    }

    /// Returns the organizations associated with the user that has the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}/organizations`
    pub fn organizations_with_org_id_members_with_member_id_organizations_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet {
            realm_admin: self,
            org_id,
            member_id,
        }
    }
}

// <h4>Organizations</h4>
pub struct RealmOrganizationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned - defaults to 10
    pub max: Option<i32>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String representing either an organization name or domain
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_organizations_get(
            self.realm_admin.realm,
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
        )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmOrganizationsCountGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String representing either an organization name or domain
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsCountGet<'a, TS>
{
    type Output = i64;
    type Args = RealmOrganizationsCountGetArgs;

    fn opts(
        self,
        Self::Args { exact, q, search }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_organizations_count_get(
            self.realm_admin.realm,
            exact,
            q,
            search,
        )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<i64, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub member_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_members_with_member_id_organizations_get(
                self.realm_admin.realm,
                self.member_id,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdMembersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned. Defaults to 10
    pub max: Option<i32>,
    /// The membership type
    pub membership_type: Option<String>,
    /// A String representing either a member's username, e-mail, first name, or last name.
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersGet<'a, TS>
{
    type Output = TypeVec<MemberRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersGetArgs;

    fn opts(
        self,
        Self::Args {
            exact,
            first,
            max,
            membership_type,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_get(
                self.realm_admin.realm,
                self.org_id,
                exact,
                first,
                max,
                membership_type,
                search,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<MemberRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub member_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_with_member_id_organizations_get(
                self.realm_admin.realm,
                self.org_id,
                self.member_id,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Organizations</h4>
    impl<'a, TS> RealmOrganizationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results to be returned - defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// A String representing either an organization name or domain
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results to be returned - defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// A String representing either an organization name or domain
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsCountGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// A String representing either an organization name or domain
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsCountGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// A String representing either an organization name or domain
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsMembersWithMemberIdOrganizationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdMembersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results to be returned. Defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// The membership type
        pub fn membership_type(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().membership_type(value)
        }
        /// A String representing either a member's username, e-mail, first name, or last name.
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdMembersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results to be returned. Defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// The membership type
        pub fn membership_type(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.membership_type = value.into();
            self
        }
        /// A String representing either a member's username, e-mail, first name, or last name.
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }
}
