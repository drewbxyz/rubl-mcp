use reqwest::Client;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::tool::ToolRouter, model::*, tool,
    tool_handler, tool_router,
};

use crate::ebird_api;
use crate::tools::geo::RegionInfo;
use crate::tools::rare_birds::RareBird;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FetchRareRequest {
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRegionInfoRequest {
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
}

#[derive(Clone)]
pub struct EbirdClient {
    tool_router: ToolRouter<Self>,
    http: Client,
    api_key: String,
}

#[tool_router]
impl EbirdClient {
    pub fn new(api_key: String) -> Self {
        Self {
            tool_router: Self::tool_router(),
            api_key,
            http: Client::new(),
        }
    }

    #[tool(description = "Get region information from eBird")]
    async fn get_region_info(
        &self,
        Parameters(req): Parameters<GetRegionInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        let path = format!("/ref/region/info/{}", req.region_code);
        let region_info: Vec<RegionInfo> = ebird_api::get(&self.http, &path, &self.api_key, &[])
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::json(region_info)?]))
    }

    #[tool(description = "Fetch rare birds from eBird for a region")]
    async fn fetch_rare(
        &self,
        Parameters(req): Parameters<FetchRareRequest>,
    ) -> Result<CallToolResult, McpError> {
        let path = format!("/data/obs/{}/recent/notable", req.region);
        let birds: Vec<RareBird> = ebird_api::get(&self.http, &path, &self.api_key, &[])
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::json(birds)?]))
    }
}

#[tool_handler]
impl ServerHandler for EbirdClient {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Utilities for calling the eBird API".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
