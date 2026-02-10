use reqwest::Client;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::tool::ToolRouter, model::*, tool,
    tool_handler, tool_router,
};
use serde::de::DeserializeOwned;
use toon::encode as toon_encode;

use crate::ebird_api;
use crate::tools::hotspot::{GetNearbyHotspotsRequest, Hotspot};
use crate::tools::rare_birds::{FetchRareRequest, RareBird};
use crate::tools::region::{GetRegionInfoRequest, GetSubRegionsRequest, RegionInfo, SubRegion};

fn toon_content<T: serde::Serialize>(value: &T) -> Result<Content, McpError> {
    let json =
        serde_json::to_value(value).map_err(|e| McpError::internal_error(e.to_string(), None))?;
    Ok(Content::text(toon_encode(&json, None)))
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

    async fn fetch_from_ebird<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T, McpError> {
        ebird_api::get(&self.http, path, &self.api_key, params)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))
    }

    fn toon_result<T: serde::Serialize>(value: &T) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![toon_content(value)?]))
    }

    #[tool(
        description = "Look up eBird region metadata (name, type, bounds, parent). Use when you need to resolve or validate a region code (e.g. US-NC) or get geographic bounds.",
        annotations(title = "Region info", read_only_hint = true)
    )]
    async fn get_region_info(
        &self,
        Parameters(req): Parameters<GetRegionInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        let path = format!("/ref/region/info/{}", req.region_code);
        let region_info: RegionInfo = self.fetch_from_ebird(&path, &[]).await?;
        Self::toon_result(&region_info)
    }

    #[tool(
        description = "List subregions (states, counties, etc.) under an eBird region. Use to drill down from country to state to county or to enumerate areas within a region.",
        annotations(title = "Subregions", read_only_hint = true)
    )]
    async fn get_subregions(
        &self,
        Parameters(req): Parameters<GetSubRegionsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let path = format!("/ref/region/subnational/{}", req.region_code);
        let subregions: Vec<SubRegion> = self.fetch_from_ebird(&path, &[]).await?;
        Self::toon_result(&subregions)
    }

    #[tool(
        description = "Fetch recently reported notable/rare bird sightings for an eBird region. Returns species, location, date, and count. Use for rarity alerts or recent notable observations.",
        annotations(title = "Rare birds", read_only_hint = true)
    )]
    async fn fetch_rare(
        &self,
        Parameters(req): Parameters<FetchRareRequest>,
    ) -> Result<CallToolResult, McpError> {
        let path = format!("/data/obs/{}/recent/notable", req.region_code);
        let birds: Vec<RareBird> = self.fetch_from_ebird(&path, &[]).await?;
        Self::toon_result(&birds)
    }

    #[tool(
        description = "Find eBird hotspots (birding locations) by place name or coordinates. Use for queries like 'Where should I go birding in [location]?', 'Best birding spots near [place]', 'Top birding locations in [area]', or 'Where to bird in [region]'. Accepts latitude/longitude coordinates - for location names, geocode first (use web search or other tools to get coordinates). Optional radius (km) and back (days, 1-30) to filter results. Returns list of eBird hotspots with names, coordinates, and recent activity.",
        annotations(title = "Nearby hotspots", read_only_hint = true)
    )]
    async fn get_nearby_hotspots(
        &self,
        Parameters(req): Parameters<GetNearbyHotspotsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut params = vec![
            ("lat".to_string(), req.latitude.to_string()),
            ("lng".to_string(), req.longitude.to_string()),
            ("fmt".to_string(), "json".to_string()),
        ];
        if let Some(radius) = req.radius {
            params.push(("dist".to_string(), radius.to_string()));
        }
        if let Some(back) = req.back {
            params.push(("back".to_string(), back.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        let hotspots: Vec<Hotspot> = self
            .fetch_from_ebird("/ref/hotspot/geo", &param_refs)
            .await?;
        Self::toon_result(&hotspots)
    }
}

#[tool_handler]
impl ServerHandler for EbirdClient {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "eBird API tools: region hierarchy (get_region_info, get_subregions), notable sightings (fetch_rare), and nearby birding locations (get_nearby_hotspots). \
                 Region codes are like US, US-NC, US-NC-067. All tools are read-only."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
