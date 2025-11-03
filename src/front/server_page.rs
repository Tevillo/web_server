use axum::{
    Json,
    response::{Html, IntoResponse},
};

use hypertext::prelude::*;
use hypertext::{Buffer, Renderable, maud};

use serde::{Deserialize, Serialize};

use crate::back::server::{Action, Server, manipulate_server};
use crate::front::common;

pub async fn get_servers() -> impl IntoResponse {
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

pub async fn get() -> impl IntoResponse {
    println!("--> - server_get -");

    let mut buffer = Buffer::new();
    let servers = [
        Server::MinecraftBedrock,
        Server::MinecraftVanilla,
        Server::MinecraftAllTheMods,
    ];

    common::nav_bar("/server", &mut buffer).await;

    maud! {
        head {
            title { "Server Status" }
            link rel="stylesheet" href=("style/server.css");
            script src="scripts/server.js" {}
        }
        body {
            div #servers .main { // Location for initial servers
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
pub struct ServerRequest {
    server: String,
    action: String,
}

#[derive(Serialize)]
pub struct ServerResponse {
    error_msg: String,
    server: String,
    action: String,
}

pub async fn post(payload: Json<ServerRequest>) -> Json<ServerResponse> {
    println!("--> - server_post -");
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
