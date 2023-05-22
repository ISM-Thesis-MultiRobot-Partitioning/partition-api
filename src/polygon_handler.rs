use std::time::Instant;

use axum::{http::StatusCode, Json};
use local_robot_map::{
    CellMap, LocalMap, Partition, PartitionError::NoPartitioningAlgorithm, Visualize,
};

mod helpers;
mod types;

pub async fn polygon_handler(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    let now = Instant::now();
    let mut map: LocalMap<CellMap> = helpers::make_localmap(
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

    let result = match map.partition() {
        Ok(map) => {
            map.as_image().save("map.png").unwrap();
            Ok((
                StatusCode::OK,
                Json(types::OutputData::from_cellmap(map.map())),
            ))
        }
        Err(e) => match e {
            NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algortihm was provided",
            )),
        },
    };

    println!("Time elapsed: {:?}", now.elapsed());
    result
}
