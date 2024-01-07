use std::{collections::HashMap, path::Path};

use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::{fatal, must_open, utils};

use super::rooms::{RoomKind, Rooms};

fn default_max_per_day() -> usize {
    1
}

#[derive(Debug, Clone, Deserialize)]
struct RawEvent {
    name: String,
    num_per_week: usize,
    #[serde(default = "default_max_per_day")]
    max_per_day: usize,
    room_kind: String,
}

pub struct Event(usize);

pub struct Events {
    names: Vec<String>,
    max_per_day: Vec<usize>,
    room_kind: Vec<RoomKind>,
}

pub fn parse_events<P: AsRef<Path>>(path: P, rooms: &Rooms) -> Events {
    let path = path.as_ref();
    let events_json = must_open!(path, "events.json");

    let events = serde_json::from_reader::<_, Vec<RawEvent>>(events_json);
    if let Err(e) = events {
        fatal!("Failed to parse events.json: {e}");
    }
    let events = events.unwrap();

    // Expand
    let mut expanded_events = vec![];
    for e in events {
        for _ in 0..e.num_per_week {
            expanded_events.push(e.clone());
        }
    }

    // Shuffle
    expanded_events.shuffle(&mut rand::thread_rng());

    let mut names = vec![];
    let mut max_per_day = vec![];
    for (i, e) in expanded_events.iter().enumerate() {
        names.push(e.name.clone());
        max_per_day.push(e.max_per_day);
    }

    todo!()
}
