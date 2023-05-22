use std::time::Instant;

use axum::{http::StatusCode, Json};
use local_robot_map::{CellMap, LocalMap, PartitionError::NoPartitioningAlgorithm};

mod helpers;
mod types;

pub async fn polygon_handler(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    let now = Instant::now();
    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
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
