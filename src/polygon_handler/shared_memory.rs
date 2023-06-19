use std::time::Instant;

use axum::{http::StatusCode, Json};
use local_robot_map::PartitionError;
use local_robot_map::{CellMap, LocalMap};

use super::helpers;
use super::types;

/// Partitiong a polygon map and return all cells. Uses *shared memory*.
///
/// Returns all cells in matrix coordinates. Corresponding offset and resolution
/// will also be provided to let the client convert the coordinates into
/// real-world locations.
///
/// It works the same as [`crate::polygon_handler::polygon_handler_json`].
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_shm(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    #![allow(unreachable_code, unused_variables)]
    println!("=== Request received! ===");
    println!(">>> Partition map and return all cells (uses shared memory)");
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
            PartitionError::NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algorithm was provided",
            )),
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };

    println!("Time elaposed: {:?}", now.elapsed());
    result
}
