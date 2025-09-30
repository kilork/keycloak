use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Roles</h4>

    /// Get all roles for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidroles>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles`
    pub async fn realm_clients_with_client_uuid_roles_get(
        &self,
        realm: &str,
        client_uuid: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles",
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
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidroles>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/roles`
    pub async fn realm_clients_with_client_uuid_roles_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_put(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get composites of the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client-level roles for the client that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecompositesclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites/clients/{client-uuid}`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/composites/realm`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/composites/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of groups that have the specified role name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/groups`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/groups",
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

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/management/permissions`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/management/permissions`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/users`
    pub async fn realm_clients_with_client_uuid_roles_with_role_name_users_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/roles/{role_name}/users",
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

    /// Get all roles for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmroles>
    pub async fn realm_roles_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/roles", self.url))
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
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new role for the realm or client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmroles>
    pub async fn realm_roles_post(
        &self,
        realm: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/roles", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}`
    pub async fn realm_roles_with_role_name_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}`
    pub async fn realm_roles_with_role_name_put(
        &self,
        realm: &str,
        role_name: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete a role by name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmrolesrole_name>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}`
    pub async fn realm_roles_with_role_name_delete(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles/{role_name}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get composites of the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites`
    pub async fn realm_roles_with_role_name_composites_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a composite to the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `POST /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles/{role-name}/composites`
    pub async fn realm_roles_with_role_name_composites_post(
        &self,
        realm: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles`
    ///
    /// `DELETE /admin/realms/{realm}/roles/{role_name}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmrolesrole_namecomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles/{role-name}/composites`
    pub async fn realm_roles_with_role_name_composites_delete(
        &self,
        realm: &str,
        role_name: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client-level roles for the client that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesclientsclient_uuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/clients/{client-uuid}`
    pub async fn realm_roles_with_role_name_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles of the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_namecompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/composites/realm`
    pub async fn realm_roles_with_role_name_composites_realm_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/composites/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of groups that have the specified role name
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_namegroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/groups`
    pub async fn realm_roles_with_role_name_groups_get(
        &self,
        realm: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/groups",
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

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/management/permissions`
    pub async fn realm_roles_with_role_name_management_permissions_get(
        &self,
        realm: &str,
        role_name: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/management/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Return object stating whether role Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    /// - `body`
    ///
    /// Resource: `Roles`
    ///
    /// `PUT /admin/realms/{realm}/roles/{role_name}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmrolesrole_namemanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles/{role-name}/management/permissions`
    pub async fn realm_roles_with_role_name_management_permissions_put(
        &self,
        realm: &str,
        role_name: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns a stream of users that have the specified role name.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or {@code null}.
    ///
    /// Resource: `Roles`
    ///
    /// `GET /admin/realms/{realm}/roles/{role_name}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmrolesrole_nameusers>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles/{role-name}/users`
    pub async fn realm_roles_with_role_name_users_get(
        &self,
        realm: &str,
        role_name: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_name = p(role_name);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles/{role_name}/users",
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
// left 231
