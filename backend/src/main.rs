use axum::{
    routing::{get, get_service},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use reqwest;
use commons::{Census, Districts};
use itertools::Itertools;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/api/census", get(census))
        .route("/api/districts", get(districts))
        .nest(
            "/static",
            get_service(ServeDir::new("frontend/dist")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn districts() -> impl IntoResponse {

    let census : Census = reqwest::get("https://www.censtatd.gov.hk/api/get.php?id=216&lang=en&full_series=1")
        .await
        .unwrap()
        .json::<Census>()
        .await
        .unwrap();
    let districts : Vec<Districts> = census.populations
                                    .into_iter()
                                    .map(|population| population.district)
                                    .unique()
                                    .collect();
    Json(districts)
}


async fn census() -> impl IntoResponse {

    let census : Census = reqwest::get("https://www.censtatd.gov.hk/api/get.php?id=216&lang=en&full_series=1")
        .await
        .unwrap()
        .json::<Census>()
        .await
        .unwrap();
    Json(census.populations)
}






