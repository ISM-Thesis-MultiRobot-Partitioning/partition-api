use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router};
use local_robot_map::{CellMap, LocalMap, MapState};
use local_robot_map::{Location, MaskMapState};
use serde::{Deserialize, Serialize};

mod polygon_handler;
use polygon_handler::polygon_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(help_message))
        .route(
            "/PolygonToCellMap",
            post(|e| polygon_handler(e, bydistance)),
        )
        .route("/test", post(test_add));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Input {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct Output {
    result: i32,
}

async fn test_add(Json(input): Json<Input>) -> (StatusCode, Json<Output>) {
    (
        StatusCode::OK,
        Json(Output {
            result: input.a + input.b,
        }),
    )
}

// let vertices = vec![
//     RealWorldLocation::from_xyz(50.0, 100.0, 0.0),
//     RealWorldLocation::from_xyz(200.0, 30.0, 0.0),
//     RealWorldLocation::from_xyz(350.0, 120.0, 0.0),
// ];
//
// let polygonmap = PolygonMap::new(vertices);
// let resolution = AxisResolution::uniform(1.0);
// let cellmap = polygonmap.to_cell_map(resolution);
//
// let my_position = RealWorldLocation::from_xyz(100.0, 80.0, 0.0);
// // let my_position = RealWorldLocation::from_xyz(20.0, 12.0, 0.0);
// let other_positions = vec![
//     // RealWorldLocation::from_xyz(299.0, 100.0, 0.0),
//     // RealWorldLocation::from_xyz(200.0, 60.0, 0.0),
//     RealWorldLocation::from_xyz(130.0, 50.0, 0.0),
//     RealWorldLocation::from_xyz(80.0, 80.0, 0.0),
// ];
//
// let mut map = LocalMap::new_noexpand(cellmap, my_position,
// other_positions).unwrap();
//
// let mut map = make_localmap(vertices, resolution, my_position,
// other_positions); map.set_partition_algorithm(bydistance);
// let map = map.partition().unwrap();
// map.as_image().save("the-map.png").unwrap();

async fn help_message() -> Html<&'static str> {
    Html(
        "
         <h1>Routes</h1>
         <h2>/</h2>
         Display this help page.
         <h2>/PolygonToCellMap</h2>
         ",
    )
}

fn bydistance(mut map: LocalMap<CellMap>) -> LocalMap<CellMap> {
    let mut cells_to_assign = Vec::new();

    for cell in map.map().get_map_state(MapState::Unexplored) {
        let my_dist = map.my_position().distance(cell.location());
        let other_dists = map
            .other_positions()
            .iter()
            .map(|loc| loc.distance(cell.location()));
        if map.other_positions().is_empty() || other_dists.into_iter().all(|dist| my_dist < dist) {
            cells_to_assign.push(cell.location().clone());
        }
    }

    for location in &cells_to_assign {
        map.map_mut()
            .set_location(location, MapState::Assigned)
            .expect("All locations are in the map");
    }

    map
}
