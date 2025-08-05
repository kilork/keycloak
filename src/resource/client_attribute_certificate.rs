use super::*;

impl<'a, TS: KeycloakTokenSupplier> KeycloakRealmAdmin<'a, TS> {
    // <h4>Client Attribute Certificate</h4>
    /// Get key info
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
