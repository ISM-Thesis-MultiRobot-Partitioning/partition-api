use local_robot_map::{
    Algorithm, AxisResolution, LocalMap, Partition, PartitionError, PolygonMap,
    PolygonMapError, RealWorldLocation, Visualize,
};

use crate::{Map, RobotLocation};

pub(super) fn make_localmap(
    vertices: Vec<RealWorldLocation>,
    explored: Option<Vec<Vec<RealWorldLocation>>>,
    resolution: AxisResolution,
    my_position: RobotLocation,
    other_positions: Vec<RobotLocation>,
) -> Result<Map, PolygonMapError> {
    let map = LocalMap::new_noexpand_nooutofmap(
        PolygonMap::new_explored(vertices, explored)?.to_cell_map(resolution),
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

    Ok(map)
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
/// This function will return a [`PartitionError`] if the partitioning failed. A
/// [`PolygonMapError`] can also cause an error to be returned.
pub(super) fn partition_input_data(
    data: super::types::InputData,
    algorithm: Algorithm<Map>,
) -> Result<Map, PartitionError> {
    let map: Map = match make_localmap(
        data.vertices
            .into_iter()
            .map(|v| v.into_real_world())
            .collect(),
        data.explored.map(|e| {
            e.into_iter()
                .map(|polygon| {
                    polygon.into_iter().map(|v| v.into_real_world()).collect()
                })
                .collect()
        }),
        data.resolution.into_axis_resolution(),
        data.me.into(),
        data.others.into_iter().map(|v| v.into()).collect(),
    ) {
        Ok(m) => m,
        Err(e) => match e {
            PolygonMapError::NotEnoughVertices => {
                return Err(PartitionError::NoMap)
            }
        },
    };
    let map = map.partition(algorithm);
    if let Ok(ref map) = map {
        map.as_image().save("map.png").unwrap();
    }
    map
}

/// Trait for dealing with Polar coordinates given Cartesian coordinates.
///
/// The points are assumed to not be in the centroid's reference frame. This
/// trait's functions should handle the translation and perform the calculation
/// accordingly.
pub(super) trait Polar {
    /// Compute the angular coordinate of a point relative to a centroid.
    ///
    /// The angle is computed using [`f64::atan2`].
    ///
    /// Inspired by:
    /// - <https://en.wikipedia.org/wiki/Polar_coordinate_system#Converting_between_polar_and_Cartesian_coordinates>
    /// - <https://stackoverflow.com/a/6989383>
    fn angular_coordinate(&self, centroid: &Self) -> f64;

    /// Compute radial coordinate (distance) of a point relative to a centroid.
    ///
    /// The distance is computed using the euclidean distance.
    fn radial_coordinate(&self, centroid: &Self) -> f64;
}

impl Polar for RealWorldLocation {
    fn angular_coordinate(&self, centroid: &Self) -> f64 {
        (self.y() - centroid.y()).atan2(self.x() - centroid.x())
    }

    fn radial_coordinate(&self, centroid: &Self) -> f64 {
        // we ignore the 3rd dimension
        let point1 = RealWorldLocation::from_xyz(self.x(), self.y(), 0.0);
        let point2 =
            RealWorldLocation::from_xyz(centroid.x(), centroid.y(), 0.0);

        point1.distance(&point2)
    }
}
