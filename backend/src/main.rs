use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use commons::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/census", get(census))
        // `POST /users` goes to `create_user`
        .route("/api/users", get(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize)]
struct Census {
    census: Vec<Population>
}

async fn census() -> impl IntoResponse {
    let census = vec![
        Population {
            age: String::from("18 - 24"),
            count: 180
        }
    ];
    Json(census)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: String::from("Hello"),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Json(user)
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}