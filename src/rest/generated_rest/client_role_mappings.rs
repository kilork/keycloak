use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Client Role Mappings</h4>

    /// Get client-level role mappings for the user or group, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_get(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `POST /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_post(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete client-level roles from user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}`
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_delete(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get available client-level roles that can be mapped to the user or group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_idavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}/available`
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_available_get(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings This recurses any composite roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client_id`: client id (not clientId!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroupsgroup_idrole_mappingsclientsclient_idcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client-id}/composite`
    pub async fn realm_groups_with_group_id_role_mappings_clients_with_client_id_composite_get(
        &self,
        realm: &str,
        group_id: &str,
        client_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let client_id = p(client_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/groups/{group_id}/role-mappings/clients/{client_id}/composite",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get client-level role mappings for the user or group, and the app
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_get(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add client-level roles to the user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_post(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete client-level roles from user or group role mapping
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmusersuser_idrole_mappingsclientsclient_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}`
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_delete(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
        body: Vec<RoleRepresentation>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get available client-level roles that can be mapped to the user or group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/available`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_idavailable>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}/available`
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_available_get(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/available",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get effective client-level role mappings This recurses any composite roles
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id (not clientId!)
    /// - `brief_representation`: if false, return roles with their attributes
    ///
    /// Resource: `Client Role Mappings`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/composite`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusersuser_idrole_mappingsclientsclient_idcomposite>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client-id}/composite`
    pub async fn realm_users_with_user_id_role_mappings_clients_with_client_id_composite_get(
        &self,
        realm: &str,
        user_id: &str,
        client_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RoleRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_id = p(client_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/role-mappings/clients/{client_id}/composite",
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
// left 241
