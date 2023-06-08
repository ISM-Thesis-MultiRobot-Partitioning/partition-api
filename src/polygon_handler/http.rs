use std::time::Instant;

use axum::{http::StatusCode, Json};
use local_robot_map::{AxisResolution, Coords, LocationType, MaskMapState};
use local_robot_map::{CellMap, LocalMap, PartitionError};

use super::helpers;
use super::types;

/// Partitiong a polygon map and return all cells.
///
/// Returns all cells in matrix coordinates. Corresponding offset and resolution
/// will also be provided to let the client convert the coordinates into
/// real-world locations.
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_json(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    println!(">>> Partition map and return all cells");
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
            PartitionError::NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algortihm was provided",
            )),
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}

/// Partition a polygon and return only border cells of assigned region.
///
/// Returns all the cells marked as [`LocationType::Frontier`] in real-world
/// coordinates. Liberty has been taking in interpreting the *frontier* to be
/// the border between [`LocationType::Assigned`] and everything else.
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_frontiers_json(
    Json(data): Json<types::InputData>,
    algorithm: fn(LocalMap<CellMap>) -> LocalMap<CellMap>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    println!(">>> Partition map and return frontier cells (edge of assigned region)");
    let now = Instant::now();
    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());
            Ok((
                StatusCode::OK,
                Json(types::OutputData::new(
                    map.map()
                        .get_map_state(LocationType::Frontier)
                        .iter()
                        .map(|c| (c.location().into(), c.value().into()))
                        .collect(),
                    (&Coords::new(0.0, 0.0, 0.0)).into(),
                    (&<AxisResolution as Default>::default()).into(),
                )),
            ))
        }
        Err(e) => match e {
            PartitionError::NoPartitioningAlgorithm => Err((
                StatusCode::NOT_IMPLEMENTED,
                "No partitioning algortihm was provided",
            )),
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}
