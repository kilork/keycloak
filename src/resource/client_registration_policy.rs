use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Registration Policy</h4>
    /// Base path for retrieve providers with the configProperties properly filled
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Client Registration Policy`
    ///
    /// `GET /admin/realms/{realm}/client-registration-policy/providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_registration_policyproviders>
    pub fn client_registration_policy_providers_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ComponentTypeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_registration_policy_providers_get(self.realm)
    }
}
