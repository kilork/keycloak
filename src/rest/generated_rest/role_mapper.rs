use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Role Mapper</h4>

    /// Get role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings`
    pub async fn realm_groups_with_group_id_role_mappings_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    pub async fn realm_groups_with_group_id_role_mappings_realm_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    pub async fn realm_groups_with_group_id_role_mappings_realm_post(
        &self,
        realm: &str,
        group_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_idrole_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    pub async fn realm_groups_with_group_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        group_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm/available`
    pub async fn realm_groups_with_group_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/realm/composite`
    pub async fn realm_groups_with_group_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        group_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappings>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings`
    pub async fn realm_users_with_user_id_role_mappings_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<MappingsRepresentation, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    pub async fn realm_users_with_user_id_role_mappings_realm_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add realm-level role mappings to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    pub async fn realm_users_with_user_id_role_mappings_realm_post(
        &self,
        realm: &str,
        user_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete realm-level role mappings
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Role Mapper`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/role-mappings/realm`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idrole_mappingsrealm>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    pub async fn realm_users_with_user_id_role_mappings_realm_delete(
        &self,
        realm: &str,
        user_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm-level roles that can be mapped
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealmavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm/available`
    pub async fn realm_users_with_user_id_role_mappings_realm_available_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective realm-level role mappings This will recurse all composite roles to get the result.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Role Mapper`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/realm/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsrealmcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/realm/composite`
    pub async fn realm_users_with_user_id_role_mappings_realm_composite_get(
        &self,
        realm: &str,
        user_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/realm/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 239
