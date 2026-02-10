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

    let status = resp.status();
    let resp = resp.error_for_status()?;
    let body = resp.text().await?;
    let data = serde_json::from_str::<T>(&body).map_err(|e| {
        anyhow::anyhow!(
            "failed to decode eBird response (status {}): {}. body: {}",
            status,
            e,
            body.chars().take(400).collect::<String>()
        )
    })?;
    Ok(data)
}
