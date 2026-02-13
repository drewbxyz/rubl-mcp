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

#[cfg(test)]
mod tests {
    use super::*;

    mod get_region_info {
        use super::*;

        #[test]
        fn path_includes_region_code() {
            let req = GetRegionInfoRequest {
                region_code: "US-NC".into(),
            };
            assert_eq!(req.path(), "ref/region/info/US-NC");
        }

        #[test]
        fn path_with_different_region() {
            let req = GetRegionInfoRequest {
                region_code: "US-CA".into(),
            };
            assert_eq!(req.path(), "ref/region/info/US-CA");
        }

        #[test]
        fn path_with_country_code() {
            let req = GetRegionInfoRequest {
                region_code: "US".into(),
            };
            assert_eq!(req.path(), "ref/region/info/US");
        }

        #[test]
        fn query_returns_empty_tuple() {
            let req = GetRegionInfoRequest {
                region_code: "US-NC".into(),
            };
            assert_eq!(req.query(), &());
        }
    }

    mod get_subregions {
        use super::*;

        #[test]
        fn path_includes_region_code() {
            let req = GetSubRegionsRequest {
                region_code: "US-NC".into(),
            };
            assert_eq!(req.path(), "ref/region/list/subnational2/US-NC");
        }

        #[test]
        fn path_with_different_region() {
            let req = GetSubRegionsRequest {
                region_code: "US-CA".into(),
            };
            assert_eq!(req.path(), "ref/region/list/subnational2/US-CA");
        }

        #[test]
        fn path_with_country_code() {
            let req = GetSubRegionsRequest {
                region_code: "US".into(),
            };
            assert_eq!(req.path(), "ref/region/list/subnational2/US");
        }

        #[test]
        fn query_returns_empty_tuple() {
            let req = GetSubRegionsRequest {
                region_code: "US-NC".into(),
            };
            assert_eq!(req.query(), &());
        }
    }

    mod region_type {
        use super::*;

        #[test]
        fn serializes_to_lowercase() {
            let country = RegionType::Country;
            let serialized = serde_json::to_string(&country).unwrap();
            assert_eq!(serialized, "\"country\"");

            let subnational1 = RegionType::Subnational1;
            let serialized = serde_json::to_string(&subnational1).unwrap();
            assert_eq!(serialized, "\"subnational1\"");

            let subnational2 = RegionType::Subnational2;
            let serialized = serde_json::to_string(&subnational2).unwrap();
            assert_eq!(serialized, "\"subnational2\"");
        }

        #[test]
        fn deserializes_from_lowercase() {
            let country: RegionType = serde_json::from_str("\"country\"").unwrap();
            assert_eq!(country, RegionType::Country);

            let subnational1: RegionType = serde_json::from_str("\"subnational1\"").unwrap();
            assert_eq!(subnational1, RegionType::Subnational1);

            let subnational2: RegionType = serde_json::from_str("\"subnational2\"").unwrap();
            assert_eq!(subnational2, RegionType::Subnational2);
        }
    }
}
