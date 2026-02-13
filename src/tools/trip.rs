/// Returns structured birding trip planning guidelines.
///
/// Provides comprehensive best practices for planning birding trips, including:
/// - Optimal timing (dawn, midday, evening considerations)
/// - Location sequencing strategies
/// - Break and meal planning
/// - Hotspot prioritization
/// - Species targeting
/// - Weather considerations
/// - Itinerary structure
///
/// # Examples
///
/// ```
/// use rubl::tools::trip::get_trip_planning_guidelines;
///
/// let guidelines = get_trip_planning_guidelines();
/// assert!(guidelines.contains("Dawn"));
/// assert!(guidelines.contains("Time of Day"));
/// assert!(guidelines.contains("Location Sequencing"));
/// ```
pub fn get_trip_planning_guidelines() -> String {
    r#"# Birding Trip Planning Guidelines

## Time of Day Considerations
- **Dawn (30 min before sunrise to 2-3 hours after)**: Peak activity period. Birds are most vocal and active. Prioritize best habitats during this window.
- **Midday (10am-3pm)**: Reduced activity. Good time for travel between locations, lunch breaks, or checking water sources where birds gather.
- **Late afternoon/evening (2-3 hours before sunset)**: Secondary activity peak. Birds feed actively before roosting.
- **Golden hour**: Last hour before sunset often produces interesting lighting for photography and renewed bird activity.

## Location Sequencing
- Start at the most productive location at dawn when activity peaks
- Group nearby locations together to minimize travel time
- Use midday lull for longer drives between regions
- Check coordinates and calculate realistic travel times (assume 30-45 mph average for rural areas)
- Consider road conditions and accessibility

## Breaks and Meals
- Plan a proper lunch break during midday lull (11am-1pm)
- Schedule 15-20 min breaks every 2-3 hours for rest and hydration
- Morning coffee/snack stop can be productive if at a good birding location
- Bring snacks and water - birding often takes longer than expected

## Hotspot Prioritization
- Prioritize locations with recent notable/rare sightings (last 7 days)
- Check number of recent checklists - more checklists = more reliable data
- Consider habitat diversity - varied habitats = more species
- Note access restrictions, hours, parking availability
- Have backup locations in case primary spots are unproductive

## Species Targeting
- Research target species' habitat preferences and behaviors
- Check recent observations to confirm presence
- Note best times for specific species (e.g., raptors more active midday, rails at dawn/dusk)
- Plan routes that maximize habitat types for target species

## Weather Considerations
- Check forecast - wind, rain, temperature affect bird activity
- Light winds (5-15 mph) are ideal; heavy winds reduce activity
- Overcast can extend morning activity period
- Extreme heat reduces activity; focus on shaded areas and water

## Itinerary Structure
Present trip plans as:
1. Overview (total duration, regions covered, key targets)
2. Route map with locations marked and paths between them
3. Hour-by-hour schedule with:
   - Time blocks
   - Location names with coordinates
   - Travel time between stops
   - Target species for each location
   - Specific tips or techniques for the habitat
4. Alternatives/contingencies for weather or unproductive locations
"#
    .to_string()
}
