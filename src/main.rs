use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use sled::Db;
use std::sync::Arc;

mod back;
mod database;
mod front;

#[tokio::main]
async fn main() {
    // Define the address
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server Initialized at {:?}!", addr);

    // Initialize the database
    let db = Arc::new(sled::open("data").expect("failed to open database"));
    println!("Database Initialized");

    let app = route_all(db);

    // Self signed certs
    let config = RustlsConfig::from_pem_file(
        "/home/pborrego/cert/cert.pem",
        "/home/pborrego/cert/key.pem",
    )
    .await
    .expect("failed to load TLS config");

    // Serve with HTTPS
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn route_all(db: Arc<Db>) -> Router {
    front::route_pages(db).merge(front::route_elements())
}
