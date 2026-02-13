use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::endpoint::Endpoint;

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

// Type alias for notable/rare birds (same structure, semantically distinct).
pub type RareBird = Observation;

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

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchGeoRecentRequest {
    #[schemars(description = "Latitude")]
    pub lat: f64,
    #[schemars(description = "Longitude")]
    pub lng: f64,
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

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchSpeciesRecentRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
    #[serde(skip_serializing)]
    #[schemars(description = "Species code (e.g., barswa for Barn Swallow)")]
    pub species_code: String,
    #[schemars(
        description = "Number of days back to fetch observations",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchSpeciesRecentRequest {
    type Query = FetchSpeciesRecentRequest;
    type Response = Vec<Observation>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("data/obs/{}/recent/{}", self.region_code, self.species_code)
    }

    fn query(&self) -> &Self::Query {
        self
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchSpeciesNearestRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "Species code (e.g., barswa for Barn Swallow)")]
    pub species_code: String,
    #[schemars(description = "Latitude")]
    pub lat: f64,
    #[schemars(description = "Longitude")]
    pub lng: f64,
    #[schemars(
        description = "Search radius in kilometers (max 50)",
        range(min = 1, max = 50)
    )]
    pub dist: Option<u32>,
    #[schemars(
        description = "Number of days back to search",
        range(min = 1, max = 30)
    )]
    pub back: Option<u32>,
}

