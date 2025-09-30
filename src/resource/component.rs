use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
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
    pub fn components_get(&'a self) -> RealmComponentsGet<'a, TS> {
        RealmComponentsGet { realm_admin: self }
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
    pub fn components_post(
        &'a self,
        body: ComponentRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_post(self.realm, body)
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
    pub fn components_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<ComponentRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_with_id_get(self.realm, id)
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
    pub fn components_with_id_put(
        &'a self,
        id: &'a str,
        body: ComponentRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_components_with_id_put(self.realm, id, body)
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
    pub fn components_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_with_id_delete(self.realm, id)
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
    pub fn components_with_id_sub_component_types_get(
        &'a self,
        id: &'a str,
    ) -> RealmComponentsWithIdSubComponentTypesGet<'a, TS> {
        RealmComponentsWithIdSubComponentTypesGet {
            realm_admin: self,
            id,
        }
    }
}

// <h4>Component</h4>
pub struct RealmComponentsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmComponentsGetArgs {
    pub name: Option<String>,
    pub parent: Option<String>,
    pub type_: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmComponentsGet<'a, TS>
{
    type Output = TypeVec<ComponentRepresentation>;
    type Args = RealmComponentsGetArgs;

    fn opts(
        self,
        Self::Args {
            name,
            parent,
            type_,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_components_get(self.realm_admin.realm, name, parent, type_)
    }
}

impl<'a, TS> IntoFuture for RealmComponentsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<ComponentRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmComponentsWithIdSubComponentTypesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub id: &'a str,
}

#[derive(Default)]
pub struct RealmComponentsWithIdSubComponentTypesGetArgs {
    pub type_: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmComponentsWithIdSubComponentTypesGet<'a, TS>
{
    type Output = TypeVec<ComponentTypeRepresentation>;
    type Args = RealmComponentsWithIdSubComponentTypesGetArgs;

    fn opts(
        self,
        Self::Args { type_ }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_components_with_id_sub_component_types_get(
                self.realm_admin.realm,
                self.id,
                type_,
            )
    }
}

impl<'a, TS> IntoFuture for RealmComponentsWithIdSubComponentTypesGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Component</h4>
    impl<'a, TS> RealmComponentsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().name(value)
        }
        pub fn parent(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().parent(value)
        }
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
    }

    impl<TS> Builder<'_, RealmComponentsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.name = value.into();
            self
        }
        pub fn parent(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.parent = value.into();
            self
        }
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
    }

    impl<'a, TS> RealmComponentsWithIdSubComponentTypesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn type_(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
    }

    impl<TS> Builder<'_, RealmComponentsWithIdSubComponentTypesGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn type_(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.type_ = value.into();
            self
        }
    }
}
