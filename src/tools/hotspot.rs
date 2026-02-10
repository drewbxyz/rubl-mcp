use serde::{Deserialize, Serialize};

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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetNearbyHotspotsRequest {
    #[schemars(description = "Latitude")]
    pub latitude: f32,
    #[schemars(description = "Longitude")]
    pub longitude: f32,
    #[schemars(description = "Radius in kilometers", range(min = 0, max = 500))]
    pub radius: Option<f32>,
    #[schemars(description = "Only fetch hotspots visited up to back days ago", range(min = 1, max = 30))]
    pub back: Option<u32>,
    
}