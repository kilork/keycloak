use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Groups</h4>

    /// Get group hierarchy.  Only `name` and `id` are returned.  `subGroups` are only returned when using the `search` or `q` parameter. If none of these parameters is provided, the top-level groups are returned without `subGroups` being filled.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `exact`
    /// - `first`
    /// - `max`
    /// - `populate_hierarchy`
    /// - `q`
    /// - `search`
    /// - `sub_groups_count`: Boolean which defines whether to return the count of subgroups for each group (default: true
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmgroups>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_groups_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        populate_hierarchy: Option<bool>,
        q: Option<String>,
        search: Option<String>,
        sub_groups_count: Option<bool>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/groups", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = populate_hierarchy {
            builder = builder.query(&[("populateHierarchy", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = sub_groups_count {
            builder = builder.query(&[("subGroupsCount", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// create or add a top level realm groupSet or create child.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `POST /admin/realms/{realm}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmgroups>
    pub async fn realm_groups_post(
        &self,
        realm: &str,
        body: GroupRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/groups", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the groups counts.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `search`
    /// - `top`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmgroupscount>
    pub async fn realm_groups_count_get(
        &self,
        realm: &str,
        search: Option<String>,
        top: Option<bool>,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/groups/count", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = top {
            builder = builder.query(&[("top", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}`
    pub async fn realm_groups_with_group_id_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update group, ignores subgroups.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /admin/realms/{realm}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmgroupsgroup_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/groups/{group-id}`
    pub async fn realm_groups_with_group_id_put(
        &self,
        realm: &str,
        group_id: &str,
        body: GroupRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/groups/{group_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}`
    pub async fn realm_groups_with_group_id_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return a paginated list of subgroups that have a parent group corresponding to the group on the URL
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: Boolean which defines whether brief groups representations are returned or not (default: false)
    /// - `exact`: Boolean which defines whether the params "search" must match exactly or not
    /// - `first`: The position of the first result to be returned (pagination offset).
    /// - `max`: The maximum number of results that are to be returned. Defaults to 10
    /// - `search`: A String representing either an exact group name or a partial name
    /// - `sub_groups_count`: Boolean which defines whether to return the count of subgroups for each subgroup of this group (default: true
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idchildren>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/children`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_groups_with_group_id_children_get(
        &self,
        realm: &str,
        group_id: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
        sub_groups_count: Option<bool>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/children",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
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
        if let Some(v) = sub_groups_count {
            builder = builder.query(&[("subGroupsCount", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Set or create child.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Groups`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idchildren>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/children`
    pub async fn realm_groups_with_group_id_children_post(
        &self,
        realm: &str,
        group_id: &str,
        body: GroupRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/children",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/management/permissions`
    pub async fn realm_groups_with_group_id_management_permissions_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Resource: `Groups`
    ///
    /// `PUT /admin/realms/{realm}/groups/{group_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmgroupsgroup_idmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/groups/{group-id}/management/permissions`
    pub async fn realm_groups_with_group_id_management_permissions_put(
        &self,
        realm: &str,
        group_id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get users Returns a stream of users, filtered according to query parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Groups`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/members`
    pub async fn realm_groups_with_group_id_members_get(
        &self,
        realm: &str,
        group_id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/members",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 241
