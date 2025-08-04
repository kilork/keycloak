use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Registration Policy</h4>
    /// Base path for retrieve providers with the configProperties properly filled
    pub fn client_registration_policy_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_registration_policy_providers_get(self.realm)
    }
}

// <h4>Client Registration Policy</h4>
#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Client Registration Policy</h4>
}
