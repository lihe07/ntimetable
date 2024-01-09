use std::{fmt::Debug, path::Path};

mod config;
mod events;
mod people;
mod rooms;

pub use config::Config;
pub use events::{Event, EventKind, Events};
pub use people::{People, Person};
pub use rooms::{Room, RoomKind, Rooms};

pub struct Project {
    pub config: config::Config,
    pub rooms: rooms::Rooms,
    pub events: events::Events,
    pub people: people::People,
    pub criteria: crate::criteria::Criteria,
}

impl Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Project with {} events, {} people and {} rooms",
            self.events.len(),
            self.people.len(),
            self.rooms.len()
        )
    }
}

impl Project {
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let config = config::parse_config(&path);
        let rooms = rooms::parse_rooms(&path);
        let mut events = events::parse_events(&path, &rooms);
        let people = people::parse_people(&path, &events);
        events.fill_attendees(&people);

        Project {
            config,
            rooms,
            events,
            people,
            criteria: crate::criteria::parse_criteria(path),
        }
    }
}
