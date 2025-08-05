use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    // <h4>Organizations</h4>

    /// Returns a paginated list of organizations filtered according to the specified parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned - defaults to 10
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String representing either an organization name or domain
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizations>
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_organizations_get(
        &self,
        realm: &str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        q: Option<String>,
        search: Option<String>,
    ) -> Result<TypeVec<OrganizationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!("{}/admin/realms/{realm}/organizations", self.url))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
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
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Creates a new organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizations>
    pub async fn realm_organizations_post(
        &self,
        realm: &str,
        body: OrganizationRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let builder = self
            .client
            .post(format!("{}/admin/realms/{realm}/organizations", self.url))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the organizations counts.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `q`: A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    /// - `search`: A String representing either an organization name or domain
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationscount>
    pub async fn realm_organizations_count_get(
        &self,
        realm: &str,
        exact: Option<bool>,
        q: Option<String>,
        search: Option<String>,
    ) -> Result<i64, KeycloakError> {
        let realm = p(realm);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = q {
            builder = builder.query(&[("q", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns the organizations associated with the user that has the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `member_id`
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/members/{member_id}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/members/{member-id}/organizations`
    pub async fn realm_organizations_members_with_member_id_organizations_get(
        &self,
        realm: &str,
        member_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<OrganizationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let member_id = p(member_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/members/{member_id}/organizations",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Returns the organization representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}`
    pub async fn realm_organizations_with_org_id_get(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<OrganizationRepresentation, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Updates the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `PUT /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/organizations/{org-id}`
    pub async fn realm_organizations_with_org_id_put(
        &self,
        realm: &str,
        org_id: &str,
        body: OrganizationRepresentation,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .put(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Deletes the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}`
    pub async fn realm_organizations_with_org_id_delete(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns all identity providers associated with the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    pub async fn realm_organizations_with_org_id_identity_providers_get(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<TypeVec<IdentityProviderRepresentation>, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Adds the identity provider with the specified id to the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/identity-providers`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    pub async fn realm_organizations_with_org_id_identity_providers_post(
        &self,
        realm: &str,
        org_id: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the identity provider associated with the organization that has the specified alias
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    pub async fn realm_organizations_with_org_id_identity_providers_with_alias_get(
        &self,
        realm: &str,
        org_id: &str,
        alias: &str,
    ) -> Result<IdentityProviderRepresentation, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let alias = p(alias);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Removes the identity provider with the specified alias from the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    pub async fn realm_organizations_with_org_id_identity_providers_with_alias_delete(
        &self,
        realm: &str,
        org_id: &str,
        alias: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let alias = p(alias);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns a paginated list of organization members filtered according to the specified parameters
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `exact`: Boolean which defines whether the param 'search' must match exactly or not
    /// - `first`: The position of the first result to be processed (pagination offset)
    /// - `max`: The maximum number of results to be returned. Defaults to 10
    /// - `membership_type`: The membership type
    /// - `search`: A String representing either a member's username, e-mail, first name, or last name.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members`
    #[allow(clippy::too_many_arguments)]
    pub async fn realm_organizations_with_org_id_members_get(
        &self,
        realm: &str,
        org_id: &str,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        membership_type: Option<String>,
        search: Option<String>,
    ) -> Result<TypeVec<MemberRepresentation>, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = exact {
            builder = builder.query(&[("exact", v)]);
        }
        if let Some(v) = first {
            builder = builder.query(&[("first", v)]);
        }
        if let Some(v) = max {
            builder = builder.query(&[("max", v)]);
        }
        if let Some(v) = membership_type {
            builder = builder.query(&[("membershipType", v)]);
        }
        if let Some(v) = search {
            builder = builder.query(&[("search", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Adds the user with the specified id as a member of the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members`
    pub async fn realm_organizations_with_org_id_members_post(
        &self,
        realm: &str,
        org_id: &str,
        body: String,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns number of members in the organization.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/count`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmemberscount>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/count`
    pub async fn realm_organizations_with_org_id_members_count_get(
        &self,
        realm: &str,
        org_id: &str,
    ) -> Result<i64, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/count",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Invites an existing user to the organization, using the specified user id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members/invite-existing-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_existing_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-existing-user`
    pub async fn realm_organizations_with_org_id_members_invite_existing_user_post(
        &self,
        realm: &str,
        org_id: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/invite-existing-user",
                self.url
            ))
            .form(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Invites an existing user or sends a registration link to a new user, based on the provided e-mail address.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/members/invite-user`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-user`
    pub async fn realm_organizations_with_org_id_members_invite_user_post(
        &self,
        realm: &str,
        org_id: &str,
        body: TypeMap<String, String>,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/invite-user",
                self.url
            ))
            .form(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the member of the organization with the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    pub async fn realm_organizations_with_org_id_members_with_member_id_get(
        &self,
        realm: &str,
        org_id: &str,
        member_id: &str,
    ) -> Result<MemberRepresentation, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let member_id = p(member_id);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/{member_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }

    /// Removes the user with the specified id from the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/members/{member_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    pub async fn realm_organizations_with_org_id_members_with_member_id_delete(
        &self,
        realm: &str,
        org_id: &str,
        member_id: &str,
    ) -> Result<DefaultResponse, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let member_id = p(member_id);
        let builder = self
            .client
            .delete(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/{member_id}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        error_check(response).await.map(From::from)
    }

    /// Returns the organizations associated with the user that has the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    /// - `brief_representation`: if false, return the full representation. Otherwise, only the basic fields are returned.
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}/organizations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}/organizations`
    pub async fn realm_organizations_with_org_id_members_with_member_id_organizations_get(
        &self,
        realm: &str,
        org_id: &str,
        member_id: &str,
        brief_representation: Option<bool>,
    ) -> Result<TypeVec<OrganizationRepresentation>, KeycloakError> {
        let realm = p(realm);
        let org_id = p(org_id);
        let member_id = p(member_id);
        let mut builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/organizations/{org_id}/members/{member_id}/organizations",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        if let Some(v) = brief_representation {
            builder = builder.query(&[("briefRepresentation", v)]);
        }
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 235
