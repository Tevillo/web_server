use axum::Router;
use axum_server::tls_rustls::RustlsConfig;

mod back;
mod front;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(route_all());
    let config = RustlsConfig::from_pem_file(
        "/home/pborrego/cert/cert.pem",
        "/home/pborrego/cert/key.pem",
    )
    .await
    .expect("failed to load TLS config");

    // Define the address
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Starting server at {:?}!", addr);

    // Serve with HTTPS
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn route_all() -> Router {
    front::route_pages().merge(front::route_elements())
}
