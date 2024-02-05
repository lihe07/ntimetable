use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::{fatal, must_open, utils};

use super::{
    people::{People, Person},
    rooms::{RoomKind, Rooms},
};

fn default_max_per_day() -> usize {
    2
}

#[derive(Debug, Clone, Deserialize)]
struct RawEvent {
    name: String,
    num_per_week: usize,
    #[serde(default = "default_max_per_day")]
    max_per_day: usize,
    room_kind: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Event(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct EventKind(pub usize);

#[derive(Debug)]
pub struct Events {
    kinds: Vec<EventKind>,
    max_per_day: Vec<usize>,
    room_kind: Vec<RoomKind>,
    kind_name_to_id: HashMap<String, EventKind>,
    kind_id_to_name: HashMap<EventKind, String>,

    /// This field is updated lazily
    attendees: Vec<HashSet<Person>>,
}

impl Events {
    pub fn kind_name_to_id(&self, name: &str) -> EventKind {
        if let Some(e) = self.kind_name_to_id.get(name) {
            *e
        } else {
            fatal!("Invalid event: {name}");
        }
    }

    pub fn events_with_kind(&self, kind: EventKind) -> HashSet<Event> {
        let mut events = HashSet::new();
        for (i, k) in self.kinds.iter().enumerate() {
            if *k == kind {
                events.insert(Event(i));
            }
        }

        events
    }

    pub fn events_with_room_kind(&self, kind: RoomKind) -> HashSet<Event> {
        let mut events = HashSet::new();
        for (i, k) in self.room_kind.iter().enumerate() {
            if *k == kind {
                events.insert(Event(i));
            }
        }

        events
    }

    pub fn room_kind(&self, event: &Event) -> RoomKind {
        self.room_kind[event.0]
    }

    pub fn kind(&self, event: &Event) -> EventKind {
        self.kinds[event.0]
    }

    pub fn kind_name(&self, event: &Event) -> &str {
        self.kind_id_to_name.get(&self.kind(event)).unwrap()
    }

    pub fn max_per_day(&self, event: &Event) -> usize {
        self.max_per_day[event.0]
    }

    pub fn kinds_len(&self) -> usize {
        self.kinds.len()
    }

    pub fn len(&self) -> usize {
        self.max_per_day.len()
    }

    pub fn iter_all(&self) -> impl Iterator<Item = Event> + Clone {
        (0..self.len()).map(|e| Event(e))
    }

    pub fn fill_attendees(&mut self, people: &People) {
        if !self.attendees.is_empty() {
            println!("Warn: Events attendees might already been filled!");
            self.attendees.clear();
        }

        for _ in 0..self.len() {
            self.attendees.push(HashSet::new());
        }

        for i in 0..people.len() {
            for e in people.events_attended_by(Person(i)) {
                self.attendees[e.0].insert(Person(i));
            }
        }
    }

    pub fn event_attendees(&self, e: Event) -> &HashSet<Person> {
        &self.attendees[e.0]
    }

    pub fn have_people_conflict(&self, e1: Event, e2: Event) -> bool {
        self.event_attendees(e1)
            .intersection(self.event_attendees(e2))
            .count()
            > 0
    }
}

pub fn parse_events<P: AsRef<Path>>(path: P, rooms: &Rooms) -> Events {
    let path = path.as_ref();
    let events_json = must_open!(path, "events.json");

    let events = serde_json::from_reader::<_, Vec<RawEvent>>(events_json);
    if let Err(e) = events {
        fatal!("Failed to parse events.json: {e}");
    }
    let events = events.unwrap();
    let kind_name_to_id = utils::int_encode(events.iter().map(|e| e.name.clone()).collect(), |e| {
        EventKind(e)
    });

    let kind_id_to_name: HashMap<EventKind, String> = kind_name_to_id
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();

    // Expand
    let mut expanded_events = vec![];
    for e in events {
        for _ in 0..e.num_per_week {
            expanded_events.push(e.clone());
        }
    }

    // Shuffle
    expanded_events.shuffle(&mut rand::thread_rng());

    let mut max_per_day = vec![];
    let mut room_kind = vec![];
    let mut kinds = vec![];
    for e in expanded_events {
        kinds.push(kind_name_to_id.get(&e.name).unwrap().clone());
        max_per_day.push(e.max_per_day);
        room_kind.push(rooms.kind_name_to_id(&e.room_kind));
    }

    Events {
        kinds,
        max_per_day,
        room_kind,
        kind_name_to_id,
        kind_id_to_name,
        attendees: vec![],
    }
}
