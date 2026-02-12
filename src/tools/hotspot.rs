use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hotspot {
    #[serde(rename = "locId")]
    pub location_id: String,
    #[serde(rename = "locName")]
    pub name: String,
    #[serde(rename = "lat")]
    pub latitude: f64,
    #[serde(rename = "lng")]
    pub longitude: f64,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "countryName")]
    pub country_name: Option<String>,
    #[serde(rename = "subnational1Code")]
    pub subnational1_code: String,
    #[serde(rename = "subnational1Name")]
    pub subnational1_name: Option<String>,
    #[serde(rename = "subnational2Code")]
    pub subnational2_code: Option<String>,
    #[serde(rename = "subnational2Name")]
    pub subnational2_name: Option<String>,
    #[serde(rename = "isHotspot")]
    pub is_hotspot: Option<bool>,
    #[serde(rename = "hierarchicalName")]
    pub hierarchical_name: Option<String>,
    #[serde(rename = "latestObsDt")]
    pub latest_observation_date: Option<String>,
    #[serde(rename = "numSpeciesAllTime")]
    pub num_species_all_time: Option<u32>,
    #[serde(rename = "numChecklistsAllTime")]
    pub num_checklists_all_time: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchRegionHotspotsRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
    #[schemars(
        description = "Only fetch hotspots visited up to back days ago",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchRegionHotspotsRequest {
    type Query = FetchRegionHotspotsRequest;
    type Response = Vec<Hotspot>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("ref/hotspot/{}", self.region_code)
    }

    fn query(&self) -> &Self::Query {
        self
    }

    fn format(&self) -> Option<&'static str> {
        Some("json")
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchNearbyHotspotsRequest {
    #[schemars(description = "Latitude")]
    pub lat: f64,
    #[schemars(description = "Longitude")]
    pub lng: f64,
    #[schemars(description = "Radius in kilometers", range(min = 0, max = 500))]
    pub dist: Option<f64>,
    #[schemars(
        description = "Only fetch hotspots visited up to back days ago",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchNearbyHotspotsRequest {
    type Query = FetchNearbyHotspotsRequest;
    type Response = Vec<Hotspot>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        "ref/hotspot/geo".into()
    }

    fn query(&self) -> &Self::Query {
        self
    }

    fn format(&self) -> Option<&'static str> {
        Some("json")
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchHotspotInfoRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird hotspot location ID (e.g., L123456)")]
    pub loc_id: String,
}

impl Endpoint for FetchHotspotInfoRequest {
    type Query = FetchHotspotInfoRequest;
    type Response = Hotspot;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("ref/hotspot/info/{}", self.loc_id)
    }

    fn query(&self) -> &Self::Query {
        self
    }

    fn format(&self) -> Option<&'static str> {
        Some("json")
    }
}
