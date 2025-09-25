use axum::{
    Router,
    extract::Query,
    response::{Html,IntoResponse},
    routing::get,
};

use axum_extra::response::Css;

use hypertext::{GlobalAttributes, Renderable, html_elements, maud};

use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
    let app = Router::new()
        .merge(route_schedule());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn route_schedule() -> Router {
    Router::new()
        .route("/", get(handler_index))
        .route("/schedule", get(handler_schedule))
        .route("/server", get(handler_server))
        .route("/style/server.css", get(style_server))
        .route("/style/schedule.css", get(style_schedule))
}

#[derive(Debug, Deserialize)]
struct ScheduleRequest {
    task: Option<String>,
}

// /scheuler?task=example
async fn handler_schedule(Query(params): Query<ScheduleRequest>) -> impl IntoResponse {
    println!("--> schedule_handler - {:?}", params);

    let ppls = vec![
        People { name: String::from("Nora"), status: false },
        People { name: String::from("Paul"), status: false },
        People { name: String::from("Logan"), status: false },
        People { name: String::from("Cisco"), status: false },
        People { name: String::from("Seve"), status: false },
        People { name: String::from("Brown"), status: false },
        People { name: String::from("Simmons"), status: false },
        People { name: String::from("Garret"), status: false },
        People { name: String::from("Dave"), status: false },
    ];
    let task = params.task.as_deref().unwrap_or("No task provided");
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
    }.render();
    Html(format.into_inner())
}


async fn handler_index() -> impl IntoResponse {
    println!("--> - handler_index -");

    let format = maud! {
        link rel="stylesheet" href=("css/schedule.css");
        div {
            h1 { "Index" }
            ul {
                li { "one" }
                li { "two" }
                li { "three" }
            }
        }
    }.render();
    Html(format.into_inner())
}

async fn handler_server() -> impl IntoResponse {
    println!("--> - handler_server -");

    let format = maud! {
        link rel="stylesheet" href=("/style.css");
        div {
            h1 { "Server" }
            ul {
                li { "Bedock" }
                li { "Java" }
            }
        }
        img src="hehe.jpg";
    }.render();
    Html(format.into_inner())
}

async fn style_server() -> Css<&'static str> {
    let server_css: &str = include_str!("../css/server.css");
    Css(server_css)
}

async fn style_schedule() -> Css<&'static str> {
    let server_css: &str = include_str!("../css/schedule.css");
    Css(server_css)
}

struct People {
    name: String,
    status: bool,
}