impl Endpoint for FetchSpeciesNearestRequest {
    type Query = FetchSpeciesNearestRequest;
    type Response = Vec<Observation>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!("data/nearest/geo/recent/{}", self.species_code)
    }

    fn query(&self) -> &Self::Query {
        self
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FetchHistoricRequest {
    #[serde(skip_serializing)]
    #[schemars(description = "eBird region code (e.g., US-NC)")]
    pub region_code: String,
    #[serde(skip_serializing)]
    #[schemars(description = "Year (e.g., 2023)")]
    pub year: u32,
    #[serde(skip_serializing)]
    #[schemars(description = "Month (1-12)")]
    pub month: u32,
    #[serde(skip_serializing)]
    #[schemars(description = "Day (1-31)")]
    pub day: u32,
}

impl Endpoint for FetchHistoricRequest {
    type Query = ();
    type Response = Vec<Observation>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> String {
        format!(
            "data/obs/{}/historic/{}/{}/{}",
            self.region_code, self.year, self.month, self.day
        )
    }

    fn query(&self) -> &Self::Query {
        &()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fetch_region_recent {
        use super::*;

        #[test]
        fn path_includes_region_code() {
            let req = FetchRegionRecentRequest {
                region_code: "US-NC".into(),
                back: None,
            };
            assert_eq!(req.path(), "data/obs/US-NC/recent");
        }

        #[test]
        fn path_with_different_region_code() {
            let req = FetchRegionRecentRequest {
                region_code: "US-CA".into(),
                back: None,
            };
            assert_eq!(req.path(), "data/obs/US-CA/recent");
        }

        #[test]
        fn query_includes_back_parameter_when_present() {
            let req = FetchRegionRecentRequest {
                region_code: "US-NC".into(),
                back: Some(7),
            };
            // Verify the query can be serialized and contains back parameter
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["back"], 7);
        }

        #[test]
        fn query_omits_back_parameter_when_none() {
            let req = FetchRegionRecentRequest {
                region_code: "US-NC".into(),
                back: None,
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert!(serialized["back"].is_null());
        }
    }

    mod fetch_geo_recent {
        use super::*;

        #[test]
        fn path_is_constant() {
            let req = FetchGeoRecentRequest {
                lat: 35.9132,
                lng: -79.0558,
                back: None,
            };
            assert_eq!(req.path(), "data/obs/geo/recent");
        }

        #[test]
        fn query_includes_coordinates() {
            let req = FetchGeoRecentRequest {
                lat: 35.9132,
                lng: -79.0558,
                back: None,
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["lat"], 35.9132);
            assert_eq!(serialized["lng"], -79.0558);
        }

        #[test]
        fn query_includes_back_parameter_when_present() {
            let req = FetchGeoRecentRequest {
                lat: 35.9132,
                lng: -79.0558,
                back: Some(14),
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["back"], 14);
        }
    }

    mod fetch_notable_recent {
        use super::*;

        #[test]
        fn path_includes_region_code_and_notable() {
            let req = FetchNotableRecentRequest {
                region_code: "US-NC".into(),
                back: None,
            };
            assert_eq!(req.path(), "data/obs/US-NC/recent/notable");
        }

        #[test]
        fn query_includes_back_parameter() {
            let req = FetchNotableRecentRequest {
                region_code: "US-NC".into(),
                back: Some(3),
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["back"], 3);
        }
    }

    mod fetch_species_recent {
        use super::*;

        #[test]
        fn path_includes_region_and_species() {
            let req = FetchSpeciesRecentRequest {
                region_code: "US-NC".into(),
                species_code: "barswa".into(),
                back: None,
            };
            assert_eq!(req.path(), "data/obs/US-NC/recent/barswa");
        }

        #[test]
        fn path_with_different_species() {
            let req = FetchSpeciesRecentRequest {
                region_code: "US-CA".into(),
                species_code: "caltow".into(),
                back: Some(7),
            };
            assert_eq!(req.path(), "data/obs/US-CA/recent/caltow");
        }

        #[test]
        fn query_serializes_back_parameter() {
            let req = FetchSpeciesRecentRequest {
                region_code: "US-NC".into(),
                species_code: "barswa".into(),
                back: Some(10),
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["back"], 10);
        }
    }

    mod fetch_species_nearest {
        use super::*;

        #[test]
        fn path_includes_species_code() {
            let req = FetchSpeciesNearestRequest {
                species_code: "barswa".into(),
                lat: 35.9132,
                lng: -79.0558,
                dist: None,
                back: None,
            };
            assert_eq!(req.path(), "data/nearest/geo/recent/barswa");
        }

        #[test]
        fn query_includes_coordinates() {
            let req = FetchSpeciesNearestRequest {
                species_code: "barswa".into(),
                lat: 35.9132,
                lng: -79.0558,
                dist: None,
                back: None,
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["lat"], 35.9132);
            assert_eq!(serialized["lng"], -79.0558);
        }

        #[test]
        fn query_includes_optional_parameters() {
            let req = FetchSpeciesNearestRequest {
                species_code: "barswa".into(),
                lat: 35.9132,
                lng: -79.0558,
                dist: Some(25),
                back: Some(7),
            };
            let serialized = serde_json::to_value(req.query()).unwrap();
            assert_eq!(serialized["dist"], 25);
            assert_eq!(serialized["back"], 7);
        }
    }

    mod fetch_historic {
        use super::*;

        #[test]
        fn path_formats_date_correctly() {
            let req = FetchHistoricRequest {
                region_code: "US-NC".into(),
                year: 2023,
                month: 1,
                day: 15,
            };
            assert_eq!(req.path(), "data/obs/US-NC/historic/2023/1/15");
        }

        #[test]
        fn path_formats_single_digit_date() {
            let req = FetchHistoricRequest {
                region_code: "US-NC".into(),
                year: 2023,
                month: 5,
                day: 3,
            };
            assert_eq!(req.path(), "data/obs/US-NC/historic/2023/5/3");
        }

        #[test]
        fn path_formats_double_digit_date() {
            let req = FetchHistoricRequest {
                region_code: "US-CA".into(),
                year: 2022,
                month: 12,
                day: 31,
            };
            assert_eq!(req.path(), "data/obs/US-CA/historic/2022/12/31");
        }

        #[test]
        fn query_returns_empty_tuple() {
            let req = FetchHistoricRequest {
                region_code: "US-NC".into(),
                year: 2023,
                month: 1,
                day: 15,
            };
            // Ensure no query params are serialized (should be unit type)
            assert_eq!(req.query(), &());
        }
    }
}
