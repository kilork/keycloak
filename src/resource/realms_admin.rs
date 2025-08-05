use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Realms Admin</h4>
    /// Get the top-level representation of the realm It will not include nested information like User and Client representations.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealm>
    pub fn get(
        &'a self,
    ) -> impl Future<Output = Result<RealmRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_get(self.realm)
    }

    /// Update the top-level information of the realm Any user, roles or client information in the representation will be ignored.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealm>
    pub fn put(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_put(self.realm, body)
    }

    /// Delete the realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealm>
    pub fn delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_delete(self.realm)
    }

    /// Get admin events Returns all admin events, or filters events based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `auth_client`
    /// - `auth_ip_address`
    /// - `auth_realm`
    /// - `auth_user`: user id
    /// - `date_from`: From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `date_to`: To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `direction`: The direction to sort events by (asc or desc)
    /// - `first`
    /// - `max`: Maximum results size (defaults to 100)
    /// - `operation_types`
    /// - `resource_path`
    /// - `resource_types`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmadmin_events>
    pub fn admin_events_get(&'a self) -> RealmAdminEventsGet<'a, TS> {
        RealmAdminEventsGet { realm_admin: self }
    }

    /// Delete all admin events
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/admin-events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmadmin_events>
    pub fn admin_events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_admin_events_delete(self.realm)
    }

    /// Base path for importing clients under this realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/client-description-converter`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclient_description_converter>
    pub fn client_description_converter_post(
        &'a self,
        body: String,
    ) -> impl Future<Output = Result<ClientRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_description_converter_post(self.realm, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `include_global_policies`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_policiespolicies>
    pub fn client_policies_policies_get(&'a self) -> RealmClientPoliciesPoliciesGet<'a, TS> {
        RealmClientPoliciesPoliciesGet { realm_admin: self }
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/policies`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_policiespolicies>
    pub fn client_policies_policies_put(
        &'a self,
        body: ClientPoliciesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_policies_policies_put(self.realm, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `include_global_profiles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_policiesprofiles>
    pub fn client_policies_profiles_get(&'a self) -> RealmClientPoliciesProfilesGet<'a, TS> {
        RealmClientPoliciesProfilesGet { realm_admin: self }
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-policies/profiles`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_policiesprofiles>
    pub fn client_policies_profiles_put(
        &'a self,
        body: ClientProfilesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_policies_profiles_put(self.realm, body)
    }

    /// Get client session stats Returns a JSON map.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-session-stats`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_session_stats>
    pub fn client_session_stats_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<TypeMap<String, String>>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_client_session_stats_get(self.realm)
    }

    /// List all client types available in the current realm
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/client-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclient_types>
    pub fn client_types_get(
        &'a self,
    ) -> impl Future<Output = Result<ClientTypesRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_get(self.realm)
    }

    /// Update a client type
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/client-types`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmclient_types>
    pub fn client_types_put(
        &'a self,
        body: ClientTypesRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_client_types_put(self.realm, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/credential-registrators`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmcredential_registrators>
    pub fn credential_registrators_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin.realm_credential_registrators_get(self.realm)
    }

    /// Get realm default client scopes. Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-default-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_default_client_scopes>
    pub fn default_default_client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_default_default_client_scopes_get(self.realm)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    pub fn default_default_client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_default_client_scopes_with_client_scope_id_put(
                self.realm,
                client_scope_id,
            )
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-default-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_default_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    pub fn default_default_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_default_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_scope_id,
            )
    }

    /// Get group hierarchy.  Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-groups`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_groups>
    pub fn default_groups_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<GroupRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_default_groups_get(self.realm)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-groups/{groupId}`
    pub fn default_groups_with_group_id_put(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_groups_with_group_id_put(self.realm, group_id)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-groups/{group_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_groupsgroupid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-groups/{groupId}`
    pub fn default_groups_with_group_id_delete(
        &'a self,
        group_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_groups_with_group_id_delete(self.realm, group_id)
    }

    /// Get realm optional client scopes. Only name and ids are returned.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/default-optional-client-scopes`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmdefault_optional_client_scopes>
    pub fn default_optional_client_scopes_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<ClientScopeRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_default_optional_client_scopes_get(self.realm)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    pub fn default_optional_client_scopes_with_client_scope_id_put(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_optional_client_scopes_with_client_scope_id_put(
                self.realm,
                client_scope_id,
            )
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/default-optional-client-scopes/{client_scope_id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmdefault_optional_client_scopesclientscopeid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    pub fn default_optional_client_scopes_with_client_scope_id_delete(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_default_optional_client_scopes_with_client_scope_id_delete(
                self.realm,
                client_scope_id,
            )
    }

    /// Get events Returns all events, or filters them based on URL query parameters listed here
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client`: App or oauth client name
    /// - `date_from`: From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `date_to`: To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    /// - `direction`: The direction to sort events by (asc or desc)
    /// - `first`: Paging offset
    /// - `ip_address`: IP Address
    /// - `max`: Maximum results size (defaults to 100)
    /// - `type_`: The types of events to return
    /// - `user`: User id
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmevents>
    pub fn events_get(&'a self) -> RealmEventsGet<'a, TS> {
        RealmEventsGet { realm_admin: self }
    }

    /// Delete all events
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/events`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmevents>
    pub fn events_delete(
        &'a self,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_delete(self.realm)
    }

    /// Get the events provider configuration Returns JSON object with events provider configuration
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmeventsconfig>
    pub fn events_config_get(
        &'a self,
    ) -> impl Future<Output = Result<RealmEventsConfigRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin.realm_events_config_get(self.realm)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/events/config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmeventsconfig>
    pub fn events_config_put(
        &'a self,
        body: RealmEventsConfigRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_events_config_put(self.realm, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `path`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/group-by-path/{path}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmgroup_by_pathpath>
    pub fn group_by_path_with_path_get(
        &'a self,
        path: &'a str,
    ) -> impl Future<Output = Result<GroupRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_group_by_path_with_path_get(self.realm, path)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalization>
    pub fn localization_get(
        &'a self,
    ) -> impl Future<Output = Result<TypeVec<String>, KeycloakError>> + use<'a, TS> {
        self.admin.realm_localization_get(self.realm)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `use_realm_default_locale_fallback`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalizationlocale>
    pub fn localization_with_locale_get(
        &'a self,
        locale: &'a str,
    ) -> RealmLocalizationWithLocaleGet<'a, TS> {
        RealmLocalizationWithLocaleGet {
            realm_admin: self,
            locale,
        }
    }

    /// Import localization from uploaded JSON file
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmlocalizationlocale>
    pub fn localization_with_locale_post(
        &'a self,
        locale: &'a str,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_post(self.realm, locale, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `locale`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocale>
    pub fn localization_with_locale_delete(
        &'a self,
        locale: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_delete(self.realm, locale)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmlocalizationlocalekey>
    pub fn localization_with_locale_with_key_get(
        &'a self,
        key: &'a str,
        locale: &'a str,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_get(self.realm, key, locale)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmlocalizationlocalekey>
    pub fn localization_with_locale_with_key_put(
        &'a self,
        key: &'a str,
        locale: &'a str,
        body: String,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_put(self.realm, key, locale, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `key`
    /// - `locale`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/localization/{locale}/{key}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmlocalizationlocalekey>
    pub fn localization_with_locale_with_key_delete(
        &'a self,
        key: &'a str,
        locale: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_localization_with_locale_with_key_delete(self.realm, key, locale)
    }

    /// Removes all user sessions.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/logout-all`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmlogout_all>
    pub fn logout_all_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_logout_all_post(self.realm)
    }

    /// Partial export of existing realm into a JSON file.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `export_clients`
    /// - `export_groups_and_roles`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/partial-export`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpartial_export>
    pub fn partial_export_post(&'a self) -> RealmPartialExportPost<'a, TS> {
        RealmPartialExportPost { realm_admin: self }
    }

    /// Partial import from a JSON file to an existing realm.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/partialImport`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpartialimport>
    pub fn partial_import_post(
        &'a self,
        body: RealmRepresentation,
    ) -> impl Future<Output = Result<Value, KeycloakError>> + use<'a, TS> {
        self.admin.realm_partial_import_post(self.realm, body)
    }

    /// Push the realm's revocation policy to any client that has an admin url associated with it.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/push-revocation`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmpush_revocation>
    pub fn push_revocation_post(
        &'a self,
    ) -> impl Future<Output = Result<GlobalRequestResult, KeycloakError>> + use<'a, TS> {
        self.admin.realm_push_revocation_post(self.realm)
    }

    /// Remove a specific user session.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `session`
    /// - `is_offline`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `DELETE /admin/realms/{realm}/sessions/{session}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_delete_adminrealmsrealmsessionssession>
    pub fn sessions_with_session_delete(
        &'a self,
        session: &'a str,
    ) -> RealmSessionsWithSessionDelete<'a, TS> {
        RealmSessionsWithSessionDelete {
            realm_admin: self,
            session,
        }
    }

    /// Test SMTP connection with current logged in user
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Realms Admin`
    ///
    /// `POST /admin/realms/{realm}/testSMTPConnection`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmtestsmtpconnection>
    #[deprecated]
    pub fn test_smtp_connection_post(
        &'a self,
        body: TypeMap<String, String>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin.realm_test_smtp_connection_post(self.realm, body)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    ///
    /// Resource: `Realms Admin`
    ///
    /// `GET /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmusers_management_permissions>
    pub fn users_management_permissions_get(
        &'a self,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_management_permissions_get(self.realm)
    }

    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `body`
    ///
    /// Resource: `Realms Admin`
    ///
    /// `PUT /admin/realms/{realm}/users-management-permissions`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_put_adminrealmsrealmusers_management_permissions>
    pub fn users_management_permissions_put(
        &'a self,
        body: ManagementPermissionReference,
    ) -> impl Future<Output = Result<ManagementPermissionReference, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_users_management_permissions_put(self.realm, body)
    }
}

// <h4>Realms Admin</h4>
pub struct RealmAdminEventsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmAdminEventsGetArgs {
    pub auth_client: Option<String>,
    pub auth_ip_address: Option<String>,
    pub auth_realm: Option<String>,
    /// user id
    pub auth_user: Option<String>,
    /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_from: Option<String>,
    /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_to: Option<String>,
    /// The direction to sort events by (asc or desc)
    pub direction: Option<String>,
    pub first: Option<i32>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    pub operation_types: Option<Vec<String>>,
    pub resource_path: Option<String>,
    pub resource_types: Option<Vec<String>>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmAdminEventsGet<'a, TS> {
    type Output = TypeVec<AdminEventRepresentation>;
    type Args = RealmAdminEventsGetArgs;

    fn opts(
        self,
        Self::Args {
            auth_client,
            auth_ip_address,
            auth_realm,
            auth_user,
            date_from,
            date_to,
            direction,
            first,
            max,
            operation_types,
            resource_path,
            resource_types,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_admin_events_get(
            self.realm_admin.realm,
            auth_client,
            auth_ip_address,
            auth_realm,
            auth_user,
            date_from,
            date_to,
            direction,
            first,
            max,
            operation_types,
            resource_path,
            resource_types,
        )
    }
}

impl<'a, TS> IntoFuture for RealmAdminEventsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<AdminEventRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientPoliciesPoliciesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmClientPoliciesPoliciesGetArgs {
    pub include_global_policies: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientPoliciesPoliciesGet<'a, TS>
{
    type Output = ClientPoliciesRepresentation;
    type Args = RealmClientPoliciesPoliciesGetArgs;

    fn opts(
        self,
        Self::Args {
            include_global_policies,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_policies_policies_get(self.realm_admin.realm, include_global_policies)
    }
}

impl<'a, TS> IntoFuture for RealmClientPoliciesPoliciesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ClientPoliciesRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmClientPoliciesProfilesGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmClientPoliciesProfilesGetArgs {
    pub include_global_profiles: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmClientPoliciesProfilesGet<'a, TS>
{
    type Output = ClientProfilesRepresentation;
    type Args = RealmClientPoliciesProfilesGetArgs;

    fn opts(
        self,
        Self::Args {
            include_global_profiles,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin
            .admin
            .realm_client_policies_profiles_get(self.realm_admin.realm, include_global_profiles)
    }
}

impl<'a, TS> IntoFuture for RealmClientPoliciesProfilesGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<ClientProfilesRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmEventsGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmEventsGetArgs {
    /// App or oauth client name
    pub client: Option<String>,
    /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_from: Option<String>,
    /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
    pub date_to: Option<String>,
    /// The direction to sort events by (asc or desc)
    pub direction: Option<String>,
    /// Paging offset
    pub first: Option<i32>,
    /// IP Address
    pub ip_address: Option<String>,
    /// Maximum results size (defaults to 100)
    pub max: Option<i32>,
    /// The types of events to return
    pub type_: Option<Vec<String>>,
    /// User id
    pub user: Option<String>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmEventsGet<'a, TS> {
    type Output = TypeVec<EventRepresentation>;
    type Args = RealmEventsGetArgs;

    fn opts(
        self,
        Self::Args {
            client,
            date_from,
            date_to,
            direction,
            first,
            ip_address,
            max,
            type_,
            user,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_events_get(
            self.realm_admin.realm,
            client,
            date_from,
            date_to,
            direction,
            first,
            ip_address,
            max,
            type_,
            user,
        )
    }
}

impl<'a, TS> IntoFuture for RealmEventsGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeVec<EventRepresentation>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmLocalizationWithLocaleGet<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub locale: &'a str,
}

#[derive(Default)]
pub struct RealmLocalizationWithLocaleGetArgs {
    pub use_realm_default_locale_fallback: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmLocalizationWithLocaleGet<'a, TS>
{
    type Output = TypeMap<String, TypeString>;
    type Args = RealmLocalizationWithLocaleGetArgs;

    fn opts(
        self,
        Self::Args {
            use_realm_default_locale_fallback,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_localization_with_locale_get(
            self.realm_admin.realm,
            self.locale,
            use_realm_default_locale_fallback,
        )
    }
}

impl<'a, TS> IntoFuture for RealmLocalizationWithLocaleGet<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<TypeMap<String, TypeString>, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmPartialExportPost<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
}

#[derive(Default)]
pub struct RealmPartialExportPostArgs {
    pub export_clients: Option<bool>,
    pub export_groups_and_roles: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod for RealmPartialExportPost<'a, TS> {
    type Output = RealmRepresentation;
    type Args = RealmPartialExportPostArgs;

    fn opts(
        self,
        Self::Args {
            export_clients,
            export_groups_and_roles,
        }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_partial_export_post(
            self.realm_admin.realm,
            export_clients,
            export_groups_and_roles,
        )
    }
}

impl<'a, TS> IntoFuture for RealmPartialExportPost<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<RealmRepresentation, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

pub struct RealmSessionsWithSessionDelete<'a, TS: KeycloakTokenSupplier> {
    /// Realm admin client
    pub realm_admin: &'a KeycloakRealmAdmin<'a, TS>,
    pub session: &'a str,
}

#[derive(Default)]
pub struct RealmSessionsWithSessionDeleteArgs {
    pub is_offline: Option<bool>,
}

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdminMethod
    for RealmSessionsWithSessionDelete<'a, TS>
{
    type Output = DefaultResponse;
    type Args = RealmSessionsWithSessionDeleteArgs;

    fn opts(
        self,
        Self::Args { is_offline }: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, KeycloakError>> + use<'a, TS> {
        self.realm_admin.admin.realm_sessions_with_session_delete(
            self.realm_admin.realm,
            self.session,
            is_offline,
        )
    }
}

impl<'a, TS> IntoFuture for RealmSessionsWithSessionDelete<'a, TS>
where
    TS: KeycloakTokenSupplier,
{
    type Output = Result<DefaultResponse, KeycloakError>;
    type IntoFuture = Pin<Box<dyn 'a + Future<Output = Self::Output>>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.opts(Default::default()))
    }
}

#[cfg(feature = "builder")]
mod builder {
    use crate::builder::Builder;

    use super::*;

    // <h4>Realms Admin</h4>
    impl<'a, TS> RealmAdminEventsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn auth_client(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().auth_client(value)
        }
        pub fn auth_ip_address(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().auth_ip_address(value)
        }
        pub fn auth_realm(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().auth_realm(value)
        }
        /// user id
        pub fn auth_user(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().auth_user(value)
        }
        /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_from(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().date_from(value)
        }
        /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_to(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().date_to(value)
        }
        /// The direction to sort events by (asc or desc)
        pub fn direction(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().direction(value)
        }
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        pub fn operation_types(self, value: impl Into<Option<Vec<String>>>) -> Builder<'a, Self> {
            self.builder().operation_types(value)
        }
        pub fn resource_path(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().resource_path(value)
        }
        pub fn resource_types(self, value: impl Into<Option<Vec<String>>>) -> Builder<'a, Self> {
            self.builder().resource_types(value)
        }
    }

    impl<TS> Builder<'_, RealmAdminEventsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn auth_client(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.auth_client = value.into();
            self
        }
        pub fn auth_ip_address(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.auth_ip_address = value.into();
            self
        }
        pub fn auth_realm(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.auth_realm = value.into();
            self
        }
        /// user id
        pub fn auth_user(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.auth_user = value.into();
            self
        }
        /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_from(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.date_from = value.into();
            self
        }
        /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_to(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.date_to = value.into();
            self
        }
        /// The direction to sort events by (asc or desc)
        pub fn direction(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.direction = value.into();
            self
        }
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        pub fn operation_types(mut self, value: impl Into<Option<Vec<String>>>) -> Self {
            self.args.operation_types = value.into();
            self
        }
        pub fn resource_path(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.resource_path = value.into();
            self
        }
        pub fn resource_types(mut self, value: impl Into<Option<Vec<String>>>) -> Self {
            self.args.resource_types = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientPoliciesPoliciesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn include_global_policies(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().include_global_policies(value)
        }
    }

    impl<TS> Builder<'_, RealmClientPoliciesPoliciesGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn include_global_policies(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.include_global_policies = value.into();
            self
        }
    }

    impl<'a, TS> RealmClientPoliciesProfilesGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn include_global_profiles(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().include_global_profiles(value)
        }
    }

    impl<TS> Builder<'_, RealmClientPoliciesProfilesGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn include_global_profiles(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.include_global_profiles = value.into();
            self
        }
    }

    impl<'a, TS> RealmEventsGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        /// App or oauth client name
        pub fn client(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().client(value)
        }
        /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_from(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().date_from(value)
        }
        /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_to(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().date_to(value)
        }
        /// The direction to sort events by (asc or desc)
        pub fn direction(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().direction(value)
        }
        /// Paging offset
        pub fn first(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().first(value)
        }
        /// IP Address
        pub fn ip_address(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().ip_address(value)
        }
        /// Maximum results size (defaults to 100)
        pub fn max(self, value: impl Into<Option<i32>>) -> Builder<'a, Self> {
            self.builder().max(value)
        }
        /// The types of events to return
        pub fn type_(self, value: impl Into<Option<Vec<String>>>) -> Builder<'a, Self> {
            self.builder().type_(value)
        }
        /// User id
        pub fn user(self, value: impl Into<Option<String>>) -> Builder<'a, Self> {
            self.builder().user(value)
        }
    }

    impl<TS> Builder<'_, RealmEventsGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        /// App or oauth client name
        pub fn client(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.client = value.into();
            self
        }
        /// From (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_from(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.date_from = value.into();
            self
        }
        /// To (inclusive) date (yyyy-MM-dd) or time in Epoch timestamp millis (number of milliseconds since January 1, 1970, 00:00:00 GMT)
        pub fn date_to(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.date_to = value.into();
            self
        }
        /// The direction to sort events by (asc or desc)
        pub fn direction(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.direction = value.into();
            self
        }
        /// Paging offset
        pub fn first(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.first = value.into();
            self
        }
        /// IP Address
        pub fn ip_address(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.ip_address = value.into();
            self
        }
        /// Maximum results size (defaults to 100)
        pub fn max(mut self, value: impl Into<Option<i32>>) -> Self {
            self.args.max = value.into();
            self
        }
        /// The types of events to return
        pub fn type_(mut self, value: impl Into<Option<Vec<String>>>) -> Self {
            self.args.type_ = value.into();
            self
        }
        /// User id
        pub fn user(mut self, value: impl Into<Option<String>>) -> Self {
            self.args.user = value.into();
            self
        }
    }

    impl<'a, TS> RealmLocalizationWithLocaleGet<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn use_realm_default_locale_fallback(
            self,
            value: impl Into<Option<bool>>,
        ) -> Builder<'a, Self> {
            self.builder().use_realm_default_locale_fallback(value)
        }
    }

    impl<TS> Builder<'_, RealmLocalizationWithLocaleGet<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn use_realm_default_locale_fallback(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.use_realm_default_locale_fallback = value.into();
            self
        }
    }

    impl<'a, TS> RealmPartialExportPost<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn export_clients(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().export_clients(value)
        }
        pub fn export_groups_and_roles(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().export_groups_and_roles(value)
        }
    }

    impl<TS> Builder<'_, RealmPartialExportPost<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn export_clients(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.export_clients = value.into();
            self
        }
        pub fn export_groups_and_roles(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.export_groups_and_roles = value.into();
            self
        }
    }

    impl<'a, TS> RealmSessionsWithSessionDelete<'a, TS>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn is_offline(self, value: impl Into<Option<bool>>) -> Builder<'a, Self> {
            self.builder().is_offline(value)
        }
    }

    impl<TS> Builder<'_, RealmSessionsWithSessionDelete<'_, TS>>
    where
        TS: KeycloakTokenSupplier,
    {
        pub fn is_offline(mut self, value: impl Into<Option<bool>>) -> Self {
            self.args.is_offline = value.into();
            self
        }
    }
}
