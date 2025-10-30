pub mod common;
pub mod elements;
pub mod home_page;
pub mod schedule_page;
pub mod server_page;

use axum::{
    Router,
    routing::{get, post},
};

pub fn route_pages() -> Router {
    Router::new()
        .route("/", get(home_page::get))
        .route("/home", get(home_page::get))
        .route("/schedule", get(schedule_page::get))
        .route("/server", get(server_page::get).post(server_page::post))
        .route("/get_servers", post(server_page::get_servers))
}

pub fn route_elements() -> Router {
    Router::new()
        .route("/style/server.css", get(elements::style_server))
        .route("/style/schedule.css", get(elements::style_schedule))
        .route("/style/base.css", get(elements::style_base))
        .route("/scripts/server.js", get(elements::script_server))
        .route("/scripts/schedule.js", get(elements::script_schedule))
        .route("/public/images/{name}", get(elements::images))
}
