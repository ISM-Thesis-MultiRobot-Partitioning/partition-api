use std::time::Instant;

use axum::{http::StatusCode, Json};
use geo::{ConcaveHull, ConvexHull, CoordsIter};
use local_robot_map::{
    Algorithm, AxisResolution, Coords, LocationType, MaskMapState, PartitionError,
    RealWorldLocation,
};

use helpers::Polar;

use crate::Map;

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
    algorithm: Algorithm<Map>,
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
    algorithm: Algorithm<Map>,
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
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}

/// Same as [`polygon_handler_frontiers_json`], except that the frontiers being
/// returned are the result of a *convex hull* operation. The desired side
/// effect is that polygon points will be sorted in a circular and
/// sequential fashion. Another side effect is that the overall shape may end up
/// being simplified, thus resulting in an ever so slightly decrease in points
/// being returned.
///
/// Inpsired by this answer: <https://stackoverflow.com/a/6989383>
///
/// # CAREFUL
///
/// This handler returns a **modified** shape of the *actual* frontier. This may
/// lead to overlaps with [`LocationType::Assigned`] zones of other robots or
/// lead to entirely uncovered regions. The exact contours may not exactly
/// follow the initial region as well.
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_contours_convex_hull(
    Json(data): Json<types::InputData>,
    algorithm: Algorithm<Map>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    println!(">>> Partition map and return convex hull contour cells (edge of assigned region)");
    let now = Instant::now();
    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());
            Ok((
                StatusCode::OK,
                Json(types::OutputData::new(
                    geo::Polygon::new(
                        geo::LineString::from(
                            map.map()
                                .get_map_state(LocationType::Frontier)
                                .iter()
                                .map(|c| (c.location().x(), c.location().y()))
                                .collect::<Vec<(f64, f64)>>(),
                        ),
                        vec![],
                    )
                    .convex_hull()
                    .exterior_coords_iter()
                    .map(|geo::Coord { x, y }| {
                        (
                            (&RealWorldLocation::new(Coords::new(x, y, 0.0))).into(),
                            (&LocationType::Frontier).into(),
                        )
                    })
                    .collect(),
                    (&Coords::new(0.0, 0.0, 0.0)).into(),
                    (&<AxisResolution as Default>::default()).into(),
                )),
            ))
        }
        Err(e) => match e {
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}

/// Same as [`polygon_handler_frontiers_json`], except that the frontiers being
/// returned are the result of a *concave hull* operation.
///
/// # CAREFUL
///
/// Please see [`polygon_handler_contours_convex_hull`] equally named section.
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_contours_concave_hull(
    Json(data): Json<types::InputData>,
    algorithm: Algorithm<Map>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    println!(">>> Partition map and return concave hull contour cells (edge of assigned region)");
    let now = Instant::now();
    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());
            Ok((
                StatusCode::OK,
                Json(types::OutputData::new(
                    geo::Polygon::new(
                        geo::LineString::from(
                            map.map()
                                .get_map_state(LocationType::Frontier)
                                .iter()
                                .map(|c| (c.location().x(), c.location().y()))
                                .collect::<Vec<(f64, f64)>>(),
                        ),
                        vec![],
                    )
                    .concave_hull(1.0)
                    .exterior_coords_iter()
                    .map(|geo::Coord { x, y }| {
                        (
                            (&RealWorldLocation::new(Coords::new(x, y, 0.0))).into(),
                            (&LocationType::Frontier).into(),
                        )
                    })
                    .collect(),
                    (&Coords::new(0.0, 0.0, 0.0)).into(),
                    (&<AxisResolution as Default>::default()).into(),
                )),
            ))
        }
        Err(e) => match e {
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}

