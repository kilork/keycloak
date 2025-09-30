use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Component</h4>

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `name`
    /// - `parent`
    /// - `type_`
    ///
    /// Resource: `Component`
    ///
    /// `GET /admin/realms/{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmcomponents>
    pub async fn realm_components_get(
        &self,
        realm: &str,
        name: Option<String>,
        parent: Option<String>,
        type_: Option<String>,
    ) -> Result<TypeVec<ComponentRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/components", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = name {
            builder = builder.query(&[("name", v)]);
        }
        if let Some(v) = parent {
            builder = builder.query(&[("parent", v)]);
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
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Component`
    ///
    /// `POST /admin/realms/{realm}/components`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmcomponents>
    pub async fn realm_components_post(
        &self,
        realm: &str,
        body: ComponentRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/components", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Resource: `Component`
    ///
    /// `GET /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmcomponentsid>
    pub async fn realm_components_with_id_get(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<ComponentRepresentation, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/components/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Component`
    ///
    /// `PUT /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmcomponentsid>
    pub async fn realm_components_with_id_put(
        &self,
        realm: &str,
        id: &str,
        body: ComponentRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/components/{id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Component`
    ///
    /// `DELETE /admin/realms/{realm}/components/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmcomponentsid>
    pub async fn realm_components_with_id_delete(
        &self,
        realm: &str,
        id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/components/{id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `type_`
    ///
    /// Resource: `Component`
    ///
    /// `GET /admin/realms/{realm}/components/{id}/sub-component-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmcomponentsidsub_component_types>
    pub async fn realm_components_with_id_sub_component_types_get(
        &self,
        realm: &str,
        id: &str,
        type_: Option<String>,
    ) -> Result<TypeVec<ComponentTypeRepresentation>, KeycloakError> {
        let realm = p(realm);
        let id = p(id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/components/{id}/sub-component-types",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = type_ {
            builder = builder.query(&[("type", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 244
