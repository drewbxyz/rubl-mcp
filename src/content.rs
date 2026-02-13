use rmcp::model::Content;
use toon::encode as toon_encode;

pub trait ToContent {
    fn to_content(&self) -> Result<Content, serde_json::Error>;
}

impl<T: serde::Serialize> ToContent for T {
    fn to_content(&self) -> Result<Content, serde_json::Error> {
        let json = serde_json::to_value(self)?;
        Ok(Content::text(toon_encode(&json, None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::hotspot::Hotspot;
    use crate::tools::observations::Observation;
    use crate::tools::region::{Bounds, RegionInfo, RegionType};

    mod observation_to_content {
        use super::*;

        #[test]
        fn formats_with_all_fields_present() {
            let obs = Observation {
                common_name: "Barn Swallow".into(),
                scientific_name: "Hirundo rustica".into(),
                location: "My Backyard".into(),
                observation_date: "2023-05-15 10:30".into(),
                count: Some(3),
                location_id: Some("L123456".into()),
                latitude: Some(35.9132),
                longitude: Some(-79.0558),
            };

            // First verify the JSON serialization works
            let json = serde_json::to_value(&obs).unwrap();
            assert_eq!(json["comName"], "Barn Swallow");
            assert_eq!(json["sciName"], "Hirundo rustica");
            assert_eq!(json["locName"], "My Backyard");
            assert_eq!(json["obsDt"], "2023-05-15 10:30");
            assert_eq!(json["howMany"], 3);
            assert_eq!(json["locId"], "L123456");

            // Then verify content conversion succeeds
            let content = obs.to_content();
            assert!(content.is_ok());
        }

        #[test]
        fn handles_missing_optional_fields() {
            let obs = Observation {
                common_name: "Barn Swallow".into(),
                scientific_name: "Hirundo rustica".into(),
                location: "My Backyard".into(),
                observation_date: "2023-05-15 10:30".into(),
                count: None,
                location_id: None,
                latitude: None,
                longitude: None,
            };

            // Verify JSON serialization with null optional fields
            let json = serde_json::to_value(&obs).unwrap();
            assert!(json["howMany"].is_null());
            assert!(json["locId"].is_null());
            assert!(json["lat"].is_null());
            assert!(json["lng"].is_null());

            // Should succeed even with missing optional fields
            let content = obs.to_content();
            assert!(content.is_ok());
        }

        #[test]
        fn handles_vec_of_observations() {
            let observations = vec![
                Observation {
                    common_name: "Barn Swallow".into(),
                    scientific_name: "Hirundo rustica".into(),
                    location: "Location 1".into(),
                    observation_date: "2023-05-15 10:30".into(),
                    count: Some(3),
                    location_id: None,
                    latitude: None,
                    longitude: None,
                },
                Observation {
                    common_name: "Eastern Bluebird".into(),
                    scientific_name: "Sialia sialis".into(),
                    location: "Location 2".into(),
                    observation_date: "2023-05-15 11:00".into(),
                    count: Some(1),
                    location_id: None,
                    latitude: None,
                    longitude: None,
                },
            ];

            // Verify JSON array serialization
            let json = serde_json::to_value(&observations).unwrap();
            assert!(json.is_array());
            assert_eq!(json.as_array().unwrap().len(), 2);
            assert_eq!(json[0]["comName"], "Barn Swallow");
            assert_eq!(json[1]["comName"], "Eastern Bluebird");

            // Verify content is created successfully for Vec of observations
            let content = observations.to_content();
            assert!(content.is_ok());
        }
    }

    mod hotspot_to_content {
        use super::*;

        #[test]
        fn formats_with_all_activity_stats() {
            let hotspot = Hotspot {
                location_id: "L123456".into(),
                name: "Test Hotspot".into(),
                latitude: 35.9132,
                longitude: -79.0558,
                country_code: "US".into(),
                country_name: Some("United States".into()),
                subnational1_code: "US-NC".into(),
                subnational1_name: Some("North Carolina".into()),
                subnational2_code: Some("US-NC-067".into()),
                subnational2_name: Some("Durham".into()),
                is_hotspot: Some(true),
                hierarchical_name: Some("Test Hotspot, Durham, NC, US".into()),
                latest_observation_date: Some("2023-05-15".into()),
                num_species_all_time: Some(150),
                num_checklists_all_time: Some(500),
            };

            // Verify JSON serialization with all fields
            let json = serde_json::to_value(&hotspot).unwrap();
            assert_eq!(json["locId"], "L123456");
            assert_eq!(json["locName"], "Test Hotspot");
            assert_eq!(json["lat"], 35.9132);
            assert_eq!(json["lng"], -79.0558);
            assert_eq!(json["numSpeciesAllTime"], 150);
            assert_eq!(json["numChecklistsAllTime"], 500);

            // Verify content is created successfully for hotspot with all fields
            let content = hotspot.to_content();
            assert!(content.is_ok());
        }

        #[test]
        fn handles_missing_optional_fields() {
            let hotspot = Hotspot {
                location_id: "L123456".into(),
                name: "Test Hotspot".into(),
                latitude: 35.9132,
                longitude: -79.0558,
                country_code: "US".into(),
                country_name: None,
                subnational1_code: "US-NC".into(),
                subnational1_name: None,
                subnational2_code: None,
                subnational2_name: None,
                is_hotspot: None,
                hierarchical_name: None,
                latest_observation_date: None,
                num_species_all_time: None,
                num_checklists_all_time: None,
            };

            // Verify JSON serialization with null optional fields
            let json = serde_json::to_value(&hotspot).unwrap();
            assert_eq!(json["locId"], "L123456");
            assert_eq!(json["locName"], "Test Hotspot");
            assert!(json["countryName"].is_null());
            assert!(json["numSpeciesAllTime"].is_null());
            assert!(json["numChecklistsAllTime"].is_null());

            let content = hotspot.to_content();
            assert!(content.is_ok());
        }
    }

    mod region_to_content {
        use super::*;

        #[test]
        fn formats_with_bounds() {
            let region = RegionInfo {
                bounds: Some(Bounds {
                    min_x: -79.5,
                    max_x: -78.5,
                    min_y: 35.5,
                    max_y: 36.5,
                }),
                result: "North Carolina".into(),
                code: "US-NC".into(),
                region_type: RegionType::Subnational1,
                parent: None,
                longitude: -79.0,
                latitude: 36.0,
            };

            // Verify JSON serialization with bounds
            let json = serde_json::to_value(&region).unwrap();
            assert_eq!(json["result"], "North Carolina");
            assert_eq!(json["code"], "US-NC");
            assert_eq!(json["type"], "subnational1");
            assert_eq!(json["bounds"]["minX"], -79.5);
            assert_eq!(json["bounds"]["maxX"], -78.5);
            assert_eq!(json["longitude"], -79.0);
            assert_eq!(json["latitude"], 36.0);

            // Verify content is created successfully for region with bounds
            let content = region.to_content();
            assert!(content.is_ok());
        }

        #[test]
        fn handles_region_without_bounds() {
            let region = RegionInfo {
                bounds: None,
                result: "United States".into(),
                code: "US".into(),
                region_type: RegionType::Country,
                parent: None,
                longitude: -98.0,
                latitude: 39.0,
            };

            // Verify JSON serialization without bounds
            let json = serde_json::to_value(&region).unwrap();
            assert_eq!(json["result"], "United States");
            assert_eq!(json["code"], "US");
            assert_eq!(json["type"], "country");
            assert!(json["bounds"].is_null());

            let content = region.to_content();
            assert!(content.is_ok());
        }

        #[test]
        fn handles_region_with_parent() {
            let region = RegionInfo {
                bounds: None,
                result: "North Carolina".into(),
                code: "US-NC".into(),
                region_type: RegionType::Subnational1,
                parent: Some(Box::new(RegionInfo {
                    bounds: None,
                    result: "United States".into(),
                    code: "US".into(),
                    region_type: RegionType::Country,
                    parent: None,
                    longitude: -98.0,
                    latitude: 39.0,
                })),
                longitude: -79.0,
                latitude: 36.0,
            };

            // Verify JSON serialization with parent
            let json = serde_json::to_value(&region).unwrap();
            assert_eq!(json["result"], "North Carolina");
            assert_eq!(json["parent"]["code"], "US");
            assert_eq!(json["parent"]["result"], "United States");
            assert_eq!(json["parent"]["type"], "country");

            let content = region.to_content();
            assert!(content.is_ok());
        }
    }
}
