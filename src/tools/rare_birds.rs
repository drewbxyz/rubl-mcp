use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RareBird {
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
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FetchRareRequest {
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
}
