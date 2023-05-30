use local_robot_map::{CellMap, LocalMap, MapState, RealWorldLocation};
use local_robot_map::{Location, MaskMapState};

pub fn bydistance(mut map: LocalMap<CellMap>) -> LocalMap<CellMap> {
    let mut cells_to_assign: Vec<RealWorldLocation> = Vec::new();

    for cell in map.map().get_map_state(MapState::Unexplored) {
        let my_dist = map.my_position().distance(cell.location());
        let other_dists = map
            .other_positions()
            .iter()
            .map(|loc| loc.distance(cell.location()));
        if map.other_positions().is_empty() || other_dists.into_iter().all(|dist| my_dist < dist) {
            cells_to_assign.push(cell.location().clone());
        }
    }

    for location in &cells_to_assign {
        map.map_mut()
            .set_location(location, MapState::Assigned)
            .expect("All locations are in the map");
    }

    map
}
