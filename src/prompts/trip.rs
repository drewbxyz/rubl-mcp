use rmcp::model::{GetPromptResult, PromptMessage, PromptMessageRole};
use serde_json::{Map, Value};

use crate::{
    api::client::ApiClient,
    tools::{
        hotspot::FetchRegionHotspotsRequest,
        observations::{FetchNotableRecentRequest, FetchSpeciesRecentRequest},
        region::GetRegionInfoRequest,
    },
};

pub async fn plan_birding_day(
    client: &ApiClient,
    args: &Map<String, Value>,
) -> Result<GetPromptResult, String> {
    // Extract arguments
    let region = args
        .get("region")
        .and_then(|v| v.as_str())
        .ok_or("Missing required argument: region")?;

    let duration_hours = args
        .get("duration_hours")
        .and_then(|v| v.as_u64())
        .unwrap_or(6) as u32;

    let target_species = args.get("target_species").and_then(|v| v.as_str());

    // Pre-fetch data
    let region_info = client
        .send(GetRegionInfoRequest {
            region_code: region.to_string(),
        })
        .await
        .map_err(|e| format!("Failed to fetch region info: {}", e))?;

    let notable = client
        .send(FetchNotableRecentRequest {
            region_code: region.to_string(),
            back: Some(7),
        })
        .await
        .map_err(|e| format!("Failed to fetch notable sightings: {}", e))?;

    let hotspots = client
        .send(FetchRegionHotspotsRequest {
            region_code: region.to_string(),
            back: Some(7),
        })
        .await
        .map_err(|e| format!("Failed to fetch hotspots: {}", e))?;

    // Format the data
    let notable_text = format_notable(&notable);
    let hotspots_text = format_hotspots(&hotspots);

    let target_info = if let Some(species) = target_species {
        let species_obs = client
            .send(FetchSpeciesRecentRequest {
                region_code: region.to_string(),
                species_code: species.to_string(),
                back: Some(14),
            })
            .await
            .map_err(|e| format!("Failed to fetch species observations: {}", e))?;

        format!(
            "\n\n### Target Species: {}\nRecent sightings (last 14 days):\n{}",
            species,
            format_observations(&species_obs)
        )
    } else {
        String::new()
    };

    // Build the prompt
    let prompt_text = format!(
        "Plan a {}-hour birding day trip in {}.\n\n\
         ## Available Data\n\n\
         ### Recent Notable/Rare Sightings (last 7 days)\n{}\n\n\
         ### Nearby Hotspots\n{}{}\n\n\
         ## Your Task\n\n\
         Create an hour-by-hour itinerary that:\n\
         1. Maximizes chances of seeing rare/notable species\n\
         2. Considers travel time between locations (use coordinates to estimate)\n\
         3. Accounts for bird activity patterns (dawn chorus, midday lull, evening activity)\n\
         4. Prioritizes locations with recent activity\n\
         5. Lists specific species to look for at each stop\n\
         6. Suggests observation techniques and tips\n\n\
         Format your response as a clear, actionable schedule with times, locations, and targets.\n\n\
         You have access to additional tools if you need real-time data:\n\
         - fetch_hotspot_recent: Get latest observations at a specific hotspot\n\
         - fetch_species_nearest: Find where a species was recently seen\n\
         - get_region_info: Get details about regions",
        duration_hours, region_info.result, notable_text, hotspots_text, target_info
    );

    Ok(GetPromptResult {
        description: None,
        messages: vec![PromptMessage::new_text(
            PromptMessageRole::User,
            prompt_text,
        )],
    })
}

pub async fn find_species(
    client: &ApiClient,
    args: &Map<String, Value>,
) -> Result<GetPromptResult, String> {
    let species_code = args
        .get("species_code")
        .and_then(|v| v.as_str())
        .ok_or("Missing required argument: species_code")?;

    let region = args
        .get("region")
        .and_then(|v| v.as_str())
        .ok_or("Missing required argument: region")?;

    // Fetch recent species observations
    let observations = client
        .send(FetchSpeciesRecentRequest {
            region_code: region.to_string(),
            species_code: species_code.to_string(),
            back: Some(14),
        })
        .await
        .map_err(|e| format!("Failed to fetch species observations: {}", e))?;

    let obs_text = format_observations(&observations);

    let prompt_text = format!(
        "Find the best locations and times to see **{}** in {}.\n\n\
         ## Recent Sightings (last 14 days)\n{}\n\n\
         ## Your Task\n\n\
         Based on these recent sightings, recommend:\n\
         1. **Top 3 locations** with the highest probability of seeing this species\n\
         2. **Best times of day** based on when it's been observed\n\
         3. **Identification tips** specific to this species\n\
         4. **Habitat preferences** where it's been found\n\
         5. **Recent trends** (increasing/stable/decreasing sightings)\n\n\
         If there are no recent sightings, suggest:\n\
         - Whether it's currently in range (seasonal patterns)\n\
         - Where it was last seen (use fetch_historic or expand the search)\n\
         - Alternative species to look for instead\n\n\
         You can use these tools for more information:\n\
         - fetch_hotspot_recent: Check specific hotspot activity\n\
         - fetch_species_nearest: Find nearest sightings by coordinates",
        species_code, region, obs_text
    );

    Ok(GetPromptResult {
        description: None,
        messages: vec![PromptMessage::new_text(
            PromptMessageRole::User,
            prompt_text,
        )],
    })
}

// Helper functions to format data
fn format_notable(observations: &[crate::tools::observations::Observation]) -> String {
    if observations.is_empty() {
        return "No notable sightings in the last 7 days.".to_string();
    }

    observations
        .iter()
        .take(15)
        .map(|obs| {
            format!(
                "- **{}** ({}) at {} on {} (count: {})",
                obs.common_name,
                obs.scientific_name,
                obs.location,
                obs.observation_date,
                obs.count.map(|c| c.to_string()).unwrap_or_else(|| "X".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_hotspots(hotspots: &[crate::tools::hotspot::Hotspot]) -> String {
    if hotspots.is_empty() {
        return "No hotspots found.".to_string();
    }

    hotspots
        .iter()
        .take(10)
        .map(|h| {
            format!(
                "- **{}** (ID: {}) — {:.4}°N, {:.4}°W — {} species all-time",
                h.name,
                h.location_id,
                h.latitude,
                h.longitude,
                h.num_species_all_time.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_observations(observations: &[crate::tools::observations::Observation]) -> String {
    if observations.is_empty() {
        return "No recent observations found.".to_string();
    }

    observations
        .iter()
        .take(20)
        .map(|obs| {
            format!(
                "- {} at {} on {} (count: {})",
                obs.common_name,
                obs.location,
                obs.observation_date,
                obs.count.map(|c| c.to_string()).unwrap_or_else(|| "X".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
