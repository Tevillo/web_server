use axum::{
    Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use sled::Db;
use std::sync::Arc;

pub fn route_data(db: Arc<Db>) -> Router {
    Router::new()
        .route("/get/{key}", get(get_value))
        .route("/set/{key}/{value}", get(set_value))
        .with_state(db.clone())
}
async fn set_value(
    State(db): State<Arc<Db>>,
    Path((key, value)): Path<(String, String)>,
) -> impl IntoResponse {
    println!("Setting value for key: {}", key);
    db.insert(key.as_bytes(), value.as_bytes()).unwrap();
    db.flush().unwrap();
    format!("Set {key} = {value}")
}

async fn get_value(State(db): State<Arc<Db>>, Path(key): Path<String>) -> impl IntoResponse {
    println!("Getting value for key: {}", key);
    match db.get(key.as_bytes()).unwrap() {
        Some(value) => {
            let s = String::from_utf8(value.to_vec()).unwrap();
            format!("Value: {}", s)
        }
        None => "Not found".to_string(),
    }
}
