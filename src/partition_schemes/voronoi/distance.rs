//! A simple distance based partitioning.

use local_robot_map::{Location, MaskMapState};
use local_robot_map::{LocationType, RealWorldLocation};

use crate::ps::Factors;
use crate::Map;

pub fn bydistance(mut map: Map, factors: Option<Factors>) -> Map {
    let mut cells_to_assign: Vec<RealWorldLocation> = Vec::new();

    for cell in map.map().get_map_state(LocationType::Unexplored) {
        if map.other_positions().is_empty() {
            cells_to_assign.push(cell.location().clone());
        }
        let my_dist = map.my_position().distance(cell.location());
        let other_dists = map
            .other_positions()
            .into_iter()
            .map(|loc| loc.distance(cell.location()));
        match factors {
            Some(_) => todo!(),
            None => {
                if other_dists.into_iter().all(|dist| my_dist < dist) {
                    cells_to_assign.push(cell.location().clone());
                }
            }
        }
    }

    for location in &cells_to_assign {
        map.map_mut()
            .set_location(location, LocationType::Assigned)
            .expect("All locations are in the map");
    }

    map
}
