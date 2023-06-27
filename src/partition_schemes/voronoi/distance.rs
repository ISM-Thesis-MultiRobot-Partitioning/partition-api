//! A simple distance based partitioning.

use local_robot_map::{Location, MaskMapState};
use local_robot_map::{LocationType, RealWorldLocation};

use crate::Map;

pub fn bydistance(mut map: Map) -> Map {
    let mut cells_to_assign: Vec<RealWorldLocation> = Vec::new();

    for cell in map.map().get_map_state(LocationType::Unexplored) {
        if map.other_positions().is_empty() {
            cells_to_assign.push(cell.location().clone());
            continue;
        }
        let my_score = map.my_position().distance(cell.location());
        let other_scores = map
            .other_positions()
            .into_iter()
            .map(|loc| loc.distance(cell.location()));
        if other_scores.into_iter().all(|score| my_score < score) {
            cells_to_assign.push(cell.location().clone());
        }
    }

    for location in &cells_to_assign {
        map.map_mut()
            .set_location(location, LocationType::Assigned)
            .expect("All locations are in the map");
    }

    map
}
