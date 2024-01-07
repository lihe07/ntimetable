use std::path::Path;

mod config;
mod events;
mod rooms;

pub struct Project {
    config: config::Config,
    rooms: rooms::Rooms,
    events: events::Events,
}

impl Project {
    pub fn parse<P: AsRef<Path>>(path: P) -> Self {
        let config = config::parse_config(path);
        let rooms = rooms::parse_rooms(path);
        let events = events::parse_events(path, &rooms);

        Project {
            config,
            rooms,
            events,
        }
    }
}
