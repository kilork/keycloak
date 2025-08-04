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

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Groups</h4>
    impl<'a, TS> RealmGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn populate_hierarchy(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().populate_hierarchy(value)
        }
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// Boolean which defines whether to return the count of subgroups for each group (default: true
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
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
        pub fn populate_hierarchy(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.populate_hierarchy = value.into();
            self
        }
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// Boolean which defines whether to return the count of subgroups for each group (default: true
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmGroupsCountGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        pub fn top(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().top(value)
        }
    }

    impl<TS> Builder<'_, RealmGroupsCountGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        pub fn top(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.top = value.into();
            self
        }
    }

    impl<'a, TS> RealmGroupsWithGroupIdChildrenGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief groups representations are returned or not (default: false)
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// Boolean which defines whether the params "search" must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result to be returned (pagination offset).
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results that are to be returned. Defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A String representing either an exact group name or a partial name
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// Boolean which defines whether to return the count of subgroups for each subgroup of this group (default: true
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmGroupsWithGroupIdChildrenGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Boolean which defines whether brief groups representations are returned or not (default: false)
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// Boolean which defines whether the params "search" must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result to be returned (pagination offset).
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results that are to be returned. Defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A String representing either an exact group name or a partial name
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// Boolean which defines whether to return the count of subgroups for each subgroup of this group (default: true
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmGroupsWithGroupIdMembersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
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
    }

    impl<TS> Builder<'_, RealmGroupsWithGroupIdMembersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
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
    }
}
