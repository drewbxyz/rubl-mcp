use reqwest::Client;

use crate::api::endpoint::Endpoint;

const BASE_URL: &str = "https://api.ebird.org/v2";

#[derive(Clone)]
pub struct ApiClient {
    api_key: String,
    http: Client,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            http: Client::new(),
            api_key,
        }
    }

    pub async fn send<E: Endpoint>(&self, endpoint: E) -> Result<E::Response, reqwest::Error> {
        let url = format!("{}/{}", BASE_URL, endpoint.path());
        let request = self
            .http
            .request(E::METHOD, url)
            .query(endpoint.query())
            .header("X-eBirdApiToken", &self.api_key);

        let response = request.send().await?;
        response.json::<E::Response>().await
    }
}
