use axum::{http::StatusCode, Json};
use local_robot_map::{
    AxisResolution, CellMap, LocalMap, Partition, PartitionError::NoPartitioningAlgorithm,
    PolygonMap, RealWorldLocation,
};
use serde::{Deserialize, Serialize};

pub async fn polygon_handler(
    Json(data): Json<InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<OutputData>), (StatusCode, &'static str)> {
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

    match map.partition() {
        Ok(map) => Ok((StatusCode::OK, Json(OutputData::from_cellmap(map.map())))),
        Err(e) => match e {
            NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning aglrotihm was provided",
            )),
        },
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct CoordXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl CoordXYZ {
    fn into_real_world(self) -> RealWorldLocation {
        RealWorldLocation::from_xyz(self.x, self.y, self.z)
    }
    fn into_axis_resolution(self) -> AxisResolution {
        AxisResolution::new(self.x, self.y, self.z)
    }
}

#[derive(Deserialize, Debug)]
pub struct InputData {
    vertices: Vec<CoordXYZ>,
    resolution: CoordXYZ,
    me: CoordXYZ,
    others: Vec<CoordXYZ>,
}

#[derive(Serialize)]
pub struct OutputData {
    cells: Vec<(CoordXYZ, &'static str)>,
}

impl OutputData {
    fn from_cellmap(map: &CellMap) -> Self {
        Self {
            cells: map
                .cells()
                .indexed_iter()
                .map(|((row, col), e)| {
                    (
                        CoordXYZ {
                            x: col as f64,
                            y: row as f64,
                            z: 0.0,
                        },
                        e.into(),
                    )
                })
                .collect(),
        }
    }
}

fn make_localmap(
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
