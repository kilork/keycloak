use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
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
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_get(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .get(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_download_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
        body: KeyStoreConfig,
    ) -> Result<TypeString, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/download",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.text().await.map(From::from)?)
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
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_generate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_generate_and_download_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
        body: KeyStoreConfig,
    ) -> Result<TypeString, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/generate-and-download",
                self.url
            ))
            .json(&body)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.text().await.map(From::from)?)
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
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_upload_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
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
    pub async fn realm_clients_with_client_uuid_certificates_with_attr_upload_certificate_post(
        &self,
        realm: &str,
        client_uuid: &str,
        attr: &str,
    ) -> Result<CertificateRepresentation, KeycloakError> {
        let realm = p(realm);
        let client_uuid = p(client_uuid);
        let attr = p(attr);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/clients/{client_uuid}/certificates/{attr}/upload-certificate",
                self.url
            ))
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
// not all paths processed
// left 241
