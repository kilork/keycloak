use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Attack Detection</h4>
    /// Clear any user login failures for all users This can release temporary disabled users
    pub fn attack_detection_brute_force_users_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_delete(self.realm)
    }

    /// Get status of a username in brute force detection
    pub fn attack_detection_brute_force_users_with_user_id_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_get(self.realm, user_id)
    }

    /// Clear any user login failures for the user This can release temporary disabled user
    pub fn attack_detection_brute_force_users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_delete(self.realm, user_id)
    }
}

// <h4>Attack Detection</h4>
#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Attack Detection</h4>
}
