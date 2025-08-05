use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Realms Admin</h4>

    /// Get accessible realms Returns a list of accessible realms. The list is filtered based on what realms the caller is allowed to view.
    ///
    /// Parameters:
    ///
    /// - `brief_representation`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealms>
    pub async fn get(
        &self,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<RealmRepresentation>, KeycloakError> {
        let mut builder = self
            .client
            .get(format!("{}/admin/realms", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Import a realm. Imports a realm from a full representation of that realm.
    ///
    /// Parameters:
    ///
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealms>
    pub async fn post(&self, body: RealmRepresentation) -> Result<DefaultResponse, KeycloakError> {
        let builder = self
            .client
            .post(format!("{}/admin/realms", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the top-level representation of the realm It will not include nested information like User and Client representations.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealm>
    pub async fn realm_get(&self, realm: &str) -> Result<RealmRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the top-level information of the realm Any user, roles or client information in the representation will be ignored.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealm>
    pub async fn realm_put(
        &self,
        realm: &str,
        body: RealmRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealm>
    pub async fn realm_delete(&self, realm: &str) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get admin events Returns all admin events, or filters events based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `auth_client`
    /// - `auth_ip_address`
    /// - `auth_realm`
    /// - `auth_user`: user id
    /// - `date_from`: From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `date_to`: To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `direction`: The direction to sort events by (asc or desc)
    /// - `first`
    /// - `max`: Maximum results size (defaults to 100)
    /// - `operation_types`
    /// - `resource_path`
    /// - `resource_types`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmadmin_events>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_admin_events_get(
        &self,
        realm: &str,
        auth_client: Option<String>,
        auth_ip_address: Option<String>,
        auth_realm: Option<String>,
        auth_user: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        direction: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        operation_types: Option<Vec<String>>,
        resource_path: Option<String>,
        resource_types: Option<Vec<String>>,
    ) -> Result<TypeVec<AdminEventRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/admin-events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = auth_client {
            builder = builder.query(&[("authClient", v)]);
        }
        if let Some(v) = auth_ip_address {
            builder = builder.query(&[("authIpAddress", v)]);
        }
        if let Some(v) = auth_realm {
            builder = builder.query(&[("authRealm", v)]);
        }
        if let Some(v) = auth_user {
            builder = builder.query(&[("authUser", v)]);
        }
        if let Some(v) = date_from {
            builder = builder.query(&[("dateFrom", v)]);
        }
        if let Some(v) = date_to {
            builder = builder.query(&[("dateTo", v)]);
        }
        if let Some(v) = direction {
            builder = builder.query(&[("direction", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = operation_types {
            builder = builder.query(
                &v.into_iter()
                    .map(|e| ("operationTypes", e))
                    .collect::<Vec<_>>(),
            );
        }
        if let Some(v) = resource_path {
            builder = builder.query(&[("resourcePath", v)]);
        }
        if let Some(v) = resource_types {
            builder = builder.query(
                &v.into_iter()
                    .map(|e| ("resourceTypes", e))
                    .collect::<Vec<_>>(),
            );
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete all admin events
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmadmin_events>
    pub async fn realm_admin_events_delete(
        &self,
        realm: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/admin-events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Base path for importing clients under this realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/client-description-converter`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_description_converter>
    pub async fn realm_client_description_converter_post(
        &self,
        realm: &str,
        body: String,
    ) -> Result<ClientRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/client-description-converter",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `include_global_policies`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_policiespolicies>
    pub async fn realm_client_policies_policies_get(
        &self,
        realm: &str,
        include_global_policies: Option<bool>,
    ) -> Result<ClientPoliciesRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-policies/policies",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = include_global_policies {
            builder = builder.query(&[("include-global-policies", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_policiespolicies>
    pub async fn realm_client_policies_policies_put(
        &self,
        realm: &str,
        body: ClientPoliciesRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-policies/policies",
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
    /// - `include_global_profiles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_policiesprofiles>
    pub async fn realm_client_policies_profiles_get(
        &self,
        realm: &str,
        include_global_profiles: Option<bool>,
    ) -> Result<ClientProfilesRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-policies/profiles",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = include_global_profiles {
            builder = builder.query(&[("include-global-profiles", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_policiesprofiles>
    pub async fn realm_client_policies_profiles_put(
        &self,
        realm: &str,
        body: ClientProfilesRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/client-policies/profiles",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get client session stats Returns a JSON map.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-session-stats`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_session_stats>
    pub async fn realm_client_session_stats_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<TypeMap<String, String>>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/client-session-stats",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// List all client types available in the current realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_types>
    pub async fn realm_client_types_get(
        &self,
        realm: &str,
    ) -> Result<ClientTypesRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/client-types", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a client type
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_types>
    pub async fn realm_client_types_put(
        &self,
        realm: &str,
        body: ClientTypesRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/client-types", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/credential-registrators`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmcredential_registrators>
    pub async fn realm_credential_registrators_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<String>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/credential-registrators",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get realm default client scopes. Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_default_client_scopes>
    pub async fn realm_default_default_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/default-default-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    pub async fn realm_default_default_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/default-default-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    pub async fn realm_default_default_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/default-default-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get group hierarchy.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_groups>
    pub async fn realm_default_groups_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/default-groups", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-groups/{groupId}`
    pub async fn realm_default_groups_with_group_id_put(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/default-groups/{group_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
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
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-groups/{groupId}`
    pub async fn realm_default_groups_with_group_id_delete(
        &self,
        realm: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/default-groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get realm optional client scopes. Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_optional_client_scopes>
    pub async fn realm_default_optional_client_scopes_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<ClientScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_put(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    pub async fn realm_default_optional_client_scopes_with_client_scope_id_delete(
        &self,
        realm: &str,
        client_scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_scope_id = p(client_scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get events Returns all events, or filters them based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client`: App or oauth client name
    /// - `date_from`: From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `date_to`: To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `direction`: The direction to sort events by (asc or desc)
    /// - `first`: Paging offset
    /// - `ip_address`: IP Address
    /// - `max`: Maximum results size (defaults to 100)
    /// - `type_`: The types of events to return
    /// - `user`: User id
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmevents>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_events_get(
        &self,
        realm: &str,
        client: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        direction: Option<String>,
        first: Option<i32>,
        ip_address: Option<String>,
        max: Option<i32>,
        type_: Option<Vec<String>>,
        user: Option<String>,
    ) -> Result<TypeVec<EventRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client {
            builder = builder.query(&[("client", v)]);
        }
        if let Some(v) = date_from {
            builder = builder.query(&[("dateFrom", v)]);
        }
        if let Some(v) = date_to {
            builder = builder.query(&[("dateTo", v)]);
        }
        if let Some(v) = direction {
            builder = builder.query(&[("direction", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = ip_address {
            builder = builder.query(&[("ipAddress", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&v.into_iter().map(|e| ("type", e)).collect::<Vec<_>>());
        }
        if let Some(v) = user {
            builder = builder.query(&[("user", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Delete all events
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmevents>
    pub async fn realm_events_delete(&self, realm: &str) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/events", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the events provider configuration Returns JSON object with events provider configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmeventsconfig>
    pub async fn realm_events_config_get(
        &self,
        realm: &str,
    ) -> Result<RealmEventsConfigRepresentation, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/events/config", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmeventsconfig>
    pub async fn realm_events_config_put(
        &self,
        realm: &str,
        body: RealmEventsConfigRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/events/config", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `path`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/group-by-path/{path}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroup_by_pathpath>
    pub async fn realm_group_by_path_with_path_get(
        &self,
        realm: &str,
        path: &str,
    ) -> Result<GroupRepresentation, KeycloakError> {
        let realm = p(realm);
        let path = p(path);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/group-by-path/{path}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalization>
    pub async fn realm_localization_get(
        &self,
        realm: &str,
    ) -> Result<TypeVec<String>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/localization", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `use_realm_default_locale_fallback`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalizationlocale>
    pub async fn realm_localization_with_locale_get(
        &self,
        realm: &str,
        locale: &str,
        use_realm_default_locale_fallback: Option<bool>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        let realm = p(realm);
        let locale = p(locale);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = use_realm_default_locale_fallback {
            builder = builder.query(&[("useRealmDefaultLocaleFallback", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Import localization from uploaded JSON file
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmlocalizationlocale>
    pub async fn realm_localization_with_locale_post(
        &self,
        realm: &str,
        locale: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let locale = p(locale);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/localization/{locale}",
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
    /// - `locale`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocale>
    pub async fn realm_localization_with_locale_delete(
        &self,
        realm: &str,
        locale: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let locale = p(locale);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/localization/{locale}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalizationlocalekey>
    pub async fn realm_localization_with_locale_with_key_get(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
    ) -> Result<TypeString, KeycloakError> {
        let realm = p(realm);
        let key = p(key);
        let locale = p(locale);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.text().await.map(From::from)?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmlocalizationlocalekey>
    pub async fn realm_localization_with_locale_with_key_put(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let key = p(key);
        let locale = p(locale);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .body(body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocalekey>
    pub async fn realm_localization_with_locale_with_key_delete(
        &self,
        realm: &str,
        key: &str,
        locale: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let key = p(key);
        let locale = p(locale);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/localization/{locale}/{key}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Removes all user sessions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/logout-all`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmlogout_all>
    pub async fn realm_logout_all_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/logout-all", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Partial export of existing realm into a JSON file.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `export_clients`
    /// - `export_groups_and_roles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/partial-export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpartial_export>
    pub async fn realm_partial_export_post(
        &self,
        realm: &str,
        export_clients: Option<bool>,
        export_groups_and_roles: Option<bool>,
    ) -> Result<RealmRepresentation, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/partial-export", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = export_clients {
            builder = builder.query(&[("exportClients", v)]);
        }
        if let Some(v) = export_groups_and_roles {
            builder = builder.query(&[("exportGroupsAndRoles", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Partial import from a JSON file to an existing realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/partialImport`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpartialimport>
    pub async fn realm_partial_import_post(
        &self,
        realm: &str,
        body: RealmRepresentation,
    ) -> Result<Value, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/partialImport", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Push the realm's revocation policy to any client that has an admin url associated with it.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpush_revocation>
    pub async fn realm_push_revocation_post(
        &self,
        realm: &str,
    ) -> Result<GlobalRequestResult, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/push-revocation", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a specific user session.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `session`
    /// - `is_offline`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/sessions/{session}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmsessionssession>
    pub async fn realm_sessions_with_session_delete(
        &self,
        realm: &str,
        session: &str,
        is_offline: Option<bool>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let session = p(session);
        let mut builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/sessions/{session}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = is_offline {
            builder = builder.query(&[("isOffline", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Test SMTP connection with current logged in user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/testSMTPConnection`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmtestsmtpconnection>
    #[deprecated]
    pub async fn realm_test_smtp_connection_post(
        &self,
        realm: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/testSMTPConnection",
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
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusers_management_permissions>
    pub async fn realm_users_management_permissions_get(
        &self,
        realm: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users-management-permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusers_management_permissions>
    pub async fn realm_users_management_permissions_put(
        &self,
        realm: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users-management-permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 219
