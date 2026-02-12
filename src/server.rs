use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, tool, tool_handler, tool_router,
};

use crate::{
    api::client::ApiClient,
    content::ToContent,
    tools::hotspot::{
        FetchHotspotInfoRequest, FetchNearbyHotspotsRequest, FetchRegionHotspotsRequest,
    },
    tools::observations::{
        FetchGeoRecentRequest, FetchHotspotRecentRequest, FetchNotableRecentRequest,
        FetchRegionRecentRequest,
    },
};

#[derive(Clone)]
pub struct RublClient {
    tool_router: ToolRouter<Self>,
    client: ApiClient,
}

#[tool_router]
impl RublClient {
    pub fn new(api_key: String) -> Self {
        Self {
            tool_router: Self::tool_router(),
            client: ApiClient::new(api_key),
        }
    }

    #[tool(
        description = "Fetch recently reported notable/rare bird sightings for an eBird region. Returns species, location, date, and count. Use for rarity alerts or recent notable observations.",
        annotations(title = "Rare birds", read_only_hint = true)
    )]
    async fn fetch_notable_recent(
        &self,
        Parameters(req): Parameters<FetchNotableRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    #[tool(
        description = "Fetch recently reported bird sightings at a specific eBird hotspot. Returns species, location, date, and count. Use for recent hotspot activity or spotting trends.",
        annotations(title = "Hotspot activity", read_only_hint = true)
    )]
    async fn fetch_hotspot_recent(
        &self,
        Parameters(req): Parameters<FetchHotspotRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    #[tool(
        description = "Fetch recently reported bird sightings for an eBird region. Returns species, location, date, and count. Use for recent region activity or spotting trends.",
        annotations(title = "Region activity", read_only_hint = true)
    )]
    async fn fetch_region_recent(
        &self,
        Parameters(req): Parameters<FetchRegionRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    #[tool(
        description = "Fetch recently reported bird sightings by geographic coordinates. Returns species, location, date, and count. Use for recent observations or spotting trends near a specific location.",
        annotations(title = "Geographic observations", read_only_hint = true)
    )]
    async fn fetch_geo_recent(
        &self,
        Parameters(req): Parameters<FetchGeoRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    #[tool(
        description = "Fetch birding hotspots for an eBird region. Returns hotspot names, coordinates, and recent activity statistics. Use for finding birding locations or exploring birding areas.",
        annotations(title = "Region hotspots", read_only_hint = true)
    )]
    async fn fetch_region_hotspots(
        &self,
        Parameters(req): Parameters<FetchRegionHotspotsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    #[tool(
        description = "Fetch nearby birding hotspots by geographic coordinates. Returns hotspot names, coordinates, and recent activity statistics. Use for finding nearby birding locations or exploring birding areas.",
        annotations(title = "Nearby hotspots", read_only_hint = true)
    )]
    async fn fetch_nearby_hotspots(
        &self,
        Parameters(req): Parameters<FetchNearbyHotspotsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    #[tool(
        description = "Fetch information about a specific eBird hotspot by location ID. Returns hotspot name, coordinates, and recent activity statistics. Use for detailed hotspot information or spotting trends.",
        annotations(title = "Hotspot info", read_only_hint = true)
    )]
    async fn fetch_hotspot_info(
        &self,
        Parameters(req): Parameters<FetchHotspotInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .to_content()
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![response]))
    }

    // #[tool(
    //     description = "Look up eBird region metadata (name, type, bounds, parent). Use when you need to resolve or validate a region code (e.g. US-NC) or get geographic bounds.",
    //     annotations(title = "Region info", read_only_hint = true)
    // )]
    // async fn fetch_region_information(&self) {}

    // #[tool(
    //     description = "List subregions (states, counties, etc.) under an eBird region. Use to drill down from country to state to county or to enumerate areas within a region.",
    //     annotations(title = "Subregions", read_only_hint = true)
    // )]
    // async fn list_subregions(&self) {}
}

#[tool_handler]
impl ServerHandler for RublClient {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "eBird API tools: region hierarchy (get_region_info, list_subregions), notable sightings (fetch_rare), and nearby birding locations (get_nearby_hotspots). \
                 Region codes are like US, US-NC, US-NC-067. All tools are read-only."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
