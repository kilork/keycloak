use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Component</h4>
    pub fn components_get(&'a self) -> RealmComponentsGet<'a, TS> {
        RealmComponentsGet { realm_admin: self }
    }

    pub fn components_post(
        &'a self,
        body: ComponentRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_post(self.realm, body)
    }

    pub fn components_with_id_get(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<ComponentRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_with_id_get(self.realm, id)
    }

    pub fn components_with_id_put(
        &'a self,
        id: &'a str,
        body: ComponentRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_components_with_id_put(self.realm, id, body)
    }

    pub fn components_with_id_delete(
        &'a self,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_components_with_id_delete(self.realm, id)
    }

    /// List of subcomponent types that are available to configure for a particular parent component.
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

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmComponentsGet<'a, TS> {
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
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ComponentRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
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

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
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
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}
