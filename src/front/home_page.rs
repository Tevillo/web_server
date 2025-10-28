use axum::response::{Html, IntoResponse};

use hypertext::Buffer;

use crate::front::common;

pub async fn get() -> impl IntoResponse {
    println!("--> - handler_index -");

    let mut buffer = Buffer::new();

    common::nav_bar("/home", &mut buffer).await;

    Html(buffer.rendered().into_inner())
}
