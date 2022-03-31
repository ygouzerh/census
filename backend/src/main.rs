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
use commons::*;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    period: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataSet {
    #[serde(rename(deserialize = "dataSet"))]
    data_set: Vec<Data>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug)]
struct Census {
    population: Vec::<commons::Population>
}

async fn census() -> impl IntoResponse {

    // let json_value = serde_json::json!({
    //     "header": {
    //         "title" : "Census"
    //     },
    //     "data_set": [
    //       {
    //         "period": "2020-2022",
    //         "Age" : "25-30",
    //         "figure" : 1000
    //       }
    //     ]
    //   });
    // let dataset : DataSet = serde_json::from_value(json_value).unwrap();
    // println!("${:?}", dataset);
    let census_fetched : DataSet = reqwest::get("https://www.censtatd.gov.hk/api/get.php?id=216&lang=en&full_series=1")
        .await
        .unwrap()
        .json::<DataSet>()
        .await
        .unwrap();
    // let dataset : DataSet = serde_json::from_value(census).unwrap();
    println!("${:?}", census_fetched.data_set[1].extra.get("AgeDesc").unwrap());
    let population : Vec::<commons::Population> = census_fetched.data_set.iter().map(|data| {
        if let Some(age_json_value) = data.extra.get("AgeDesc") {
            if let Some(count_json_value) = data.extra.get("figure") {
                if let Some(district_json_value) = data.extra.get("DCDesc") {
                    return Population {
                        age: serde_json::from_value(age_json_value.clone()).unwrap(),
                        count: serde_json::from_value(count_json_value.clone()).unwrap(),
                        district: serde_json::from_value(district_json_value.clone()).unwrap()
                    }
                }
            }
        }
        Population {
            age : "".to_string(),
            count : 0,
            district : "".to_string()
        }
    })
    .filter(|population|
        population.age != "".to_string()
    )
    .collect();
    let census = Census {
        population : population
    };
    // println!("{:?}", census);
    // let census = vec![
    //     Population {
    //         age: String::from("18 - 24"),
    //         count: 180
    //     },
    //     Population {
    //         age: String::from("25 - 60"),
    //         count: 82
    //     }
    // ];
    Json(census.population)
}






