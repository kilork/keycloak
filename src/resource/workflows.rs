use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Workflows</h4>
    /// List workflows
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned - defaults to 10
    /// - `search`: A String representing the workflow name - either partial or exact
    ///
    /// Resource: `Workflows`
    ///
    /// `GET /admin/realms/{realm}/workflows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmworkflows>
    pub fn workflows_get(&'a self) -> RealmWorkflowsGet<'a, TS> {
        RealmWorkflowsGet { realm_admin: self }
    }

    /// Create workflow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Workflows`
    ///
    /// `POST /admin/realms/{realm}/workflows`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmworkflows>
    pub fn workflows_post(
        &'a self,
        body: WorkflowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_workflows_post(self.realm, body)
    }

    /// Migrate scheduled resources from one step to another
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `from`: A String representing the id of the step to migrate from
    /// - `to`: A String representing the id of the step to migrate to
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Workflows`
    ///
    /// `POST /admin/realms/{realm}/workflows/migrate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmworkflowsmigrate>
    pub fn workflows_migrate_post(&'a self) -> RealmWorkflowsMigratePost<'a, TS> {
        RealmWorkflowsMigratePost { realm_admin: self }
    }

    /// List scheduled workflows for resource
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `resource_id`: Identifier of the resource associated with the scheduled workflows
    ///
    /// Resource: `Workflows`
    ///
    /// `GET /admin/realms/{realm}/workflows/scheduled/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmworkflowsscheduledresource_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/workflows/scheduled/{resource-id}`
    pub fn workflows_scheduled_with_resource_id_get(
        &'a self,
        resource_id: &'a str,
    ) -> impl Future<Output = Result<WorkflowRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_workflows_scheduled_with_resource_id_get(self.realm, resource_id)
    }

    /// Get workflow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Workflow identifier
    /// - `include_id`: Indicates whether the workflow and step ids should be included in the representation or not - defaults to true
    ///
    /// Resource: `Workflows`
    ///
    /// `GET /admin/realms/{realm}/workflows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmworkflowsid>
    pub fn workflows_with_id_get(&'a self, id: &'a str) -> RealmWorkflowsWithIdGet<'a, TS> {
        RealmWorkflowsWithIdGet {
            realm_admin: self,
            id,
        }
    }

    /// Update workflow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Workflow identifier
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Workflows`
    ///
    /// `PUT /admin/realms/{realm}/workflows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_put_adminrealmsrealmworkflowsid>
    pub fn workflows_with_id_put(
        &'a self,
        id: &'a str,
        body: WorkflowRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_workflows_with_id_put(self.realm, id, body)
    }

    /// Delete workflow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Workflow identifier
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Workflows`
    ///
    /// `DELETE /admin/realms/{realm}/workflows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmworkflowsid>
    pub fn workflows_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_workflows_with_id_delete(self.realm, id)
    }

    /// Activate workflow for resource
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Workflow identifier
    /// - `resource_id`: Resource identifier
    /// - `type_`: Resource type
    /// - `not_before`: Optional value representing the time to schedule the first workflow step. The value is either an integer representing the seconds from now, an integer followed by 'ms' representing milliseconds from now, or an ISO-8601 date string.
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Workflows`
    ///
    /// `POST /admin/realms/{realm}/workflows/{id}/activate/{type_}/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmworkflowsidactivatetyperesourceid>
    ///
    /// REST method: `POST /admin/realms/{realm}/workflows/{id}/activate/{type}/{resourceId}`
    pub fn workflows_with_id_activate_with_type_with_resource_id_post(
        &'a self,
        id: &'a str,
        resource_id: &'a str,
        type_: &'a str,
    ) -> RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost<'a, TS> {
        RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost {
            realm_admin: self,
            id,
            resource_id,
            type_,
        }
    }

    /// Deactivate workflow for resource
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Workflow identifier
    /// - `resource_id`: Resource identifier
    /// - `type_`: Resource type
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Workflows`
    ///
    /// `POST /admin/realms/{realm}/workflows/{id}/deactivate/{type_}/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmworkflowsiddeactivatetyperesourceid>
    ///
    /// REST method: `POST /admin/realms/{realm}/workflows/{id}/deactivate/{type}/{resourceId}`
    pub fn workflows_with_id_deactivate_with_type_with_resource_id_post(
        &'a self,
        id: &'a str,
        resource_id: &'a str,
        type_: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_workflows_with_id_deactivate_with_type_with_resource_id_post(
                self.realm,
                id,
                resource_id,
                type_,
            )
    }
}

