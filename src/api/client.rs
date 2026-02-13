use reqwest::Client;

use crate::api::endpoint::Endpoint;
use crate::api::error::ApiError;

const BASE_URL: &str = "https://api.ebird.org/v2";

/// HTTP client for authenticated eBird API requests.
///
/// # Examples
///
/// ```no_run
/// use rubl::api::client::ApiClient;
///
/// let client = ApiClient::new("your-api-key");
/// ```
#[derive(Clone)]
pub struct ApiClient {
    api_key: String,
    http: Client,
}

impl ApiClient {
    /// Creates a new API client.
    ///
    /// Get your API key from: https://ebird.org/api/keygen
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rubl::api::client::ApiClient;
    ///
    /// let client = ApiClient::new("your-api-key");
    /// let client = ApiClient::new("your-api-key".to_string());
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            api_key: api_key.into(),
        }
    }

    /// Sends an API request and deserializes the response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rubl::api::client::ApiClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ApiClient::new("your-api-key");
    /// // let response = client.send(&some_endpoint).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send<E: Endpoint>(&self, endpoint: &E) -> Result<E::Response, ApiError> {
        let url = format!("{}/{}", BASE_URL, endpoint.path());
        let mut request = self.http.request(E::METHOD, url);
        if let Some(fmt) = endpoint.format() {
            request = request.query(&[("fmt", fmt)]);
        }
        let request = request
            .query(endpoint.query())
            .header("X-eBirdApiToken", &self.api_key);

        let response = request.send().await.map_err(ApiError::Network)?;

        // Check status code before attempting deserialization
        let status = response.status();
        if !status.is_success() {
            // Capture the error response body for better error messages
            let body = response.text().await.unwrap_or_else(|_| String::from("(unable to read response body)"));
            return Err(ApiError::HttpError { status, body });
        }

        response.json::<E::Response>().await.map_err(ApiError::Deserialization)
    }
}
