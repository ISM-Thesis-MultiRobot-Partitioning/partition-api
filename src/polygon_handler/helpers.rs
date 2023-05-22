use local_robot_map::{
    AxisResolution, CellMap, LocalMap, Partition, PartitionError, PolygonMap, RealWorldLocation,
    Visualize,
};

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

/// Takes care of the heavy lifting for transforming the data.
///
/// You can pass it the JSON data/struct and it will do all the type
/// conversions. Additionally it will perform the partitioning and return its
/// result.
///
/// As a matter of convenience, the map is also saved to a PNG file if
/// partitioning was successful.
///
/// # Errors
///
/// This function will return a [`PartitionError`] if the partitioning failed.
pub(super) fn partition_input_data(
    data: super::types::InputData,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<LocalMap<CellMap>, PartitionError> {
    let mut map: LocalMap<CellMap> = make_localmap(
        data.vertices
            .into_iter()
            .map(|v| v.into_real_world())
            .collect(),
        data.resolution.into_axis_resolution(),
        data.me.into_real_world(),
        data.others
            .into_iter()
            .map(|v| v.into_real_world())
            .collect(),
    );
    map.set_partition_algorithm(algorithm);
    let map = map.partition();
    if let Ok(ref map) = map {
        map.as_image().save("map.png").unwrap();
    }
    map
}
