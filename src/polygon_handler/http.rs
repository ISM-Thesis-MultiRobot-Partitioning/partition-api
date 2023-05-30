use std::time::Instant;

use axum::{http::StatusCode, Json};
use local_robot_map::{CellMap, LocalMap, PartitionError::NoPartitioningAlgorithm};
use local_robot_map::{MapState, MaskMapState};

use super::helpers;
use super::types;

pub async fn polygon_handler_json(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    let now = Instant::now();
    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());
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
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}

pub async fn polygon_handler_frontiers_json(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    let now = Instant::now();
    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());
            Ok((
                StatusCode::OK,
                Json(types::OutputData::new(
                    map.map()
                        .get_map_state(MapState::Frontier)
                        .iter()
                        .map(|c| (c.location().into(), c.value().into()))
                        .collect(),
                    map.map().offset().into(),
                    map.map().resolution().into(),
                )),
            ))
        }
        Err(e) => match e {
            NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algortihm was provided",
            )),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}
