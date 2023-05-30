use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use local_robot_map::{Cell, CellMap, Coords, LocalMap, LocationType, MapState, RealWorldLocation};
use local_robot_map::{Location, MaskMapState};

mod polygon_handler;
use polygon_handler::{
    polygon_handler_filepath, polygon_handler_frontiers_json, polygon_handler_json,
    polygon_handler_shm,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(help_message))
        .route(
            "/PolygonToCellMap",
            post(|e| polygon_handler_json(e, bydistance)),
        )
        .route(
            "/PolygonToCellMapFrontiers",
            post(|e| polygon_handler_frontiers_json(e, bydistance_frontiers)),
        )
        .route(
            "/PolygonToCellMapShm",
            post(|e| polygon_handler_shm(e, bydistance)),
        )
        .route(
            "/PolygonToCellMapFilePath",
            post(|e| polygon_handler_filepath(e, bydistance)),
        );

    let address = "0.0.0.0:8000";
    println!("Serving at {address} ...");
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

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
    let mut cells_to_assign: Vec<RealWorldLocation> = Vec::new();

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

fn bydistance_frontiers(map: LocalMap<CellMap>) -> LocalMap<CellMap> {
    bydistance(map).set_frontiers()
}

/// We shall take the liberty of interpreting the [`MapState::Frontier`] to be the frontier between
/// the [`MapState::Assigned`] region and everything else. It allows us to neatly embed everything
/// without introducing additional types.
trait Frontiers {
    fn set_frontiers(self) -> Self;
}

impl Frontiers for LocalMap<CellMap> {
    fn set_frontiers(mut self) -> Self {
        let width: u32 = self.map().width() as u32;
        let height: u32 = self.map().height() as u32;
        let img = image::ImageBuffer::from_fn(width, height, |x, y| -> image::Luma<u8> {
            let (row, col) = (y as usize, x as usize);
            let cell: LocationType = self.map().cells()[[row, col]];
            match cell {
                MapState::Assigned => image::Luma([255]),
                _ => image::Luma([0]),
            }
        });

        edge_detection::canny(
            img, 1.2,  // sigma
            0.2,  // strong threshold
            0.01, // weak threshold
        )
        .as_image()
        .to_luma8()
        .rows()
        .enumerate()
        .for_each(|(row, elements)| {
            elements.enumerate().for_each(|(col, value)| {
                if value != &[0u8].into() {
                    let location: RealWorldLocation = Cell::from_internal(
                        Coords::new(col as f64, row as f64, 0.0),
                        *self.map().offset(),
                        *self.map().resolution(),
                        &LocationType::Frontier,
                    )
                    .expect("Locations are inside the map")
                    .location()
                    .clone();
                    self.map_mut()
                        .set_location(&location, LocationType::Frontier)
                        .expect("Location is inside the map")
                }
            })
        });

        self
    }
}
