use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Users</h4>

    /// Get users Returns a stream of users, filtered according to query parameters.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief representations are returned (default: false)
    /// - `email`: A String contained in email, or the complete email, if param "exact" is true
    /// - `email_verified`: whether the email has been verified
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `exact`: Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
    /// - `first`: Pagination offset
    /// - `first_name`: A String contained in firstName, or the complete firstName, if param "exact" is true
    /// - `idp_alias`: The alias of an Identity Provider linked to the user
    /// - `idp_user_id`: The userId at an Identity Provider linked to the user
    /// - `last_name`: A String contained in lastName, or the complete lastName, if param "exact" is true
    /// - `max`: Maximum results size (defaults to 100)
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    /// - `username`: A String contained in username, or the complete username, if param "exact" is true
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusers>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_users_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        email: Option<String>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        first_name: Option<String>,
        idp_alias: Option<String>,
        idp_user_id: Option<String>,
        last_name: Option<String>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<String>,
        username: Option<String>,
    ) -> Result<TypeVec<UserRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
        }
        if let Some(v) = email_verified {
            builder = builder.query(&[("emailVerified", v)]);
        }
        if let Some(v) = enabled {
            builder = builder.query(&[("enabled", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = first_name {
            builder = builder.query(&[("firstName", v)]);
        }
        if let Some(v) = idp_alias {
            builder = builder.query(&[("idpAlias", v)]);
        }
        if let Some(v) = idp_user_id {
            builder = builder.query(&[("idpUserId", v)]);
        }
        if let Some(v) = last_name {
            builder = builder.query(&[("lastName", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = username {
            builder = builder.query(&[("username", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Create a new user Username must be unique.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmusers>
    pub async fn realm_users_post(
        &self,
        realm: &str,
        body: UserRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/users", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the number of users that match the given criteria.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `email`: A String contained in email, or the complete email, if param "exact" is true
    /// - `email_verified`: whether the email has been verified
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `exact`: Boolean which defines whether the params "last", "first", "email" and "username" must match exactly
    /// - `first_name`: A String contained in firstName, or the complete firstName, if param "exact" is true
    /// - `idp_alias`: The alias of an Identity Provider linked to the user
    /// - `idp_user_id`: The userId at an Identity Provider linked to the user
    /// - `last_name`: A String contained in lastName, or the complete lastName, if param "exact" is true
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String contained in username, first or last name, or email. Default search behavior is prefix-based (e.g., foo or foo*). Use *foo* for infix search and "foo" for exact search.
    /// - `username`: A String contained in username, or the complete username, if param "exact" is true
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmuserscount>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_users_count_get(
        &self,
        realm: &str,
        email: Option<String>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        exact: Option<bool>,
        first_name: Option<String>,
        idp_alias: Option<String>,
        idp_user_id: Option<String>,
        last_name: Option<String>,
        q: Option<String>,
        search: Option<String>,
        username: Option<String>,
    ) -> Result<i32, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users/count", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = email {
            builder = builder.query(&[("email", v)]);
        }
        if let Some(v) = email_verified {
            builder = builder.query(&[("emailVerified", v)]);
        }
        if let Some(v) = enabled {
            builder = builder.query(&[("enabled", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first_name {
            builder = builder.query(&[("firstName", v)]);
        }
        if let Some(v) = idp_alias {
            builder = builder.query(&[("idpAlias", v)]);
        }
        if let Some(v) = idp_user_id {
            builder = builder.query(&[("idpUserId", v)]);
        }
        if let Some(v) = last_name {
            builder = builder.query(&[("lastName", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        if let Some(v) = username {
            builder = builder.query(&[("username", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/profile`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersprofile>
    pub async fn realm_users_profile_get(&self, realm: &str) -> Result<UPConfig, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users/profile", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/profile`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersprofile>
    pub async fn realm_users_profile_put(
        &self,
        realm: &str,
        body: UPConfig,
    ) -> Result<UPConfig, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/users/profile", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/profile/metadata`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersprofilemetadata>
    pub async fn realm_users_profile_metadata_get(
        &self,
        realm: &str,
    ) -> Result<UserProfileMetadata, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/profile/metadata",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get representation of the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `user_profile_metadata`: Indicates if the user profile metadata should be added to the response
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}`
    pub async fn realm_users_with_user_id_get(
        &self,
        realm: &str,
        user_id: &str,
        user_profile_metadata: Option<bool>,
    ) -> Result<UserRepresentation, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/users/{user_id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = user_profile_metadata {
            builder = builder.query(&[("userProfileMetadata", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Update the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}`
    pub async fn realm_users_with_user_id_put(
        &self,
        realm: &str,
        user_id: &str,
        body: UserRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .put(format!("{}/admin/realms/{realm}/users/{user_id}", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Delete the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmusersuser_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}`
    pub async fn realm_users_with_user_id_delete(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .delete(format!("{}/admin/realms/{realm}/users/{user_id}", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Return credential types, which are provided by the user storage where user is stored.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/configured-user-storage-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idconfigured_user_storage_credential_types>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/configured-user-storage-credential-types`
    pub async fn realm_users_with_user_id_configured_user_storage_credential_types_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<String>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/configured-user-storage-credential-types",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Get consents granted by the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/consents`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idconsents>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/consents`
    pub async fn realm_users_with_user_id_consents_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<TypeMap<String, Value>>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/consents",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Revoke consent and offline tokens for particular client from user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`: Client id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/consents/{client}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmusersuser_idconsentsclient>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/consents/{client}`
    pub async fn realm_users_with_user_id_consents_with_client_delete(
        &self,
        realm: &str,
        user_id: &str,
        client: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client = p(client);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/consents/{client}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/credentials`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idcredentials>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/credentials`
    pub async fn realm_users_with_user_id_credentials_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<CredentialRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove a credential for a user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmusersuser_idcredentialscredentialid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}`
    pub async fn realm_users_with_user_id_credentials_with_credential_id_delete(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Move a credential to a position behind another credential
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`: The credential to move
    /// - `new_previous_credential_id`: The credential that will be the previous element in the list. If set to null, the moved credential will be the first element in the list.
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmusersuser_idcredentialscredentialidmoveafternewpreviouscredentialid>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/moveAfter/{newPreviousCredentialId}`
    pub async fn realm_users_with_user_id_credentials_with_credential_id_move_after_with_new_previous_credential_id_post(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
        new_previous_credential_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let new_previous_credential_id = p(new_previous_credential_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveAfter/{new_previous_credential_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Move a credential to a first position in the credentials list of the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`: The credential to move
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveToFirst`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmusersuser_idcredentialscredentialidmovetofirst>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/moveToFirst`
    pub async fn realm_users_with_user_id_credentials_with_credential_id_move_to_first_post(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/moveToFirst",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Update a credential label for a user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/userLabel`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_idcredentialscredentialiduserlabel>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/userLabel`
    pub async fn realm_users_with_user_id_credentials_with_credential_id_user_label_put(
        &self,
        realm: &str,
        user_id: &str,
        credential_id: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let credential_id = p(credential_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/credentials/{credential_id}/userLabel",
                self.url
            ))
            .body(body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Disable all credentials for a user of a specific type
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/disable-credential-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_iddisable_credential_types>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/disable-credential-types`
    pub async fn realm_users_with_user_id_disable_credential_types_put(
        &self,
        realm: &str,
        user_id: &str,
        body: Vec<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/disable-credential-types",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Send an email to the user with a link they can click to execute particular actions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/execute-actions-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_idexecute_actions_email>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/execute-actions-email`
    pub async fn realm_users_with_user_id_execute_actions_email_put(
        &self,
        realm: &str,
        user_id: &str,
        client_id: Option<String>,
        lifespan: Option<i32>,
        redirect_uri: Option<String>,
        body: Vec<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/execute-actions-email",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("client_id", v)]);
        }
        if let Some(v) = lifespan {
            builder = builder.query(&[("lifespan", v)]);
        }
        if let Some(v) = redirect_uri {
            builder = builder.query(&[("redirect_uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get social logins associated with the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/federated-identity`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idfederated_identity>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/federated-identity`
    pub async fn realm_users_with_user_id_federated_identity_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<FederatedIdentityRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/federated-identity",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Add a social login provider to the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `provider`: Social login provider id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmusersuser_idfederated_identityprovider>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/federated-identity/{provider}`
    pub async fn realm_users_with_user_id_federated_identity_with_provider_post(
        &self,
        realm: &str,
        user_id: &str,
        provider: &str,
        body: FederatedIdentityRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let provider = p(provider);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/federated-identity/{provider}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Remove a social login provider from user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `provider`: Social login provider id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/federated-identity/{provider}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmusersuser_idfederated_identityprovider>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/federated-identity/{provider}`
    pub async fn realm_users_with_user_id_federated_identity_with_provider_delete(
        &self,
        realm: &str,
        user_id: &str,
        provider: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let provider = p(provider);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/federated-identity/{provider}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idgroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/groups`
    pub async fn realm_users_with_user_id_groups_get(
        &self,
        realm: &str,
        user_id: &str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<String>,
    ) -> Result<TypeVec<GroupRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `search`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/groups/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idgroupscount>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/groups/count`
    pub async fn realm_users_with_user_id_groups_count_get(
        &self,
        realm: &str,
        user_id: &str,
        search: Option<String>,
    ) -> Result<TypeMap<String, i64>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups/count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_idgroupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/groups/{groupId}`
    pub async fn realm_users_with_user_id_groups_with_group_id_put(
        &self,
        realm: &str,
        user_id: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let group_id = p(group_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups/{group_id}",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `DELETE /admin/realms/{realm}/users/{user_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmusersuser_idgroupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/users/{user-id}/groups/{groupId}`
    pub async fn realm_users_with_user_id_groups_with_group_id_delete(
        &self,
        realm: &str,
        user_id: &str,
        group_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let group_id = p(group_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/users/{user_id}/groups/{group_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Impersonate the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/impersonation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmusersuser_idimpersonation>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/impersonation`
    pub async fn realm_users_with_user_id_impersonation_post(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, Value>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/impersonation",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Remove all user sessions associated with the user Also send notification to all clients that have an admin URL to invalidate the sessions for the particular user.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `POST /admin/realms/{realm}/users/{user_id}/logout`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmusersuser_idlogout>
    ///
    /// REST method: `POST /admin/realms/{realm}/users/{user-id}/logout`
    pub async fn realm_users_with_user_id_logout_post(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/users/{user_id}/logout",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get offline sessions associated with the user and client
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_uuid`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/offline-sessions/{client_uuid}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idoffline_sessionsclientuuid>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/offline-sessions/{clientUuid}`
    pub async fn realm_users_with_user_id_offline_sessions_with_client_uuid_get(
        &self,
        realm: &str,
        user_id: &str,
        client_uuid: &str,
    ) -> Result<TypeVec<UserSessionRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let client_uuid = p(client_uuid);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/offline-sessions/{client_uuid}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Set up a new password for the user.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/reset-password`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_idreset_password>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/reset-password`
    pub async fn realm_users_with_user_id_reset_password_put(
        &self,
        realm: &str,
        user_id: &str,
        body: CredentialRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/reset-password",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Send an email to the user with a link they can click to reset their password.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id
    /// - `redirect_uri`: redirect uri
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/reset-password-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_idreset_password_email>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/reset-password-email`
    #[deprecated]
    pub async fn realm_users_with_user_id_reset_password_email_put(
        &self,
        realm: &str,
        user_id: &str,
        client_id: Option<String>,
        redirect_uri: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/reset-password-email",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("client_id", v)]);
        }
        if let Some(v) = redirect_uri {
            builder = builder.query(&[("redirect_uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Send an email-verification email to the user An email contains a link the user can click to verify their email address.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Users`
    ///
    /// `PUT /admin/realms/{realm}/users/{user_id}/send-verify-email`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmusersuser_idsend_verify_email>
    ///
    /// REST method: `PUT /admin/realms/{realm}/users/{user-id}/send-verify-email`
    pub async fn realm_users_with_user_id_send_verify_email_put(
        &self,
        realm: &str,
        user_id: &str,
        client_id: Option<String>,
        lifespan: Option<i32>,
        redirect_uri: Option<String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let mut builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/users/{user_id}/send-verify-email",
                self.url
            ))
            .header(CONTENT_LENGTH, "0")
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = client_id {
            builder = builder.query(&[("client_id", v)]);
        }
        if let Some(v) = lifespan {
            builder = builder.query(&[("lifespan", v)]);
        }
        if let Some(v) = redirect_uri {
            builder = builder.query(&[("redirect_uri", v)]);
        }
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Get sessions associated with the user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/sessions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idsessions>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/sessions`
    pub async fn realm_users_with_user_id_sessions_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeVec<UserSessionRepresentation>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/sessions",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    ///
    /// Resource: `Users`
    ///
    /// `GET /admin/realms/{realm}/users/{user_id}/unmanagedAttributes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmusersuser_idunmanagedattributes>
    ///
    /// REST method: `GET /admin/realms/{realm}/users/{user-id}/unmanagedAttributes`
    pub async fn realm_users_with_user_id_unmanaged_attributes_get(
        &self,
        realm: &str,
        user_id: &str,
    ) -> Result<TypeMap<String, TypeVec<String>>, KeycloakError> {
        let realm = p(realm);
        let user_id = p(user_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/users/{user_id}/unmanagedAttributes",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 219
