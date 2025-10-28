use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use axum_extra::response::{Css, JavaScript};

pub async fn style_server() -> Css<&'static str> {
    let css: &str = include_str!("../../public/css/server.css");
    Css(css)
}

pub async fn style_schedule() -> Css<&'static str> {
    let css: &str = include_str!("../../public/css/schedule.css");
    Css(css)
}

pub async fn style_base() -> Css<&'static str> {
    let css: &str = include_str!("../../public/css/base.css");
    Css(css)
}

pub async fn script_server() -> JavaScript<&'static str> {
    let server_script: &str = include_str!("../../public/scripts/server.js");
    JavaScript(server_script)
}

pub async fn images(Path(p): Path<String>) -> impl IntoResponse {
    println!("-> image handler {:?}", p);
    let full_path = format!("public/images/{}", p);
    let path = std::path::Path::new(&full_path);
    let file = match tokio::fs::read(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime.as_ref())
        .body(Body::from(file))
        .unwrap())
}
