use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_get_adminrealmsrealmworkflows>
    pub async fn realm_workflows_get(
        &self,
        realm: &str,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<WorkflowRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/workflows", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_post_adminrealmsrealmworkflows>
    pub async fn realm_workflows_post(
        &self,
        realm: &str,
        body: WorkflowRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/workflows", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_get_adminrealmsrealmworkflowsscheduledresource_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/workflows/scheduled/{resource-id}`
    pub async fn realm_workflows_scheduled_with_resource_id_get(
        &self,
        realm: &str,
        resource_id: &str,
    ) -> Result<WorkflowRepresentation, KeycloakError> {
        let realm = p(realm);
        let resource_id = p(resource_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/workflows/scheduled/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get workflow
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`: Workflow identifier
    /// - `include_id`: Indicates whether the workflow id should be included in the representation or not - defaults to true
    ///
    /// Resource: `Workflows`
    ///
    /// `GET /admin/realms/{realm}/workflows/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_get_adminrealmsrealmworkflowsid>
    pub async fn realm_workflows_with_id_get(
        &self,
        realm: &str,
        id: &str,
        include_id: Option<bool>,
    ) -> Result<WorkflowRepresentation, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/workflows/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = include_id {
            builder = builder.query(&[("includeId", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_put_adminrealmsrealmworkflowsid>
    pub async fn realm_workflows_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: WorkflowRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/workflows/{id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_delete_adminrealmsrealmworkflowsid>
    pub async fn realm_workflows_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/workflows/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_post_adminrealmsrealmworkflowsidactivatetyperesourceid>
    ///
    /// REST method: `POST /admin/realms/{realm}/workflows/{id}/activate/{type}/{resourceId}`
    pub async fn realm_workflows_with_id_activate_with_type_with_resource_id_post(
        &self,
        realm: &str,
        id: &str,
        resource_id: &str,
        type_: &str,
        not_before: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let resource_id = p(resource_id);
        let type_ = p(type_);
        let mut builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/workflows/{id}/activate/{type_}/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = not_before {
            builder = builder.query(&[("notBefore", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.5.2/rest-api/index.html#_post_adminrealmsrealmworkflowsiddeactivatetyperesourceid>
    ///
    /// REST method: `POST /admin/realms/{realm}/workflows/{id}/deactivate/{type}/{resourceId}`
    pub async fn realm_workflows_with_id_deactivate_with_type_with_resource_id_post(
        &self,
        realm: &str,
        id: &str,
        resource_id: &str,
        type_: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let resource_id = p(resource_id);
        let type_ = p(type_);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/workflows/{id}/deactivate/{type_}/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }
}
// not all paths processed
// left 251
