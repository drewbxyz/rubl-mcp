use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

// Common observation response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
    #[serde(rename = "comName")]
    pub common_name: String,
    #[serde(rename = "sciName")]
    pub scientific_name: String,
    #[serde(rename = "locName")]
    pub location: String,
    #[serde(rename = "obsDt")]
    pub observation_date: String,
    #[serde(rename = "howMany", default)]
    pub count: Option<u32>,
    #[serde(rename = "locId", default)]
    pub location_id: Option<String>,
    #[serde(rename = "lat", default)]
    pub latitude: Option<f64>,
    #[serde(rename = "lng", default)]
    pub longitude: Option<f64>,
}

// Alias for notable/rare birds (same structure as Observation)
pub type RareBird = Observation;

// Request: Fetch recent observations for a region
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchRegionRecentRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
    #[schemars(
        description = "Number of days back to fetch observations",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchRegionRecentRequest {
    type Query = FetchRegionRecentRequest;
    type Response = Vec<Observation>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("data/obs/{}/recent", self.region_code)
    }

    fn query(&self) -> &Self::Query {
        self
    }
}

// Request: Fetch recent observations by geographic coordinates
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchGeoRecentRequest {
    #[schemars(description = "Latitude")]
    pub lat: f32,
    #[schemars(description = "Longitude")]
    pub lng: f32,
    #[schemars(
        description = "Number of days back to fetch observations",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchGeoRecentRequest {
    type Query = FetchGeoRecentRequest;
    type Response = Vec<Observation>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        "data/obs/geo/recent".into()
    }

    fn query(&self) -> &Self::Query {
        self
    }
}

// Request: Fetch notable/rare bird observations for a region
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchNotableRecentRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
    #[schemars(
        description = "Number of days back to fetch notable observations",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchNotableRecentRequest {
    type Query = FetchNotableRecentRequest;
    type Response = Vec<RareBird>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("data/obs/{}/recent/notable", self.region_code)
    }

    fn query(&self) -> &Self::Query {
        self
    }
}

// Request: Fetch recent observations at a specific hotspot
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchHotspotRecentRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird hotspot location ID (e.g., L123456)")]
    pub loc_id: String,
    #[schemars(
        description = "Number of days back to fetch observations",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchHotspotRecentRequest {
    type Query = FetchHotspotRecentRequest;
    type Response = Vec<Observation>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("data/obs/hotspot/{}/recent", self.loc_id)
    }

    fn query(&self) -> &Self::Query {
        self
    }
}
