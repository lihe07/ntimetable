use std::{collections::HashSet, path::Path};

use serde::Deserialize;

use crate::{fatal, must_open};

use super::events::{Event, Events};

#[derive(Debug, Deserialize)]
struct RawPerson {
    name: String,
    attend: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Person(pub usize);

#[derive(Debug)]
pub struct People {
    events_attended: Vec<HashSet<Event>>,
    names: Vec<String>,
}

impl People {
    // Get events attended by a person
    pub fn events_attended_by(&self, p: Person) -> &HashSet<Event> {
        &self.events_attended[p.0]
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }
}

pub fn parse_people<P: AsRef<Path>>(path: P, events: &Events) -> People {
    let path = path.as_ref();
    let people_json = must_open!(path, "people.json");

    let people = serde_json::from_reader::<_, Vec<RawPerson>>(people_json);
    if let Err(e) = people {
        fatal!("Failed to parse events.json: {e}");
    }
    let people = people.unwrap();

    let mut events_attended = vec![];
    let mut names = vec![];

    for p in people {
        names.push(p.name);

        let mut attended = HashSet::new();
        for k in p.attend {
            attended.extend(events.events_with_kind(events.kind_name_to_id(&k)))
        }

        events_attended.push(attended)
    }

    People {
        events_attended,
        names,
    }
}