// <h4>Workflows</h4>
pub struct RealmWorkflowsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmWorkflowsGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned - defaults to 10
    pub max: Option<i32>,
    /// A String representing the workflow name - either partial or exact
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmWorkflowsGet<'a, TS>
{
    type Output = WorkflowRepresentation;
    type Args = RealmWorkflowsGetArgs;

    fn opts(
        self,
        Self::Args {
            exact,
            first,
            max,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_workflows_get(
            self.realm_admin.realm,
            exact,
            first,
            max,
            search,
        )
    }
}

impl<'a, TS> IntoFuture for RealmWorkflowsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<WorkflowRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmWorkflowsMigratePost<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmWorkflowsMigratePostArgs {
    /// A String representing the id of the step to migrate from
    pub from: Option<String>,
    /// A String representing the id of the step to migrate to
    pub to: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmWorkflowsMigratePost<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmWorkflowsMigratePostArgs;

    fn opts(
        self,
        Self::Args { from, to }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_workflows_migrate_post(self.realm_admin.realm, from, to)
    }
}

impl<'a, TS> IntoFuture for RealmWorkflowsMigratePost<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmWorkflowsWithIdGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// Workflow identifier
    pub id: &'a str,
}

#[derive(Default)]
pub struct RealmWorkflowsWithIdGetArgs {
    /// Indicates whether the workflow and step ids should be included in the representation or not - defaults to true
    pub include_id: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmWorkflowsWithIdGet<'a, TS>
{
    type Output = WorkflowRepresentation;
    type Args = RealmWorkflowsWithIdGetArgs;

    fn opts(
        self,
        Self::Args { include_id }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_workflows_with_id_get(
            self.realm_admin.realm,
            self.id,
            include_id,
        )
    }
}

impl<'a, TS> IntoFuture for RealmWorkflowsWithIdGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<WorkflowRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    /// Workflow identifier
    pub id: &'a str,
    /// Resource identifier
    pub resource_id: &'a str,
    /// Resource type
    pub type_: &'a str,
}

#[derive(Default)]
pub struct RealmWorkflowsWithIdActivateWithTypeWithResourceIdPostArgs {
    /// Optional value representing the time to schedule the first workflow step. The value is either an integer representing the seconds from now, an integer followed by 'ms' representing milliseconds from now, or an ISO-8601 date string.
    pub not_before: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmWorkflowsWithIdActivateWithTypeWithResourceIdPostArgs;

    fn opts(
        self,
        Self::Args { not_before }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_workflows_with_id_activate_with_type_with_resource_id_post(
                self.realm_admin.realm,
                self.id,
                self.resource_id,
                self.type_,
                not_before,
            )
    }
}

impl<'a, TS> IntoFuture for RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Workflows</h4>
    impl<'a, TS> RealmWorkflowsGet<'a, TS>
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
        /// The maximum number of results to be returned - defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A String representing the workflow name - either partial or exact
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmWorkflowsGet<'_, TS>>
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
        /// The maximum number of results to be returned - defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A String representing the workflow name - either partial or exact
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmWorkflowsMigratePost<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// A String representing the id of the step to migrate from
        pub fn from(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().from(value)
        }
        /// A String representing the id of the step to migrate to
        pub fn to(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().to(value)
        }
    }

    impl<TS> Builder<'_, RealmWorkflowsMigratePost<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// A String representing the id of the step to migrate from
        pub fn from(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.from = value.into();
            self
        }
        /// A String representing the id of the step to migrate to
        pub fn to(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.to = value.into();
            self
        }
    }

    impl<'a, TS> RealmWorkflowsWithIdGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Indicates whether the workflow and step ids should be included in the representation or not - defaults to true
        pub fn include_id(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().include_id(value)
        }
    }

    impl<TS> Builder<'_, RealmWorkflowsWithIdGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Indicates whether the workflow and step ids should be included in the representation or not - defaults to true
        pub fn include_id(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.include_id = value.into();
            self
        }
    }

    impl<'a, TS> RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Optional value representing the time to schedule the first workflow step. The value is either an integer representing the seconds from now, an integer followed by 'ms' representing milliseconds from now, or an ISO-8601 date string.
        pub fn not_before(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().not_before(value)
        }
    }

    impl<TS> Builder<'_, RealmWorkflowsWithIdActivateWithTypeWithResourceIdPost<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Optional value representing the time to schedule the first workflow step. The value is either an integer representing the seconds from now, an integer followed by 'ms' representing milliseconds from now, or an ISO-8601 date string.
        pub fn not_before(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.not_before = value.into();
            self
        }
    }
}
