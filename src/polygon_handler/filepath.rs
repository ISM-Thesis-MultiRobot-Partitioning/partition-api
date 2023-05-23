use std::time::Instant;

use axum::http::StatusCode;
use local_robot_map::{CellMap, LocalMap, PartitionError::NoPartitioningAlgorithm};

use super::helpers;
use super::types;

pub async fn polygon_handler_filepath(
    file_path: String,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("=== Request received! ===");
    println!("File path: {}", file_path);
    let now = Instant::now();

    let data: types::InputData = match std::fs::read_to_string(&file_path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Could not convert to JSON: {e}"),
                ))
            }
        },
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not read file: {e}"),
            ))
        }
    };

    println!("Read data from file ({:?})", now.elapsed());

    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());
            match serde_json::to_string(&types::OutputData::from_cellmap(map.map())) {
                Ok(json_string) => {
                    println!("Converted cellmap to JSON string ({:?})", now.elapsed());
                    match std::fs::write(&file_path, json_string) {
                        Ok(_) => {
                            println!("Wrote data back to file ({:?})", now.elapsed());
                            Ok(StatusCode::OK)
                        }
                        Err(e) => {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Could not write file: {e}"),
                            ))
                        }
                    }
                }
                Err(e) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Could not serialize output to JSON string: {e}"),
                    ))
                }
            }
        }
        Err(e) => match e {
            NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algorithm was provided".into(),
            )),
        },
    };

    println!("Time elaposed: {:?}", now.elapsed());
    result
}
