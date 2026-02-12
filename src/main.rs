mod api;
mod content;
mod prompts;
mod server;

mod tools;

use anyhow::{Context, Result};
use rmcp::{ServiceExt, transport::stdio};

use server::RublClient;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("EBIRD_API_KEY")
        .context("EBIRD_API_KEY environment variable is required")?;

    let service = RublClient::new(api_key)
        .serve(stdio())
        .await
        .context("could not start mcp server")?;

    service
        .waiting()
        .await
        .context("something went wrong with the service")?;

    Ok(())
}
