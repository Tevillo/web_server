use axum::response::{Html, IntoResponse};

use hypertext::prelude::*;
use hypertext::{Buffer, Renderable, maud};

use crate::front::common;

pub async fn get() -> impl IntoResponse {
    println!("--> schedule_handler -");

    let mut buffer = Buffer::new();

    common::nav_bar("/schedule", &mut buffer).await;
    let ppls = vec![People {
        name: String::from("Nora"),
        status: false,
    }];
    maud! {
        link rel="stylesheet" type="text/css" href=("style/schedule.css"); // class is . , id is #
        script src="scripts/schedule.js" {}
        body {
            button id="add_person" { "Add A Schedule" }
            div #popup .popup {
                h1 .main { "Schedule Maker" }
                div #item_container .item_container { }
                div .buttons {
                    button id="add_item" { "Add Item" }
                    button onclick="submitForm()" { "Submit" }
                }
            }
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
