use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Key</h4>
    pub fn keys_get(
        &'a self,
    ) -> impl Future<Output = Result<KeysMetadataRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_keys_get(self.realm)
    }
}

// <h4>Key</h4>
#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Key</h4>
}
