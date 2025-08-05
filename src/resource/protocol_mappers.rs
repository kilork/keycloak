use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Protocol Mappers</h4>
    /// Create multiple mappers
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
    pub fn clients_with_client_uuid_protocol_mappers_models_get(
        &'a self,
        client_uuid: &'a str,
    ) -> impl Future<Output = Result<TypeVec<ProtocolMapperRepresentation>, KeycloakError>> + use<'a, TS>
    {
        self.admin
            .realm_clients_with_client_uuid_protocol_mappers_models_get(self.realm, client_uuid)
    }

    /// Create a mapper
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
