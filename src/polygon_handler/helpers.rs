use local_robot_map::{AxisResolution, CellMap, LocalMap, PolygonMap, RealWorldLocation};

pub(super) fn make_localmap(
    vertices: Vec<RealWorldLocation>,
    resolution: AxisResolution,
    my_position: RealWorldLocation,
    other_positions: Vec<RealWorldLocation>,
) -> LocalMap<CellMap> {
    let map = LocalMap::new_noexpand(
        PolygonMap::new(vertices).to_cell_map(resolution),
        my_position,
        other_positions,
    )
    .expect("All robots are in the map area");

    println!("My position: {:?}", map.my_position());
    println!("My other positions: {:#?}", map.other_positions());
    println!(
        "My map dimensions: {:#?} x {:#?}",
        map.map().width(),
        map.map().height()
    );
    println!("Map offset {:?}", map.map().offset());

    map
}
