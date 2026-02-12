use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegionType {
    Country,
    Subnational1,
    Subnational2,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRegionInfoRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
}

impl Endpoint for GetRegionInfoRequest {
    type Query = ();
    type Response = RegionInfo;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("ref/region/info/{}", self.region_code)
    }

    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetSubRegionsRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
}

impl Endpoint for GetSubRegionsRequest {
    type Query = ();
    type Response = Vec<SubRegion>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("ref/region/list/subnational2/{}", self.region_code)
    }

    fn query(&self) -> &Self::Query {
        &()
    }
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
    pub code: String,
    pub name: String,
}
