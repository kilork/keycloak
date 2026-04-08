use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizations>
    pub fn organizations_get(&'a self) -> RealmOrganizationsGet<'a, TS> {
        RealmOrganizationsGet { realm_admin: self }
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizations>
    pub fn organizations_post(
        &'a self,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_organizations_post(self.realm, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationscount>
    pub fn organizations_count_get(&'a self) -> RealmOrganizationsCountGet<'a, TS> {
        RealmOrganizationsCountGet { realm_admin: self }
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/members/{member-id}/organizations`
    pub fn organizations_members_with_member_id_organizations_get(
        &'a self,
        member_id: &'a str,
    ) -> RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS> {
        RealmOrganizationsMembersWithMemberIdOrganizationsGet {
            realm_admin: self,
            member_id,
        }
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}`
    pub fn organizations_with_org_id_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<OrganizationRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_get(self.realm, org_id)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_put_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/organizations/{org-id}`
    pub fn organizations_with_org_id_put(
        &'a self,
        org_id: &'a str,
        body: OrganizationRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_put(self.realm, org_id, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}`
    pub fn organizations_with_org_id_delete(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_delete(self.realm, org_id)
    }

    /// Get organization groups
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `brief_representation`
    /// - `exact`
    /// - `first`
    /// - `max`
    /// - `populate_hierarchy`
    /// - `q`
    /// - `search`
    /// - `sub_groups_count`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idgroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/groups`
    pub fn organizations_with_org_id_groups_get(
        &'a self,
        org_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdGroupsGet<'a, TS> {
        RealmOrganizationsWithOrgIdGroupsGet {
            realm_admin: self,
            org_id,
        }
    }

    /// Creates a new top-level group or moves an existing group to top-level
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
    /// `POST /admin/realms/{realm}/organizations/{org_id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idgroups>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/groups`
    pub fn organizations_with_org_id_groups_post(
        &'a self,
        org_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_groups_post(self.realm, org_id, body)
    }

    /// Get organization group by path
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `path`
    /// - `sub_groups_count`: Whether to return the count of subgroups (default: false)
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/groups/group-by-path/{path}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idgroupsgroup_by_pathpath>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/groups/group-by-path/{path}`
    pub fn organizations_with_org_id_groups_group_by_path_with_path_get(
        &'a self,
        org_id: &'a str,
        path: &'a str,
    ) -> RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet<'a, TS> {
        RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet {
            realm_admin: self,
            org_id,
            path,
        }
    }

    /// Get organization group representation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `sub_groups_count`: Whether to return the count of subgroups (default: false)
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idgroupsgroup_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}`
    pub fn organizations_with_org_id_groups_with_group_id_get(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdGroupsWithGroupIdGet<'a, TS> {
        RealmOrganizationsWithOrgIdGroupsWithGroupIdGet {
            realm_admin: self,
            org_id,
            group_id,
        }
    }

    /// Update organization group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `PUT /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_put_adminrealmsrealmorganizationsorg_idgroupsgroup_id>
    ///
    /// REST method: `PUT /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}`
    pub fn organizations_with_org_id_groups_with_group_id_put(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_groups_with_group_id_put(
                self.realm, org_id, group_id, body,
            )
    }

    /// Delete the organization group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idgroupsgroup_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}`
    pub fn organizations_with_org_id_groups_with_group_id_delete(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_groups_with_group_id_delete(
                self.realm, org_id, group_id,
            )
    }

    /// Get subgroups of this organization group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `exact`: Boolean which defines whether the params "search" must match exactly or not
    /// - `first`: The position of the first result to be returned (pagination offset).
    /// - `max`: The maximum number of results that are to be returned. Defaults to 10
    /// - `search`: A String representing either an exact group name or a partial name
    /// - `sub_groups_count`: Whether to return the count of subgroups (default: false)
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idgroupsgroup_idchildren>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}/children`
    pub fn organizations_with_org_id_groups_with_group_id_children_get(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet<'a, TS> {
        RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet {
            realm_admin: self,
            org_id,
            group_id,
        }
    }

    /// Create or move a subgroup
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}/children`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idgroupsgroup_idchildren>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}/children`
    pub fn organizations_with_org_id_groups_with_group_id_children_post(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
        body: GroupRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_groups_with_group_id_children_post(
                self.realm, org_id, group_id, body,
            )
    }

    /// Get members of this organization group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `brief_representation`: Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}/members`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idgroupsgroup_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}/members`
    pub fn organizations_with_org_id_groups_with_group_id_members_get(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet<'a, TS> {
        RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet {
            realm_admin: self,
            org_id,
            group_id,
        }
    }

    /// Add a user to this organization group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `PUT /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}/members/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_put_adminrealmsrealmorganizationsorg_idgroupsgroup_idmembersuserid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}/members/{userId}`
    pub fn organizations_with_org_id_groups_with_group_id_members_with_user_id_put(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_groups_with_group_id_members_with_user_id_put(
                self.realm, org_id, group_id, user_id,
            )
    }

    /// Remove a user from this organization group
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `group_id`
    /// - `user_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/groups/{group_id}/members/{user_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idgroupsgroup_idmembersuserid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/groups/{group-id}/members/{userId}`
    pub fn organizations_with_org_id_groups_with_group_id_members_with_user_id_delete(
        &'a self,
        org_id: &'a str,
        group_id: &'a str,
        user_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_groups_with_group_id_members_with_user_id_delete(
                self.realm, org_id, group_id, user_id,
            )
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    pub fn organizations_with_org_id_identity_providers_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<IdentityProviderRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_get(self.realm, org_id)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_ididentity_providers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/identity-providers`
    pub fn organizations_with_org_id_identity_providers_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_post(self.realm, org_id, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    pub fn organizations_with_org_id_identity_providers_with_alias_get(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> impl Future<Output = Result<IdentityProviderRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_identity_providers_with_alias_get(
                self.realm, org_id, alias,
            )
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_ididentity_providersalias>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}`
    pub fn organizations_with_org_id_identity_providers_with_alias_delete(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_identity_providers_with_alias_delete(
                self.realm, org_id, alias,
            )
    }

    /// Returns organization groups for the identity provider
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `alias`: The alias of the identity provider
    /// - `brief_representation`: If true, return brief representation; otherwise return full representation
    /// - `exact`: If true, perform exact match on the search parameter
    /// - `first`: The position of the first result (pagination offset)
    /// - `max`: The maximum number of results to return
    /// - `q`: A query to search for group attributes, in the format 'key1:value1 key2:value2'
    /// - `search`: A string to search for in group names
    /// - `sub_groups_count`: If true, include subgroups count in the response
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/identity-providers/{alias}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_ididentity_providersaliasgroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/identity-providers/{alias}/groups`
    pub fn organizations_with_org_id_identity_providers_with_alias_groups_get(
        &'a self,
        org_id: &'a str,
        alias: &'a str,
    ) -> RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet<'a, TS> {
        RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet {
            realm_admin: self,
            org_id,
            alias,
        }
    }

    /// Get invitations for the organization
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `email`
    /// - `first`
    /// - `first_name`
    /// - `last_name`
    /// - `max`
    /// - `search`
    /// - `status`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/invitations`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idinvitations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/invitations`
    pub fn organizations_with_org_id_invitations_get(
        &'a self,
        org_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdInvitationsGet<'a, TS> {
        RealmOrganizationsWithOrgIdInvitationsGet {
            realm_admin: self,
            org_id,
        }
    }

    /// Get invitation by ID
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `id`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/invitations/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idinvitationsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/invitations/{id}`
    pub fn organizations_with_org_id_invitations_with_id_get(
        &'a self,
        org_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<OrganizationInvitationRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_organizations_with_org_id_invitations_with_id_get(self.realm, org_id, id)
    }

    /// Delete an invitation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `DELETE /admin/realms/{realm}/organizations/{org_id}/invitations/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idinvitationsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/invitations/{id}`
    pub fn organizations_with_org_id_invitations_with_id_delete(
        &'a self,
        org_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_invitations_with_id_delete(self.realm, org_id, id)
    }

    /// Resend an invitation
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Organizations`
    ///
    /// `POST /admin/realms/{realm}/organizations/{org_id}/invitations/{id}/resend`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idinvitationsidresend>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/invitations/{id}/resend`
    pub fn organizations_with_org_id_invitations_with_id_resend_post(
        &'a self,
        org_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_invitations_with_id_resend_post(self.realm, org_id, id)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members`
    pub fn organizations_with_org_id_members_get(
        &'a self,
        org_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersGet {
            realm_admin: self,
            org_id,
        }
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembers>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members`
    pub fn organizations_with_org_id_members_post(
        &'a self,
        org_id: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_post(self.realm, org_id, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmemberscount>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/count`
    pub fn organizations_with_org_id_members_count_get(
        &'a self,
        org_id: &'a str,
    ) -> impl Future<Output = Result<i64, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_count_get(self.realm, org_id)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_existing_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-existing-user`
    pub fn organizations_with_org_id_members_invite_existing_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_existing_user_post(
                self.realm, org_id, body,
            )
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_post_adminrealmsrealmorganizationsorg_idmembersinvite_user>
    ///
    /// REST method: `POST /admin/realms/{realm}/organizations/{org-id}/members/invite-user`
    pub fn organizations_with_org_id_members_invite_user_post(
        &'a self,
        org_id: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_invite_user_post(self.realm, org_id, body)
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    pub fn organizations_with_org_id_members_with_member_id_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> impl Future<Output = Result<MemberRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_with_member_id_get(
                self.realm, org_id, member_id,
            )
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_delete_adminrealmsrealmorganizationsorg_idmembersmember_id>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/organizations/{org-id}/members/{member-id}`
    pub fn organizations_with_org_id_members_with_member_id_delete(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_organizations_with_org_id_members_with_member_id_delete(
                self.realm, org_id, member_id,
            )
    }

    /// Returns the organization group memberships for a member with the specified id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `org_id`
    /// - `member_id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    ///
    /// Resource: `Organizations`
    ///
    /// `GET /admin/realms/{realm}/organizations/{org_id}/members/{member_id}/groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_idgroups>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}/groups`
    pub fn organizations_with_org_id_members_with_member_id_groups_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet {
            realm_admin: self,
            org_id,
            member_id,
        }
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
    /// Documentation: <https://www.keycloak.org/docs-api/26.6.0/rest-api/index.html#_get_adminrealmsrealmorganizationsorg_idmembersmember_idorganizations>
    ///
    /// REST method: `GET /admin/realms/{realm}/organizations/{org-id}/members/{member-id}/organizations`
    pub fn organizations_with_org_id_members_with_member_id_organizations_get(
        &'a self,
        org_id: &'a str,
        member_id: &'a str,
    ) -> RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS> {
        RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet {
            realm_admin: self,
            org_id,
            member_id,
        }
    }
}

// <h4>Organizations</h4>
pub struct RealmOrganizationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned - defaults to 10
    pub max: Option<i32>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String representing either an organization name or domain
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_organizations_get(
            self.realm_admin.realm,
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
        )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsCountGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmOrganizationsCountGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
    pub q: Option<String>,
    /// A String representing either an organization name or domain
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsCountGet<'a, TS>
{
    type Output = i64;
    type Args = RealmOrganizationsCountGetArgs;

    fn opts(
        self,
        Self::Args { exact, q, search }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_organizations_count_get(
            self.realm_admin.realm,
            exact,
            q,
            search,
        )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsCountGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<i64, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub member_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsMembersWithMemberIdOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_members_with_member_id_organizations_get(
                self.realm_admin.realm,
                self.member_id,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdGroupsGetArgs {
    pub brief_representation: Option<bool>,
    pub exact: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub populate_hierarchy: Option<bool>,
    pub q: Option<String>,
    pub search: Option<String>,
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdGroupsGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmOrganizationsWithOrgIdGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            populate_hierarchy,
            q,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_groups_get(
                self.realm_admin.realm,
                self.org_id,
                brief_representation,
                exact,
                first,
                max,
                populate_hierarchy,
                q,
                search,
                sub_groups_count,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub path: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGetArgs {
    /// Whether to return the count of subgroups (default: false)
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet<'a, TS>
{
    type Output = GroupRepresentation;
    type Args = RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGetArgs;

    fn opts(
        self,
        Self::Args { sub_groups_count }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_groups_group_by_path_with_path_get(
                self.realm_admin.realm,
                self.org_id,
                self.path,
                sub_groups_count,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<GroupRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdGroupsWithGroupIdGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub group_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdGroupsWithGroupIdGetArgs {
    /// Whether to return the count of subgroups (default: false)
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdGroupsWithGroupIdGet<'a, TS>
{
    type Output = GroupRepresentation;
    type Args = RealmOrganizationsWithOrgIdGroupsWithGroupIdGetArgs;

    fn opts(
        self,
        Self::Args { sub_groups_count }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_groups_with_group_id_get(
                self.realm_admin.realm,
                self.org_id,
                self.group_id,
                sub_groups_count,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdGroupsWithGroupIdGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<GroupRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub group_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGetArgs {
    /// Boolean which defines whether the params "search" must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be returned (pagination offset).
    pub first: Option<i32>,
    /// The maximum number of results that are to be returned. Defaults to 10
    pub max: Option<i32>,
    /// A String representing either an exact group name or a partial name
    pub search: Option<String>,
    /// Whether to return the count of subgroups (default: false)
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGetArgs;

    fn opts(
        self,
        Self::Args {
            exact,
            first,
            max,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_groups_with_group_id_children_get(
                self.realm_admin.realm,
                self.org_id,
                self.group_id,
                exact,
                first,
                max,
                search,
                sub_groups_count,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub group_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGetArgs {
    /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
    pub brief_representation: Option<bool>,
    /// Pagination offset
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet<'a, TS>
{
    type Output = TypeVec<MemberRepresentation>;
    type Args = RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_groups_with_group_id_members_get(
                self.realm_admin.realm,
                self.org_id,
                self.group_id,
                brief_representation,
                first,
                max,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<MemberRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    /// The alias of the identity provider
    pub alias: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGetArgs {
    /// If true, return brief representation; otherwise return full representation
    pub brief_representation: Option<bool>,
    /// If true, perform exact match on the search parameter
    pub exact: Option<bool>,
    /// The position of the first result (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to return
    pub max: Option<i32>,
    /// A query to search for group attributes, in the format 'key1:value1 key2:value2'
    pub q: Option<String>,
    /// A string to search for in group names
    pub search: Option<String>,
    /// If true, include subgroups count in the response
    pub sub_groups_count: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            exact,
            first,
            max,
            q,
            search,
            sub_groups_count,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_identity_providers_with_alias_groups_get(
                self.realm_admin.realm,
                self.org_id,
                self.alias,
                brief_representation,
                exact,
                first,
                max,
                q,
                search,
                sub_groups_count,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdInvitationsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdInvitationsGetArgs {
    pub email: Option<String>,
    pub first: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub max: Option<i32>,
    pub search: Option<String>,
    pub status: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdInvitationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationInvitationRepresentation>;
    type Args = RealmOrganizationsWithOrgIdInvitationsGetArgs;

    fn opts(
        self,
        Self::Args {
            email,
            first,
            first_name,
            last_name,
            max,
            search,
            status,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_invitations_get(
                self.realm_admin.realm,
                self.org_id,
                email,
                first,
                first_name,
                last_name,
                max,
                search,
                status,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdInvitationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationInvitationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdMembersGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersGetArgs {
    /// Boolean which defines whether the param 'search' must match exactly or not
    pub exact: Option<bool>,
    /// The position of the first result to be processed (pagination offset)
    pub first: Option<i32>,
    /// The maximum number of results to be returned. Defaults to 10
    pub max: Option<i32>,
    /// The membership type
    pub membership_type: Option<String>,
    /// A String representing either a member's username, e-mail, first name, or last name.
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersGet<'a, TS>
{
    type Output = TypeVec<MemberRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersGetArgs;

    fn opts(
        self,
        Self::Args {
            exact,
            first,
            max,
            membership_type,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_get(
                self.realm_admin.realm,
                self.org_id,
                exact,
                first,
                max,
                membership_type,
                search,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<MemberRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub member_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGetArgs {
    pub brief_representation: Option<bool>,
    pub first: Option<i32>,
    pub max: Option<i32>,
    pub search: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet<'a, TS>
{
    type Output = TypeVec<GroupRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
            first,
            max,
            search,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_with_member_id_groups_get(
                self.realm_admin.realm,
                self.org_id,
                self.member_id,
                brief_representation,
                first,
                max,
                search,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<GroupRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<
    'a,
    TS: KeycloakTokenSupplier,
> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub org_id: &'a str,
    pub member_id: &'a str,
}

#[derive(Default)]
pub struct RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs {
    /// if false, return the full representation. Otherwise, only the basic fields are returned.
    pub brief_representation: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier + Send + Sync> KeycloakRealmAdminMethod
    for RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
{
    type Output = TypeVec<OrganizationRepresentation>;
    type Args = RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGetArgs;

    fn opts(
        self,
        Self::Args {
            brief_representation,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_organizations_with_org_id_members_with_member_id_organizations_get(
                self.realm_admin.realm,
                self.org_id,
                self.member_id,
                brief_representation,
            )
    }
}

impl<'a, TS> IntoFuture for RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
where
    TS: KeycloakTokenSupplier + Send + Sync,
{
    type Output = Result<TypeVec<OrganizationRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Organizations</h4>
    impl<'a, TS> RealmOrganizationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results to be returned - defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// A String representing either an organization name or domain
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results to be returned - defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// A String representing either an organization name or domain
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsCountGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// A String representing either an organization name or domain
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsCountGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// A query to search for custom attributes, in the format 'key1:value2 key2:value2'
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// A String representing either an organization name or domain
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsMembersWithMemberIdOrganizationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsMembersWithMemberIdOrganizationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn populate_hierarchy(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().populate_hierarchy(value)
        }
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn populate_hierarchy(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.populate_hierarchy = value.into();
            self
        }
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Whether to return the count of subgroups (default: false)
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdGroupsGroupByPathWithPathGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Whether to return the count of subgroups (default: false)
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdGroupsWithGroupIdGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Whether to return the count of subgroups (default: false)
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdGroupsWithGroupIdGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Whether to return the count of subgroups (default: false)
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the params "search" must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result to be returned (pagination offset).
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results that are to be returned. Defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A String representing either an exact group name or a partial name
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// Whether to return the count of subgroups (default: false)
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdGroupsWithGroupIdChildrenGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the params "search" must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result to be returned (pagination offset).
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results that are to be returned. Defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A String representing either an exact group name or a partial name
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// Whether to return the count of subgroups (default: false)
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// Pagination offset
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdGroupsWithGroupIdMembersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Only return basic information (only guaranteed to return id, username, created, first and last name, email, enabled state, email verification state, federation link, and access. Note that it means that namely user attributes, required actions, and not before are not returned.)
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// Pagination offset
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// If true, return brief representation; otherwise return full representation
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        /// If true, perform exact match on the search parameter
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result (pagination offset)
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results to return
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// A query to search for group attributes, in the format 'key1:value1 key2:value2'
        pub fn q(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().q(value)
        }
        /// A string to search for in group names
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        /// If true, include subgroups count in the response
        pub fn sub_groups_count(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().sub_groups_count(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdIdentityProvidersWithAliasGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// If true, return brief representation; otherwise return full representation
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        /// If true, perform exact match on the search parameter
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result (pagination offset)
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results to return
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// A query to search for group attributes, in the format 'key1:value1 key2:value2'
        pub fn q(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.q = value.into();
            self
        }
        /// A string to search for in group names
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        /// If true, include subgroups count in the response
        pub fn sub_groups_count(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.sub_groups_count = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdInvitationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn email(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().email(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn first_name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().first_name(value)
        }
        pub fn last_name(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().last_name(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
        pub fn status(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().status(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdInvitationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn email(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.email = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn first_name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.first_name = value.into();
            self
        }
        pub fn last_name(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.last_name = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
        pub fn status(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.status = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdMembersGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().exact(value)
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// The maximum number of results to be returned. Defaults to 10
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// The membership type
        pub fn membership_type(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().membership_type(value)
        }
        /// A String representing either a member's username, e-mail, first name, or last name.
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdMembersGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// Boolean which defines whether the param 'search' must match exactly or not
        pub fn exact(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.exact = value.into();
            self
        }
        /// The position of the first result to be processed (pagination offset)
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// The maximum number of results to be returned. Defaults to 10
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// The membership type
        pub fn membership_type(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.membership_type = value.into();
            self
        }
        /// A String representing either a member's username, e-mail, first name, or last name.
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn search(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().search(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdMembersWithMemberIdGroupsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn search(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.search = value.into();
            self
        }
    }

    impl<'a, TS> RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().brief_representation(value)
        }
    }

    impl<TS> Builder<'_, RealmOrganizationsWithOrgIdMembersWithMemberIdOrganizationsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier + Send + Sync,
    {
        /// if false, return the full representation. Otherwise, only the basic fields are returned.
        pub fn brief_representation(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.brief_representation = value.into();
            self
        }
    }
}
