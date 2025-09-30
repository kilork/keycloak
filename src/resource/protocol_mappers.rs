use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Protocol Mappers</h4>
    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/add-models`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_add_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_add_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_with_id_get(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<ProtocolMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_get(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Update the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_with_id_put(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_put(
                self.realm,
                client_scope_id,
                id,
                body,
            )
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/models/{id}`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_models_with_id_delete(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_models_with_id_delete(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-scopes/{client_scope_id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_scopesclient_scope_idprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/protocol/{protocol}`
    pub fn client_scopes_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
        &'a self,
        client_scope_id: &'a str,
        protocol: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_scopes_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
                self.realm,
                client_scope_id,
                protocol,
            )
    }

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/add-models`
    pub fn client_templates_with_client_scope_id_protocol_mappers_add_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_add_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models`
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_get(
        &'a self,
        client_scope_id: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_get(
                self.realm,
                client_scope_id,
            )
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models`
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_post(
        &'a self,
        client_scope_id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_post(
                self.realm,
                client_scope_id,
                body,
            )
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_with_id_get(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<ProtocolMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_get(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Update the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_with_id_put(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_put(
                self.realm,
                client_scope_id,
                id,
                body,
            )
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/models/{id}`
    pub fn client_templates_with_client_scope_id_protocol_mappers_models_with_id_delete(
        &'a self,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_models_with_id_delete(
                self.realm,
                client_scope_id,
                id,
            )
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/client-templates/{client_scope_id}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclient_templatesclient_scope_idprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/client-templates/{client-scope-id}/protocol-mappers/protocol/{protocol}`
    pub fn client_templates_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
        &'a self,
        client_scope_id: &'a str,
        protocol: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_client_templates_with_client_scope_id_protocol_mappers_protocol_with_protocol_get(
                self.realm,
                client_scope_id,
                protocol,
            )
    }

    /// Create multiple mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/add-models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidprotocol_mappersadd_models>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/add-models`
    pub fn clients_with_client_uuid_protocol_mappers_add_models_post(
        &'a self,
        client_uuid: &'a str,
        body: Vec<ProtocolMapperRepresentation>,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_add_models_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Get mappers
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersmodels>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    pub fn clients_with_client_uuid_protocol_mappers_models_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_get(self.realm, client_uuid)
    }

    /// Create a mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidprotocol_mappersmodels>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    pub fn clients_with_client_uuid_protocol_mappers_models_post(
        &'a self,
        client_uuid: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_post(
                self.realm,
                client_uuid,
                body,
            )
    }

    /// Get mapper by id
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    pub fn clients_with_client_uuid_protocol_mappers_models_with_id_get(
        &'a self,
        client_uuid: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<ProtocolMapperRepresentation, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_with_id_get(
                self.realm,
                client_uuid,
                id,
            )
    }

    /// Update the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    /// - `body`
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `PUT /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_put_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `PUT /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    pub fn clients_with_client_uuid_protocol_mappers_models_with_id_put(
        &'a self,
        client_uuid: &'a str,
        id: &'a str,
        body: ProtocolMapperRepresentation,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_with_id_put(
                self.realm,
                client_uuid,
                id,
                body,
            )
    }

    /// Delete the mapper
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    ///
    /// Returns response for future processing.
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `DELETE /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/models/{id}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_delete_adminrealmsrealmclientsclient_uuidprotocol_mappersmodelsid>
    ///
    /// REST method: `DELETE /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/{id}`
    pub fn clients_with_client_uuid_protocol_mappers_models_with_id_delete(
        &'a self,
        client_uuid: &'a str,
        id: &'a str,
    ) -> impl Future<Output = Result<DefaultResponse, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_with_id_delete(
                self.realm,
                client_uuid,
                id,
            )
    }

    /// Get mappers by name for a specific protocol
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `protocol`
    ///
    /// Resource: `Protocol Mappers`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/protocol-mappers/protocol/{protocol}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.4.0/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidprotocol_mappersprotocolprotocol>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/protocol/{protocol}`
    pub fn clients_with_client_uuid_protocol_mappers_protocol_with_protocol_get(
        &'a self,
        client_uuid: &'a str,
        protocol: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_protocol_with_protocol_get(
                self.realm,
                client_uuid,
                protocol,
            )
    }
}
