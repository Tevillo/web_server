use axum::{
    Json, Router,
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};

use axum_extra::response::{Css, JavaScript};
use axum_server::tls_rustls::RustlsConfig;

use hypertext::prelude::*;
use hypertext::{Buffer, Renderable, maud};

use serde::{Deserialize, Serialize};

mod back;

use crate::back::server::{Action, Server, manipulate_server};

#[tokio::main]
async fn main() {
    let app = Router::new().merge(route_base());
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

fn route_base() -> Router {
    Router::new()
        .route("/", get(handler_index))
        .route("/home", get(handler_index))
        .route("/schedule", get(handler_schedule))
        .route("/server", get(handler_server).post(handler_server_post))
        .route("/get_servers", post(get_servers))
        .route("/style/server.css", get(style_server))
        .route("/style/schedule.css", get(style_schedule))
        .route("/style/base.css", get(style_base))
        .route("/scripts/server.js", get(script_server))
        .route("/public/images/{name}", get(images))
}

async fn handler_schedule() -> impl IntoResponse {
    println!("--> schedule_handler -");

    let mut buffer = Buffer::new();

    nav_bar("/schedule", &mut buffer).await;
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
            status: true,
        },
        People {
            name: String::from("Brown"),
            status: true,
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
    maud! {
        link rel="stylesheet" type="text/css" href=("style/schedule.css"); // class is . , id is #
        body {
            div #servers .main {
                @for ppl in ppls.iter() {
                    div .person {
                        p .name {(ppl.name)}
                        @if ppl.status {
                            p #status .available { "Active" }
                        } @else {
                            p #status .unavailable { "Inactive" }
                        }
                    }
                }
            }
        }
    }
    .render_to(&mut buffer);
    Html(buffer.rendered().into_inner())
}

async fn handler_index() -> impl IntoResponse {
    println!("--> - handler_index -");

    let mut buffer = Buffer::new();

    nav_bar("/home", &mut buffer).await;

    Html(buffer.rendered().into_inner())
}

async fn nav_bar(page: &str, buffer: &mut Buffer) {
    let pages = ["/home", "/schedule", "/server"];
    maud! {
        link rel="stylesheet" href=("style/base.css");
        body {
            div .navbar {
                ul {
                    @for p in pages.iter() {
                        @let li = &p[1..p.len()];
                        @if li == page {
                            li .active { a href=(p) { (li) } }
                        } @else {
                            li { a href=(p) { (li) } }
                        }
                    }
                }
            }
        }
    }
    .render_to(buffer);
}

async fn get_servers() -> impl IntoResponse {
    let servers = [
        Server::MinecraftBedrock,
        Server::MinecraftVanilla,
        Server::MinecraftAllTheMods,
    ];
    let ret = maud! {
        @for server in servers.iter() {
            @let display = server.display();
            @let name = server.name();
            @let image = server.image();
            @let port = server.port();
            @if server.is_online() {
                div .active {
                    h1 { (name) }
                    div .display {
                        img src=(image);
                    }
                    img src="https://www.svgrepo.com/show/405751/green-circle.svg" width="15";
                    span { " Online At " (port) }
                    div {
                        button id=(display) value=1 { "Deactivate" }
                    }
                }
            } @else {
                div .inactive {
                    h1 { (name) }
                    div .display {
                        img src=(image);
                    }
                    img src="https://www.svgrepo.com/show/407314/red-circle.svg" width="15";
                    span { " Offline" }
                    div {
                        button id=(display) value=0 { "Activate" }
                    }
                }
            }
        }
    }
    .render();
    Html(ret.into_inner())
}

async fn handler_server() -> impl IntoResponse {
    println!("--> - handler_server -");

    let mut buffer = Buffer::new();

    nav_bar("/server", &mut buffer).await;

    maud! {
        head {
            title { "Server Status" }
            link rel="stylesheet" href=("style/server.css");
            script src="scripts/server.js" {}
        }
        body {
            div #servers .main { // Location for initial servers
            }
            div #popup .popup {
                h2 #server-status { "" }
                img #server-image src="";
                button #go-back { "Ok" }
            }
        }
    }
    .render_to(&mut buffer);
    Html(buffer.rendered().into_inner())
}

#[derive(Debug, Deserialize)]
struct ServerRequest {
    server: String,
    action: String,
}

#[derive(Serialize)]
struct ServerResponse {
    error_msg: String,
    server: String,
    action: String,
}

async fn handler_server_post(payload: Json<ServerRequest>) -> Json<ServerResponse> {
    let server = &payload.server;
    let action = &payload.action;

    let s = match server.as_str() {
        "mc_bedrock" => Server::MinecraftBedrock,
        "mc_java" => Server::MinecraftVanilla,
        "mc_all_the_mods" => Server::MinecraftAllTheMods,
        _ => Server::Other,
    };

    let a = match action.as_str() {
        "start" => Action::Start,
        "close" => Action::Close,
        _ => Action::Other,
    };

    let res = manipulate_server(s, a);
    println!("server: {:?}, Action: {:?} | {:?}", server, action, res);
    match res {
        Ok(_) => Json(ServerResponse {
            error_msg: "".to_string(),
            action: action.clone(),
            server: server.clone(),
        }),
        Err(e) => Json(ServerResponse {
            error_msg: e.display(),
            server: server.clone(),
            action: action.clone(),
        }),
    }
}

async fn style_server() -> Css<&'static str> {
    let css: &str = include_str!("../public/css/server.css");
    Css(css)
}

async fn style_schedule() -> Css<&'static str> {
    let css: &str = include_str!("../public/css/schedule.css");
    Css(css)
}

async fn style_base() -> Css<&'static str> {
    let css: &str = include_str!("../public/css/base.css");
    Css(css)
}

async fn script_server() -> JavaScript<&'static str> {
    let server_script: &str = include_str!("../public/scripts/server.js");
    JavaScript(server_script)
}

async fn images(Path(p): Path<String>) -> impl IntoResponse {
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

struct People {
    name: String,
    status: bool,
}
