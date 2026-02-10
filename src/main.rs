mod ebird_api;
mod server;

mod tools;

use anyhow::{Context, Result};
use rmcp::{ServiceExt, transport::stdio};

use server::EbirdClient;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("EBIRD_API_KEY").expect("EBIRD_API_KEY environment variable is required");

    let service = EbirdClient::new(api_key)
        .serve(stdio())
        .await
        .with_context(|| format!("could not start mcp server"))?;

    service
        .waiting()
        .await
        .with_context(|| format!("something went wrong with the service:"))?;

    Ok(())
}
