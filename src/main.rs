//use std::path::{Path, PathBuf};

use axum::{
    Json, Router,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};

use axum_extra::response::{Css, JavaScript};

use hypertext::{GlobalAttributes, Renderable, html_elements, maud};
use serde::Deserialize;

mod back;

use crate::back::server::{Action, Server, manipulate_server};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().merge(route_base());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn route_base() -> Router {
    Router::new()
        .route("/", get(handler_index))
        .route("/schedule", get(handler_schedule))
        .route("/server", get(handler_server).post(handler_server_put))
        .route("/style/server.css", get(style_server))
        .route("/style/schedule.css", get(style_schedule))
        .route("/scripts/server.js", get(script_server))
        .route("/public/images/{name}", get(images))
}

async fn handler_schedule() -> impl IntoResponse {
    println!("--> schedule_handler -");

    let ppls = vec![
        People {
            name: String::from("Nora"),
            status: false,
        },
        People {
            name: String::from("Paul"),
            status: false,
        },
        People {
            name: String::from("Logan"),
            status: false,
        },
        People {
            name: String::from("Cisco"),
            status: false,
        },
        People {
            name: String::from("Seve"),
            status: false,
        },
        People {
            name: String::from("Brown"),
            status: false,
        },
        People {
            name: String::from("Simmons"),
            status: false,
        },
        People {
            name: String::from("Garret"),
            status: false,
        },
        People {
            name: String::from("Dave"),
            status: false,
        },
    ];
    let format = maud! {
        link rel="stylesheet" type="text/css" href=("style/schedule.css"); // class is . , id is #
        h1 #remag {
            "Remag"
            div .ppl {
                @for ppl in ppls.iter() {
                    div .person {
                        p #name {(ppl.name)}
                        p #status {(ppl.status)}
                    }
                }

            }
        }
    }
    .render();
    Html(format.into_inner())
}

async fn handler_index() -> impl IntoResponse {
    println!("--> - handler_index -");

    let format = maud! {
        link rel="stylesheet" href=("sytle/schedule.css");
        div {
            h1 { "Paul's Site" }
            ul {
                li { a href="/server" {"Go To Server"} }
                li { a href="/schedule" {"Go To Schedule"} }
            }
        }
    }
    .render();
    Html(format.into_inner())
}

async fn handler_server() -> impl IntoResponse {
    println!("--> - handler_server -");

    let servers = vec![Server::Bedrock, Server::Java];

    let format = maud! {
        link rel="stylesheet" href=("style/server.css");
        script src="scripts/server.js" {}
        div .main {
            @for server in servers.iter() {
                @let dis = server.display();
                @if server.is_online() {
                    div .active {
                        h1 { (dis) }
                        div {
                            img src="/public/images/minecraft.png" {}
                        }
                        img src="https://www.svgrepo.com/show/405751/green-circle.svg" width="15" {}
                        span { " Online" }
                        div {
                            button id=(dis) value=1 { "Deactivate" }
                        }
                    }
                } @else {
                    div .inactive {
                        h1 { (dis) }
                        // img B&W
                        div {
                            img src="/public/images/minecraft.png" {}
                        }

                        img src="https://www.svgrepo.com/show/407314/red-circle.svg" width="15" {}
                        span { " Offline" }
                        div {
                            button id=(dis) value=0 { "Activate" }
                        }
                    }
                }
            }
        }
    }
    .render();
    Html(format.into_inner())
}

#[derive(Debug, Deserialize)]
struct ServerRequest {
    server: String,
    action: String,
}

async fn handler_server_put(payload: Json<ServerRequest>) -> impl IntoResponse {
    let server = &payload.server;
    let action = &payload.action;

    let s = match server.as_str() {
        "bedrock" => Server::Bedrock,
        "java" => Server::Java,
        _ => Server::Other,
    };

    let a = match action.as_str() {
        "start" => Action::Start,
        "restart" => Action::Restart,
        "close" => Action::Close,
        _ => Action::Other,
    };

    let res = manipulate_server(s, a);
    println!("server: {:?}, Action: {:?} | {:?}", server, action, res);
}

async fn style_server() -> Css<&'static str> {
    let server_css: &str = include_str!("../public/css/server.css");
    Css(server_css)
}

async fn style_schedule() -> Css<&'static str> {
    let server_css: &str = include_str!("../public/css/schedule.css");
    Css(server_css)
}

async fn script_server() -> JavaScript<&'static str> {
    let server_script: &str = include_str!("../public/scripts/server.js");
    JavaScript(server_script)
}

async fn images(Path(p): Path<String>) -> impl IntoResponse {
    println!("-> image handler {:?}", p);
    let full_path = format!("public/images/{}", p);
    println!("full path: {}", full_path);
    let path = std::path::Path::new(&full_path);
    let file = match tokio::fs::read(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime.as_ref())
        .body(Body::from(file))
        .unwrap())
}

struct People {
    name: String,
    status: bool,
}
