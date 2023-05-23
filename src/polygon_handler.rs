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
        Ok(map) => Ok((
            StatusCode::OK,
            Json(types::OutputData::from_cellmap(map.map())),
        )),
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

pub async fn polygon_handler_shm(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    #![allow(unreachable_code, unused_variables)]
    println!("=== Request received! ===");
    return Err((
        StatusCode::NOT_IMPLEMENTED,
        "TODO: Write JSON back to shared memory",
    ));
    todo!("Write JSON back to shared memory");
    let now = Instant::now();

    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            let output = Json(types::OutputData::from_cellmap(map.map()));
            Ok(StatusCode::OK)
        }
        Err(e) => match e {
            NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algorithm was provided",
            )),
        },
    };

    println!("Time elaposed: {:?}", now.elapsed());
    result
}
