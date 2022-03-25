use axum::{
    routing::{get, get_service},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use commons::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/api/census", get(census))
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

async fn census() -> impl IntoResponse {
    let census = vec![
        Population {
            age: String::from("18 - 24"),
            count: 180
        },
        Population {
            age: String::from("25 - 60"),
            count: 82
        }
    ];
    Json(census)
}