/// Same as [`polygon_handler_frontiers_json`], except that the frontiers being
/// returned are sorted according to their polar coordinate angles.
///
/// The polar coordinates are expressed using the centroid of the polygon. Each
/// point can then be ordered by in-/decreasing angles in the polar coordinate
/// system. This will effectively *sort* the points.
///
/// Inspiration from:
/// - <https://stackoverflow.com/a/7369725>
/// - <https://stackoverflow.com/a/6989416>
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_contours_polar_angular_sort(
    Json(data): Json<types::InputData>,
    algorithm: Algorithm<Map>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    println!(">>> Partition map and return frontier cells (edge of assigned region)");
    let now = Instant::now();

    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());

            let polygon = geo::Polygon::new(
                geo::LineString::from(
                    map.map()
                        .get_map_state(LocationType::Frontier)
                        .iter()
                        .map(|c| (c.location().x(), c.location().y()))
                        .collect::<Vec<(f64, f64)>>(),
                ),
                vec![],
            );
            println!("Transformed to polygon ({:?})", now.elapsed());

            match geo::Centroid::centroid(&polygon) {
                Some(c) => {
                    let centroid = RealWorldLocation::from_xyz(c.x(), c.y(), 0.0);
                    println!("Found centroid ({:?}) ({:?})", now.elapsed(), centroid);

                    let mut points: Vec<RealWorldLocation> = polygon
                        .exterior_coords_iter()
                        .map(|geo::Coord { x, y }| RealWorldLocation::from_xyz(x, y, 0.0))
                        .collect();
                    points.sort_by(|a, b| {
                        a.angular_coordinate(&centroid)
                            .partial_cmp(&b.angular_coordinate(&centroid))
                            .expect("Ordering f64 works")
                    });
                    println!("Sorted points ({:?})", now.elapsed());

                    Ok((
                        StatusCode::OK,
                        Json(types::OutputData::new(
                            points
                                .iter()
                                .map(|p| (p.into(), (&LocationType::Frontier).into()))
                                .collect(),
                            (&Coords::new(0.0, 0.0, 0.0)).into(),
                            (&<AxisResolution as Default>::default()).into(),
                        )),
                    ))
                }
                None => Err((
                    StatusCode::BAD_REQUEST,
                    "Assigned region with an invalid polygon centroid",
                )),
            }
        }
        Err(e) => match e {
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}

/// Same as [`polygon_handler_contours_polar_angular_sort`], except that the
/// frontiers being returned are sorted according to their polar angular AND
/// radial coordinate (i.e. angle and distance).
///
/// In addition to the sorting by angles, we also sort according to which points
/// are closest. This will solve the issue where lines passing infront of
/// another will lead to intertwining points from both lines.
///
/// # Errors
///
/// This function will return an error if no partitioning algorithm was
/// provided or if no viable map was provided through the input polygon points.
pub async fn polygon_handler_contours_polar_sort(
    Json(data): Json<types::InputData>,
    algorithm: Algorithm<Map>,
) -> Result<(StatusCode, Json<types::OutputData>), (StatusCode, &'static str)> {
    println!("=== Request received! ===");
    println!(">>> Partition map and return frontier cells (edge of assigned region)");
    let now = Instant::now();

    let result = match helpers::partition_input_data(data, algorithm) {
        Ok(map) => {
            println!("Partitioned map ({:?})", now.elapsed());

            let polygon = geo::Polygon::new(
                geo::LineString::from(
                    map.map()
                        .get_map_state(LocationType::Frontier)
                        .iter()
                        .map(|c| (c.location().x(), c.location().y()))
                        .collect::<Vec<(f64, f64)>>(),
                ),
                vec![],
            );
            println!("Transformed to polygon ({:?})", now.elapsed());

            match geo::Centroid::centroid(&polygon) {
                Some(c) => {
                    let centroid = RealWorldLocation::from_xyz(c.x(), c.y(), 0.0);
                    println!("Found centroid ({:?}) ({:?})", now.elapsed(), centroid);

                    let mut points: Vec<RealWorldLocation> = polygon
                        .exterior_coords_iter()
                        .map(|geo::Coord { x, y }| RealWorldLocation::from_xyz(x, y, 0.0))
                        .collect();
                    points.sort_by(|a, b| {
                        use std::cmp::Ordering;
                        let angular = a
                            .angular_coordinate(&centroid)
                            .partial_cmp(&b.angular_coordinate(&centroid))
                            .expect("Ordering f64 works");
                        let radial = a
                            .radial_coordinate(&centroid)
                            .partial_cmp(&b.radial_coordinate(&centroid))
                            .expect("Ordering f64 works");
                        match angular {
                            Ordering::Less => match radial {
                                Ordering::Less => Ordering::Less,
                                Ordering::Equal => Ordering::Less,
                                Ordering::Greater => Ordering::Greater,
                            },
                            Ordering::Equal => radial,
                            Ordering::Greater => match radial {
                                Ordering::Less => Ordering::Less,
                                Ordering::Equal => Ordering::Greater,
                                Ordering::Greater => Ordering::Greater,
                            },
                        }
                    });
                    println!("Sorted points ({:?})", now.elapsed());

                    Ok((
                        StatusCode::OK,
                        Json(types::OutputData::new(
                            points
                                .iter()
                                .map(|p| (p.into(), (&LocationType::Frontier).into()))
                                .collect(),
                            (&Coords::new(0.0, 0.0, 0.0)).into(),
                            (&<AxisResolution as Default>::default()).into(),
                        )),
                    ))
                }
                None => Err((
                    StatusCode::BAD_REQUEST,
                    "Assigned region with an invalid polygon centroid",
                )),
            }
        }
        Err(e) => match e {
            PartitionError::NoMap => Err((StatusCode::BAD_REQUEST, "No viable map was provided")),
        },
    };
    println!("Finished processing data ({:?})", now.elapsed());

    println!("Time elapsed: {:?}", now.elapsed());
    result
}
