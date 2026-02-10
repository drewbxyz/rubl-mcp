use reqwest::Client;
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://api.ebird.org/v2";

pub async fn get<T: DeserializeOwned>(
    client: &Client,
    path: &str,
    api_key: &str,
    params: &[(&str, &str)],
) -> anyhow::Result<T> {
    let resp = client
        .get(format!("{BASE_URL}{path}"))
        .header("X-eBirdApiToken", api_key)
        .query(params)
        .send()
        .await?;
    let data = resp.error_for_status()?.json::<T>().await?;
    Ok(data)
}
