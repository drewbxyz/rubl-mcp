use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::prompt::PromptRouter,
        tool::ToolRouter,
        wrapper::Parameters,
    },
    model::*, prompt, prompt_handler, prompt_router,
    service::RequestContext, tool, tool_handler, tool_router,
};

use serde_json::Value;

use crate::{
    api::client::ApiClient,
    content::ToContent,
    tools::hotspot::{
        FetchHotspotInfoRequest, FetchNearbyHotspotsRequest, FetchRegionHotspotsRequest,
    },
    tools::observations::{
        FetchGeoRecentRequest, FetchHistoricRequest, FetchHotspotRecentRequest,
        FetchNotableRecentRequest, FetchRegionRecentRequest, FetchSpeciesNearestRequest,
        FetchSpeciesRecentRequest,
    },
    tools::region::{GetRegionInfoRequest, GetSubRegionsRequest},
};

#[derive(Clone)]
pub struct RublClient {
    tool_router: ToolRouter<Self>,
    prompt_router: PromptRouter<Self>,
    client: ApiClient,
}

#[tool_router]
impl RublClient {
    pub fn new(api_key: String) -> Self {
        Self {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
            client: ApiClient::new(api_key),
        }
    }

    /// Helper method to handle common request/response pattern
    async fn handle_request<E>(&self, req: E) -> Result<CallToolResult, McpError>
    where
        E: crate::api::endpoint::Endpoint,
        E::Response: ToContent,
    {
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
        description = "Fetch recently reported notable/rare bird sightings for an eBird region. Returns species, location, date, and count. Use for rarity alerts or recent notable observations.",
        annotations(title = "Rare birds", read_only_hint = true)
    )]
    async fn fetch_notable_recent(
        &self,
        Parameters(req): Parameters<FetchNotableRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch recently reported bird sightings at a specific eBird hotspot. Returns species, location, date, and count. Use for recent hotspot activity or spotting trends.",
        annotations(title = "Hotspot activity", read_only_hint = true)
    )]
    async fn fetch_hotspot_recent(
        &self,
        Parameters(req): Parameters<FetchHotspotRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch recently reported bird sightings for an eBird region. Returns species, location, date, and count. Use for recent region activity or spotting trends.",
        annotations(title = "Region activity", read_only_hint = true)
    )]
    async fn fetch_region_recent(
        &self,
        Parameters(req): Parameters<FetchRegionRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch recently reported bird sightings by geographic coordinates. Returns species, location, date, and count. Use for recent observations or spotting trends near a specific location.",
        annotations(title = "Geographic observations", read_only_hint = true)
    )]
    async fn fetch_geo_recent(
        &self,
        Parameters(req): Parameters<FetchGeoRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch birding hotspots for an eBird region. Returns hotspot names, coordinates, and recent activity statistics. Use for finding birding locations or exploring birding areas.",
        annotations(title = "Region hotspots", read_only_hint = true)
    )]
    async fn fetch_region_hotspots(
        &self,
        Parameters(req): Parameters<FetchRegionHotspotsRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch nearby birding hotspots by geographic coordinates. Returns hotspot names, coordinates, and recent activity statistics. Use for finding nearby birding locations or exploring birding areas.",
        annotations(title = "Nearby hotspots", read_only_hint = true)
    )]
    async fn fetch_nearby_hotspots(
        &self,
        Parameters(req): Parameters<FetchNearbyHotspotsRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch information about a specific eBird hotspot by location ID. Returns hotspot name, coordinates, and recent activity statistics. Use for detailed hotspot information or spotting trends.",
        annotations(title = "Hotspot info", read_only_hint = true)
    )]
    async fn fetch_hotspot_info(
        &self,
        Parameters(req): Parameters<FetchHotspotInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Look up eBird region metadata (name, type, bounds, parent). Use when you need to resolve or validate a region code (e.g. US-NC) or get geographic bounds.",
        annotations(title = "Region info", read_only_hint = true)
    )]
    async fn get_region_info(
        &self,
        Parameters(req): Parameters<GetRegionInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "List subregions (states, counties, etc.) under an eBird region. Use to drill down from country to state to county or to enumerate areas within a region.",
        annotations(title = "Subregions", read_only_hint = true)
    )]
    async fn get_subregions(
        &self,
        Parameters(req): Parameters<GetSubRegionsRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch recent observations of a specific species in a region. Returns observations with location, date, and count. Use for tracking a specific bird species or finding recent sightings.",
        annotations(title = "Species observations", read_only_hint = true)
    )]
    async fn fetch_species_recent(
        &self,
        Parameters(req): Parameters<FetchSpeciesRecentRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Find nearest recent observations of a specific species by geographic coordinates. Returns observations with location, date, count, and distance. Use for finding where a species was recently seen nearby.",
        annotations(title = "Nearest species", read_only_hint = true)
    )]
    async fn fetch_species_nearest(
        &self,
        Parameters(req): Parameters<FetchSpeciesNearestRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }

    #[tool(
        description = "Fetch historic observations on a specific date in a region. Returns observations from exactly that date. Use for exploring what was seen on a particular day in the past.",
        annotations(title = "Historic observations", read_only_hint = true)
    )]
    async fn fetch_historic(
        &self,
        Parameters(req): Parameters<FetchHistoricRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_request(req).await
    }
}

#[prompt_router]
impl RublClient {
    #[prompt(
        title = "Plan Birding Day",
        description = "Create an optimized birding itinerary for a day trip. Returns a structured hour-by-hour schedule with locations, target species, and tips based on recent sightings and hotspot data."
    )]
    async fn plan_birding_day(
        &self,
        Parameters(arguments): Parameters<Option<serde_json::Map<String, Value>>>,
    ) -> Result<GetPromptResult, McpError> {
        crate::prompts::trip::plan_birding_day(&self.client, &arguments.unwrap_or_default())
            .await
            .map_err(|e| McpError::internal_error(e, None))
    }

    #[prompt(
        title = "Find Species",
        description = "Find the best locations and times to see a specific bird species. Returns recommendations based on recent sightings, habitat preferences, and identification tips."
    )]
    async fn find_species(
        &self,
        Parameters(arguments): Parameters<Option<serde_json::Map<String, Value>>>,
    ) -> Result<GetPromptResult, McpError> {
        crate::prompts::trip::find_species(&self.client, &arguments.unwrap_or_default())
            .await
            .map_err(|e| McpError::internal_error(e, None))
    }
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for RublClient {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "eBird API tools: region hierarchy (get_region_info, list_subregions), notable sightings (fetch_rare), and nearby birding locations (get_nearby_hotspots). \
                 Region codes are like US, US-NC, US-NC-067. All tools are read-only."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
            ..Default::default()
        }
    }
}
