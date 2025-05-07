use reqwest::Response;

/// Default adapter around [Response].
pub struct DefaultResponse(Response);

impl DefaultResponse {
    /// Id of resource created.
    ///
    /// Extracted as last element of header `Location` if present.
    pub fn to_id(&self) -> Option<&'_ str> {
        self.0
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('/').next_back())
    }

    pub fn into_response(self) -> Response {
        self.0
    }
}

impl From<Response> for DefaultResponse {
    fn from(value: Response) -> Self {
        Self(value)
    }
}

impl AsRef<Response> for DefaultResponse {
    fn as_ref(&self) -> &Response {
        &self.0
    }
}

impl AsMut<Response> for DefaultResponse {
    fn as_mut(&mut self) -> &mut Response {
        &mut self.0
    }
}
