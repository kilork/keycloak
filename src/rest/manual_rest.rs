#[allow(unused_imports)]
use super::{url_enc::encode_url_param as p, *};

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {
    /// Import identity provider from FORM MULTIPART body
    ///
    /// Parameters:
    ///
    /// - `realm`: realm name (not id!)
    /// - `provider_id`: identity provider type (for example `saml`)
    /// - `file`: metadata to parse (XML IDP Metadata for `saml`)
    ///
    /// Resource: `Identity Providers`
    ///
    /// `POST /admin/realms/{realm}/identity-provider/import-config`
    ///
    /// Documentation: <https://www.keycloak.org/docs-api/26.2.0/rest-api/index.html#_post_adminrealmsrealmidentity_providerimport_config>
    #[cfg(all(feature = "tag-identity-providers", feature = "multipart"))]
    pub async fn realm_identity_provider_import_config_post_form(
        &self,
        realm: &str,
        provider_id: String,
        file: Vec<u8>,
    ) -> Result<TypeMap<String, TypeString>, KeycloakError> {
        use reqwest::multipart::{Form, Part};

        let realm = p(realm);
        let file = Part::bytes(file).mime_str("application/octet-stream")?;
        let form = Form::new()
            .text("providerId", provider_id)
            .part("file", file);
        let builder = self
            .client
            .post(format!(
                "{}/admin/realms/{realm}/identity-provider/import-config",
                self.url
            ))
            .multipart(form)
            .bearer_auth(self.token_supplier.get(&self.url).await?);
        let response = builder.send().await?;
        Ok(error_check(response).await?.json().await?)
    }
}
