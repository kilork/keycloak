use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Key</h4>
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Key`
    ///
    /// `GET /admin/realms/{realm}/keys`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmkeys>
    pub fn keys_get(
        &'a self,
    ) -> impl Future<Output = Result<KeysMetadataRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_keys_get(self.realm)
    }
}
