mod api;
mod content;
mod logging;
mod server;
mod tools;

use rmcp::{ServiceExt, transport::stdio};

use server::RublClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up panic hook to log to stderr (will appear in MCP client logs)
    std::panic::set_hook(Box::new(|panic_info| {
        logging::panic(panic_info);
    }));

    let api_key = std::env::var("EBIRD_API_KEY")
        .map_err(|_| {
            logging::error("EBIRD_API_KEY environment variable is required");
            "EBIRD_API_KEY environment variable is required"
        })?;

    logging::info("Starting MCP server");

    let service = RublClient::new(api_key).serve(stdio()).await.map_err(|e| {
        logging::error(format!("Failed to start MCP server: {}", e));
        e
    })?;

    service.waiting().await.map_err(|e| {
        logging::error(format!("MCP server error: {}", e));
        e
    })?;

    logging::info("MCP server shutting down");

    Ok(())
}
