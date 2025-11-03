use axum::{Router, extract::State};
use sled::Db;
use sled::IVec;
use std::sync::Arc;

// async fn set_value(
//     State(db): State<Arc<Db>>,
//     Path((key, value)): Path<(String, String)>,
// ) -> impl IntoResponse {
//     println!("Setting value for key: {}", key);
//     db.insert(key.as_bytes(), value.as_bytes()).unwrap();
//     db.flush().unwrap();
//     format!("Set {key} = {value}")
// }

pub async fn get_schedules_data(State(db): State<Arc<Db>>) -> Vec<IVec> {
    let mut schedules = Vec::new();

    // Iterate over all keys starting with "person_"
    for result in db.scan_prefix(b"schedule_") {
        let (_key, value) = match result {
            Ok((key, value)) => (key, value),
            Err(e) => {
                eprintln!("Error reading key-value pair: {}", e);
                continue;
            }
        };
        schedules.push(value);
    }
    schedules
}

pub async fn add_schedule(State(db): State<Arc<Db>>, sched: Vec<u8>) {
    let id_bytes: Option<IVec> = db
        .update_and_fetch(b"num_schedules", increment)
        .unwrap_or(None);

    match id_bytes {
        Some(id) => {
            let id = u64::from_be_bytes(id.as_ref().try_into().unwrap());
            let key = format!("schedule_{}", id);
            let _ = db.insert(key.as_bytes(), sched.as_slice());
        }
        None => println!("No ID found"),
    }
}

fn increment(bytes: Option<&[u8]>) -> Option<Vec<u8>> {
    let number = match bytes {
        Some(bytes) => {
            let array: [u8; 8] = bytes.try_into().unwrap();
            let number = u64::from_be_bytes(array) + 1;
            number
        }
        None => 0,
    };
    Some(number.to_be_bytes().to_vec())
}
