use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Groups</h4>
    /// Get group hierarchy.  Only `name` and `id` are returned.  `subGroups` are only returned when using the `search` or `q` parameter. If none of these parameters is provided, the top-level groups are returned without `subGroups` being filled.
    pub fn groups_get(&'a self) -> RealmGroupsGet<'a, TS> {
        RealmGroupsGet { realm_admin: self }
    }

    /// create or add a top level realm groupSet or create child.
    ///
    /// This will update the group and set the parent if it exists. Create it and set the parent if the group doesn’t exist.
    pub fn groups_post(
        &'a self,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_groups_post(self.realm, body)
    }

    /// Returns the groups counts.
    pub fn groups_count_get(&'a self) -> RealmGroupsCountGet<'a, TS> {
        RealmGroupsCountGet { realm_admin: self }
    }

    pub fn groups_with_group_id_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<GroupRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_get(self.realm, group_id)
    }

    /// Update group, ignores subgroups.
    pub fn groups_with_group_id_put(
        &'a self,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_put(self.realm, group_id, body)
    }

    pub fn groups_with_group_id_delete(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_delete(self.realm, group_id)
    }

    /// Return a paginated list of subgroups that have a parent group corresponding to the group on the URL
    pub fn groups_with_group_id_children_get(
        &'a self,
        group_id: &'a str,
    ) -> RealmGroupsWithGroupIdChildrenGet<'a, TS> {
        RealmGroupsWithGroupIdChildrenGet {
            realm_admin: self,
            group_id,
        }
    }

    /// Set or create child.
    ///
    /// This will just set the parent if it exists. Create it and set the parent if the group doesn’t exist.
    pub fn groups_with_group_id_children_post(
        &'a self,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_groups_with_group_id_children_post(self.realm, group_id, body)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    pub fn groups_with_group_id_management_permissions_get(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_management_permissions_get(self.realm, group_id)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    pub fn groups_with_group_id_management_permissions_put(
        &'a self,
        group_id: &'a str,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_groups_with_group_id_management_permissions_put(self.realm, group_id, body)
    }

    /// Get users Returns a stream of users, filtered according to query parameters
    pub fn groups_with_group_id_members_get(
        &'a self,
        group_id: &'a str,
    ) -> RealmGroupsWithGroupIdMembersGet<'a, TS> {
        RealmGroupsWithGroupIdMembersGet {
            realm_admin: self,
            group_id,
        }
    }
}

// <h4>Groups</h4>
pub struct RealmGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmGroupsGetArgs {
    pub brief_representation: Option<bool>,
    pub exact: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub populate_hierarchy: Option<bool>,
    pub q: Option<String>,
    pub search: Option<String>,
    /// Boolean which defines whether to return the count of subgroups for each group (default: true
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmGroupsGet<'a, TS> {
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            populate_hierarchy,
            q,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_groups_get(
            self.realm_admin.realm,
            brief_representation,
            exact,
            first,
            max,
            populate_hierarchy,
            q,
            search,
            sub_groups_count,
        )
    }
}

impl<'a, TS> IntoFuture for RealmGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmGroupsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmGroupsCountGetArgs {
    pub search: Option<String>,
    pub top: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmGroupsCountGet<'a, TS> {
    type Output = TypeMap<String, i64>;
    type Args = RealmGroupsCountGetArgs;

    fn opts(
        self,
        Self::Args { search, top }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_count_get(self.realm_admin.realm, search, top)
    }
}

impl<'a, TS> IntoFuture for RealmGroupsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, i64>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmGroupsWithGroupIdChildrenGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
}

#[derive(Default)]
pub struct RealmGroupsWithGroupIdChildrenGetArgs {
    /// Boolean which defines whether brief groups representations are returned or not (default: false)
    pub brief_representation: Option<bool>,
    /// Boolean which defines whether the params "search" must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be returned (pagination offset).
    pub first: Option<i32>,
    /// The maximum number of results that are to be returned. Defaults to 10
    pub max: Option<i32>,
    /// A String representing either an exact group name or a partial name
    pub search: Option<String>,
    /// Boolean which defines whether to return the count of subgroups for each subgroup of this group (default: true
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmGroupsWithGroupIdChildrenGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmGroupsWithGroupIdChildrenGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_groups_with_group_id_children_get(
                self.realm_admin.realm,
                self.group_id,
                brief_representation,
                exact,
                first,
                max,
                search,
                sub_groups_count,
            )
    }
}

impl<'a, TS> IntoFuture for RealmGroupsWithGroupIdChildrenGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmGroupsWithGroupIdMembersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub group_id: &'a str,
}

#[derive(Default)]
pub struct RealmGroupsWithGroupIdMembersGetArgs {
    /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    pub brief_representation: Option<bool>,
    /// Pagination offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmGroupsWithGroupIdMembersGet<'a, TS>
{
    type Output = TypeVec<UserRepresentation>;
    type Args = RealmGroupsWithGroupIdMembersGetArgs;

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
            .realm_groups_with_group_id_members_get(
                self.realm_admin.realm,
                self.group_id,
                brief_representation,
                first,
                max,
            )
    }
}

impl<'a, TS> IntoFuture for RealmGroupsWithGroupIdMembersGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<UserRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}
