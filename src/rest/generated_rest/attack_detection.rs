use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
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
    pub async fn realm_attack_detection_brute_force_users_delete(
        &self,
        realm: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
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
    pub async fn realm_attack_detection_brute_force_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users/{user_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_attack_detection_brute_force_users_with_user_id_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/attack-detection/brute-force/users/{user_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }
}
// not all paths processed
// left 245
