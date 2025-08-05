use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>default</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_server>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ResourceServerRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidauthzresource_server>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_put(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ResourceServerRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server",
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
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/import`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverimport>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/import`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_import_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ResourceServerRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/import",
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
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `permission`
    /// - `policy_id`
    /// - `resource`
    /// - `resource_type`
    /// - `scope`
    /// - `type_`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpermission>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        permission: Option<bool>,
        policy_id: Option<String>,
        resource: Option<String>,
        resource_type: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<AbstractPolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = permission {
            builder = builder.query(&[("permission", v)]);
        }
        if let Some(v) = policy_id {
            builder = builder.query(&[("policyId", v)]);
        }
        if let Some(v) = resource {
            builder = builder.query(&[("resource", v)]);
        }
        if let Some(v) = resource_type {
            builder = builder.query(&[("resourceType", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpermission>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission",
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
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/evaluate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpermissionevaluate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission/evaluate`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_evaluate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: PolicyEvaluationRequest,
    ) -> Result<PolicyEvaluationResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/evaluate",
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
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpermissionproviders>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission/providers`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_providers_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<PolicyProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `name`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpermissionsearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/permission/search`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_permission_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        name: Option<String>,
    ) -> Result<AbstractPolicyRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/permission/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `permission`
    /// - `policy_id`
    /// - `resource`
    /// - `resource_type`
    /// - `scope`
    /// - `type_`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicy>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        permission: Option<bool>,
        policy_id: Option<String>,
        resource: Option<String>,
        resource_type: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<AbstractPolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = permission {
            builder = builder.query(&[("permission", v)]);
        }
        if let Some(v) = policy_id {
            builder = builder.query(&[("policyId", v)]);
        }
        if let Some(v) = resource {
            builder = builder.query(&[("resource", v)]);
        }
        if let Some(v) = resource_type {
            builder = builder.query(&[("resourceType", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicy>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy",
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
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/evaluate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicyevaluate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy/evaluate`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_evaluate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: PolicyEvaluationRequest,
    ) -> Result<PolicyEvaluationResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/evaluate",
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
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicyproviders>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy/providers`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_providers_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<PolicyProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `name`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverpolicysearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/policy/search`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_policy_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        fields: Option<String>,
        name: Option<String>,
    ) -> Result<AbstractPolicyRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/policy/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = fields {
            builder = builder.query(&[("fields", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresource>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
    ) -> Result<TypeVec<ResourceRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `body`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverresource>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_post(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        body: ResourceRepresentation,
    ) -> Result<ResourceRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourcesearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/search`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
    ) -> Result<ResourceRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<ResourceRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
        body: ResourceRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// Returns response for future processing.
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// Returns response for future processing.
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/attributes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_idattributes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}/attributes`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_attributes_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/attributes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_idpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}/permissions`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<TypeVec<PolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `resource_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverresourceresource_idscopes>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/resource/{resource-id}/scopes`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_clients_with_client_uuid_authz_resource_server_resource_with_resource_id_scopes_get(
        &self,
        realm: &str,
        client_uuid: &str,
        id: Option<String>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<String>,
        owner: Option<String>,
        scope: Option<String>,
        type_: Option<String>,
        uri: Option<String>,
        resource_id: &str,
    ) -> Result<TypeVec<ScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let resource_id = p(resource_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/resource/{resource_id}/scopes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = id {
            builder = builder.query(&[("_id", v)]);
        }
        if let Some(v) = deep {
            builder = builder.query(&[("deep", v)]);
        }
        if let Some(v) = exact_name {
            builder = builder.query(&[("exactName", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = matching_uri {
            builder = builder.query(&[("matchingUri", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = owner {
            builder = builder.query(&[("owner", v)]);
        }
        if let Some(v) = scope {
            builder = builder.query(&[("scope", v)]);
        }
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        if let Some(v) = uri {
            builder = builder.query(&[("uri", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscope>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_get(
        &self,
        realm: &str,
        client_uuid: &str,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<String>,
        scope_id: Option<String>,
    ) -> Result<TypeVec<ScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = scope_id {
            builder = builder.query(&[("scopeId", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidauthzresource_serverscope>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_post(
        &self,
        realm: &str,
        client_uuid: &str,
        body: ScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope",
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
    /// - `client_uuid`: id of client (not client-id!)
    /// - `name`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/search`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopesearch>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/search`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_search_get(
        &self,
        realm: &str,
        client_uuid: &str,
        name: Option<String>,
    ) -> Result<TypeVec<ScopeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/search",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<ScopeRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_put(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
        body: ScopeRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}",
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
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_delete(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_idpermissions>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}/permissions`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_permissions_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<TypeVec<PolicyRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/permissions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/resources`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serverscopescope_idresources>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/scope/{scope-id}/resources`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_scope_with_scope_id_resources_get(
        &self,
        realm: &str,
        client_uuid: &str,
        scope_id: &str,
    ) -> Result<TypeVec<ResourceRepresentation>, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let scope_id = p(scope_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/scope/{scope_id}/resources",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/settings`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidauthzresource_serversettings>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/settings`
    pub async fn realm_clients_with_client_uuid_authz_resource_server_settings_get(
        &self,
        realm: &str,
        client_uuid: &str,
    ) -> Result<ResourceServerRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/authz/resource-server/settings",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 225
