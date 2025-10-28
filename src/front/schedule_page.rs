use axum::response::{Html, IntoResponse};

use hypertext::prelude::*;
use hypertext::{Buffer, Renderable, maud};

use crate::front::common;

pub async fn get() -> impl IntoResponse {
    println!("--> schedule_handler -");

    let mut buffer = Buffer::new();

    common::nav_bar("/schedule", &mut buffer).await;
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

struct People {
    name: String,
    status: bool,
}
