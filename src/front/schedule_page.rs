use axum::Json;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use hypertext::prelude::*;
use hypertext::{Buffer, Renderable, maud};
use sled::Db;
use std::sync::Arc;

use crate::back::schedule::{Activity, Schedule};
use crate::database::{add_schedule, get_schedules_data};
use crate::front::CONFIG;
use crate::front::common;

async fn get_schedules(db: State<Arc<Db>>) -> impl IntoResponse {
    let schedule_base = get_schedules_data(db).await;
    let mut final_schedule: Vec<Schedule> = Vec::new();
    for schedule in schedule_base {
        let processed = bincode::decode_from_slice::<Schedule, _>(&schedule, CONFIG).unwrap();
        final_schedule.push(processed.0);
    }

    let ret = maud! {
        @for ppl in final_schedule.iter() {
            div .person {
                p .name {(ppl.name)}
                @if ppl.is_busy() {
                    p #status .available { "Busy" }
                } @else {
                    p #status .unavailable { "Available" }
                }
            }
        }
    }
    .render();
    Html(ret.into_inner())
}

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
                div .title {
                    h1 { "Schedule: " }
                    input type="text" id="name" class="input-field" placeholder="Name";
                }
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

struct BasicSchedule {
    name: String,
    act: Vec<BasicActivity>,
}

#[derive(Clone)]
struct BasicActivity {
    title: String,
    start_time: usize,
    end_time: usize,
    days: u8,
}

pub async fn post(db: Arc<Db>, payload: Json<BasicSchedule>) {
    println!("-->  - Schedule_post -");

    let mut sched = Schedule::new(payload.name.clone());
    let acts = payload.act.clone();

    for act in acts {
        let activity = Activity::new(act.title.clone(), act.start_time, act.end_time, act.days);
        sched.add_activity(activity);
    }

    let serialized_schedule = match bincode::encode_to_vec(sched, CONFIG) {
        Ok(x) => x,
        Err(_) => {
            println!("Failed to encode");
            return;
        }
    };

    add_schedule(State(db), serialized_schedule).await;
}
