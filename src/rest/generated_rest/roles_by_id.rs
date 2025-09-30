use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Roles (by ID)</h4>

    /// Get a specific role's representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}`
    pub async fn realm_roles_by_id_with_role_id_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<RoleRepresentation, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles-by-id/{role-id}`
    pub async fn realm_roles_by_id_with_role_id_put(
        &self,
        realm: &str,
        role_id: &str,
        body: RoleRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the role
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /admin/realms/{realm}/roles-by-id/{role_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmroles_by_idrole_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles-by-id/{role-id}`
    pub async fn realm_roles_by_id_with_role_id_delete(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get role's children Returns a set of role's children provided the role is a composite.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    pub async fn realm_roles_by_id_with_role_id_composites_get(
        &self,
        realm: &str,
        role_id: &str,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
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

    /// Make the role a composite role by associating some child roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `POST /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `POST /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    pub async fn realm_roles_by_id_with_role_id_composites_post(
        &self,
        realm: &str,
        role_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a set of roles from the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`: Role id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `DELETE /admin/realms/{realm}/roles-by-id/{role_id}/composites`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmroles_by_idrole_idcomposites>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/roles-by-id/{role-id}/composites`
    pub async fn realm_roles_by_id_with_role_id_composites_delete(
        &self,
        realm: &str,
        role_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites",
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
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/composites/clients/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcompositesclientsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites/clients/{clientUuid}`
    pub async fn realm_roles_by_id_with_role_id_composites_clients_with_client_uuid_get(
        &self,
        realm: &str,
        client_uuid: &str,
        role_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites/clients/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level roles that are in the role's composite
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/composites/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idcompositesrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/composites/realm`
    pub async fn realm_roles_by_id_with_role_id_composites_realm_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/composites/realm",
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
    /// - `role_id`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `GET /admin/realms/{realm}/roles-by-id/{role_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmroles_by_idrole_idmanagementpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    pub async fn realm_roles_by_id_with_role_id_management_permissions_get(
        &self,
        realm: &str,
        role_id: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/management/permissions",
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
    /// - `role_id`
    /// - `body`
    ///
    /// Resource: `Roles (by ID)`
    ///
    /// `PUT /admin/realms/{realm}/roles-by-id/{role_id}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmroles_by_idrole_idmanagementpermissions>
    ///
    /// REST method: `PUT /admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    pub async fn realm_roles_by_id_with_role_id_management_permissions_put(
        &self,
        realm: &str,
        role_id: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let role_id = p(role_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/roles-by-id/{role_id}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 242
