use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Identity Providers</h4>

    /// Import identity provider from JSON body
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/import-config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmidentity_providerimport_config>
    pub async fn realm_identity_provider_import_config_post(
        &self,
        realm: &str,
        body: TypeMap<String, Value>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/import-config",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// List identity providers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    /// - `realm_only`: Boolean which defines if only realm-level IDPs (not associated with orgs) should be returned (default: false)
    /// - `search`: Filter specific providers by name. Search can be prefix (name*), contains (*name*) or exact ("name"). Default prefixed.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstances>
    pub async fn realm_identity_provider_instances_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        realm_only: Option<bool>,
        search: Option<String>,
    ) -> Result<TypeVec<IdentityProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
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
        if let Some(v) = realm_only {
            builder = builder.query(&[("realmOnly", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstances>
    pub async fn realm_identity_provider_instances_post(
        &self,
        realm: &str,
        body: IdentityProviderRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/instances",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesalias>
    pub async fn realm_identity_provider_instances_with_alias_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesalias>
    pub async fn realm_identity_provider_instances_with_alias_put(
        &self,
        realm: &str,
        alias: &str,
        body: IdentityProviderRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesalias>
    pub async fn realm_identity_provider_instances_with_alias_delete(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Export public broker configuration for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `format`: Format to use
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasexport>
    pub async fn realm_identity_provider_instances_with_alias_export_get(
        &self,
        realm: &str,
        alias: &str,
        format: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/export",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = format {
            builder = builder.query(&[("format", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return object stating whether client Authorization permissions have been initialized or not and a reference
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions",
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
    /// - `alias`
    /// - `body`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmanagementpermissions>
    pub async fn realm_identity_provider_instances_with_alias_management_permissions_put(
        &self,
        realm: &str,
        alias: &str,
        body: ManagementPermissionReference,
    ) -> Result<ManagementPermissionReference, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/management/permissions",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mapper types for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmapper_types>
    pub async fn realm_identity_provider_instances_with_alias_mapper_types_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<TypeMap<String, IdentityProviderMapperTypeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get mappers for identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappers>
    pub async fn realm_identity_provider_instances_with_alias_mappers_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<TypeVec<IdentityProviderMapperRepresentation>, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a mapper to identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmidentity_providerinstancesaliasmappers>
    pub async fn realm_identity_provider_instances_with_alias_mappers_post(
        &self,
        realm: &str,
        alias: &str,
        body: IdentityProviderMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get mapper by id for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_get(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<IdentityProviderMapperRepresentation, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let id = p(id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update a mapper for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `PUT /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_put(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
        body: IdentityProviderMapperRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let id = p(id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete a mapper for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Identity Providers`
    ///
    /// `DELETE /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmidentity_providerinstancesaliasmappersid>
    pub async fn realm_identity_provider_instances_with_alias_mappers_with_id_delete(
        &self,
        realm: &str,
        alias: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let id = p(id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Reaload keys for the identity provider if the provider supports it, "true" is returned if reload was performed, "false" if not.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `alias`
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/instances/{alias}/reload-keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerinstancesaliasreload_keys>
    pub async fn realm_identity_provider_instances_with_alias_reload_keys_get(
        &self,
        realm: &str,
        alias: &str,
    ) -> Result<bool, KeycloakError> {
        let realm = p(realm);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/instances/{alias}/reload-keys",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get the identity provider factory for that provider id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `provider_id`: The provider id to get the factory
    ///
    /// Resource: `Identity Providers`
    ///
    /// `GET /admin/realms/{realm}/identity-provider/providers/{provider_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmidentity_providerprovidersprovider_id>
    pub async fn realm_identity_provider_providers_with_provider_id_get(
        &self,
        realm: &str,
        provider_id: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let provider_id = p(provider_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/identity-provider/providers/{provider_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 237
