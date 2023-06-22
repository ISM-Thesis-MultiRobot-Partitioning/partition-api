use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;

mod polygon_handler;
use polygon_handler::*;

mod partition_schemes;
use partition_schemes as ps;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(help_message))
        .route(
            "/PolygonToCellMap",
            post(|e| polygon_handler_json(e, ps::bydistance)),
        )
        .route(
            "/PolygonToCellMapFrontiers",
            post(|e| polygon_handler_frontiers_json(e, ps::bydistance_frontiers)),
        )
        .route(
            "/PolygonToCellMapContours",
            post(|e| polygon_handler_frontiers_json(e, ps::bydistance_contours)),
        )
        .route(
            "/PolygonToCellMapContoursAngularSorted",
            post(|e| polygon_handler_contours_polar_angular_sort(e, ps::bydistance_contours)),
        )
        .route(
            "/PolygonToCellMapConvexHull",
            post(|e| polygon_handler_contours_convex_hull(e, ps::bydistance_contours)),
        )
        .route(
            "/PolygonToCellMapConcaveHull",
            post(|e| polygon_handler_contours_concave_hull(e, ps::bydistance_contours)),
        )
        .route(
            "/PolygonToCellMapShm",
            post(|e| polygon_handler_shm(e, ps::bydistance)),
        )
        .route(
            "/PolygonToCellMapFilePath",
            post(|e| polygon_handler_filepath(e, ps::bydistance)),
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
