use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Attribute Certificate</h4>
    /// Get key info
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `GET /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_get_adminrealmsrealmclientsclient_uuidcertificatesattr>
    ///
    /// REST method: `GET /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}`
    pub fn clients_with_client_uuid_certificates_with_attr_get(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_get(
                self.realm,
                client_uuid,
                attr,
            )
    }

    /// Get a keystore file for the client, containing private key and public certificate
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    /// - `body`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrdownload>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/download`
    pub fn clients_with_client_uuid_certificates_with_attr_download_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
        body: KeyStoreConfig,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_download_post(
                self.realm,
                client_uuid,
                attr,
                body,
            )
    }

    /// Generate a new certificate with new key pair
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrgenerate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/generate`
    pub fn clients_with_client_uuid_certificates_with_attr_generate_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_generate_post(
                self.realm,
                client_uuid,
                attr,
            )
    }

    /// Generate a new keypair and certificate, and get the private key file
    ///
    /// Generates a keypair and certificate and serves the private key in a specified keystore format.
    /// Only generated public certificate is saved in Keycloak DB - the private key is not.
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    /// - `body`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate-and-download`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrgenerate_and_download>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/generate-and-download`
    pub fn clients_with_client_uuid_certificates_with_attr_generate_and_download_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
        body: KeyStoreConfig,
    ) -> impl Future<Output = Result<TypeString, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_generate_and_download_post(
                self.realm,
                client_uuid,
                attr,
                body,
            )
    }

    /// Upload certificate and eventually private key
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrupload>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/upload`
    pub fn clients_with_client_uuid_certificates_with_attr_upload_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_upload_post(
                self.realm,
                client_uuid,
                attr,
            )
    }

    /// Upload only certificate, not private key
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    ///
    /// Resource: `Client Attribute Certificate`
    ///
    /// `POST /admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload-certificate`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.3.2/rest-api/index.html#_post_adminrealmsrealmclientsclient_uuidcertificatesattrupload_certificate>
    ///
    /// REST method: `POST /admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/upload-certificate`
    pub fn clients_with_client_uuid_certificates_with_attr_upload_certificate_post(
        &'a self,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> impl Future<Output = Result<CertificateRepresentation, KeycloakError>> + use<'a, TS> {
        self.admin
            .realm_clients_with_client_uuid_certificates_with_attr_upload_certificate_post(
                self.realm,
                client_uuid,
                attr,
            )
    }
}
