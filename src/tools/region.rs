use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]

pub enum RegionType {
    Country,
    Subnational1,
    Subnational2,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRegionInfoRequest {
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListSubRegionsRequest {
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    #[serde(rename = "minX")]
    pub min_x: f64,
    #[serde(rename = "maxX")]
    pub max_x: f64,
    #[serde(rename = "minY")]
    pub min_y: f64,
    #[serde(rename = "maxY")]
    pub max_y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionInfo {
    pub bounds: Option<Bounds>,
    pub result: String,
    pub code: String,
    #[serde(rename = "type")]
    pub region_type: RegionType,
    pub parent: Option<Box<RegionInfo>>,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubRegion {
    code: String,
    name: String,
}
