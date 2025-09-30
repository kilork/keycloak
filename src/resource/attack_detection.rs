use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Attack Detection</h4>
    /// Clear any user login failures for all users This can release temporary disabled users
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /admin/realms/{realm}/attack-detection/brute-force/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmattack_detectionbrute_forceusers>
    pub fn attack_detection_brute_force_users_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_delete(self.realm)
    }

    /// Get status of a username in brute force detection
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Attack Detection`
    ///
    /// `GET /admin/realms/{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmattack_detectionbrute_forceusersuserid>
    ///
    /// REST method: `GET /admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    pub fn attack_detection_brute_force_users_with_user_id_get(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<TypeMap<String, Value>, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_get(self.realm, user_id)
    }

    /// Clear any user login failures for the user This can release temporary disabled user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Attack Detection`
    ///
    /// `DELETE /admin/realms/{realm}/attack-detection/brute-force/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmattack_detectionbrute_forceusersuserid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    pub fn attack_detection_brute_force_users_with_user_id_delete(
        &'a self,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_attack_detection_brute_force_users_with_user_id_delete(self.realm, user_id)
    }
}
