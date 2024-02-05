// Code for rooms graph

use std::{
    collections::{HashMap, HashSet},
    io::Read,
    path::Path,
};

use crate::{fatal, must_open, utils};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Room(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct RoomKind(usize);

#[derive(Debug)]
pub struct Rooms {
    names: Vec<String>,
    adjacent: Vec<Vec<i32>>,
    kinds: Vec<RoomKind>,
    set_kinds: HashSet<RoomKind>,
    kind_name_to_id: HashMap<String, RoomKind>,
}

impl Rooms {
    pub fn distance(&self, a: &Room, b: &Room) -> i32 {
        self.adjacent[a.0][b.0]
    }

    /// Get the kind id of a room
    pub fn room_kind(&self, r: &Room) -> RoomKind {
        self.kinds[r.0]
    }

    pub fn iter_kinds(&self) -> impl Iterator<Item = &RoomKind> + Clone {
        self.set_kinds.iter()
    }

    pub fn room_name(&self, r: &Room) -> &str {
        &self.names[r.0]
    }

    pub fn rooms_with_kind(&self, kind: &RoomKind) -> Vec<Room> {
        let mut rooms = vec![];

        for (i, k) in self.kinds.iter().enumerate() {
            if k == kind {
                rooms.push(Room(i))
            }
        }

        rooms
    }

    pub fn kind_name_to_id(&self, name: &str) -> RoomKind {
        *self.kind_name_to_id.get(name).unwrap()
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn iter_all(&self) -> impl Iterator<Item = Room> + Clone {
        (0..self.len()).map(|e| Room(e))
    }
}

pub fn parse_rooms<P: AsRef<Path>>(path: P) -> Rooms {
    let path = path.as_ref();
    let rooms_json = must_open!(path, "rooms.json");

    let name_to_kind = serde_json::from_reader::<_, HashMap<String, String>>(rooms_json);

    if let Err(e) = name_to_kind {
        fatal!("Failed to parse rooms.json: {e}");
    }
    let name_to_kind = name_to_kind.unwrap();

    let mut rooms_adj = must_open!(path, "rooms_adj.csv");

    let mut csv = String::new();
    rooms_adj.read_to_string(&mut csv).unwrap();
    let mut mat = vec![];
    let mut names_row = vec![];
    let mut names_col = vec![];

    let mut i = 0;
    for line in csv.split("\n") {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let mut row = line
            .split(",")
            .map(|e| e.trim().to_string())
            .filter(|e| e.len() > 0)
            .collect();

        if i == 0 {
            names_row = row;
            i += 1;
            continue;
        }

        names_col.push(row[0].to_string());

        mat.push(
            row.split_off(1)
                .iter()
                .map(|e| {
                    if let Ok(e) = e.parse() {
                        e
                    } else {
                        fatal!("Failed to parse '{e}' as i32 in rooms_adj");
                    }
                })
                .collect(),
        );

        i += 1;
    }

    if names_row != names_col {
        fatal!("Adjacent matrix should be symmetric");
    }

    let kinds: Vec<String> = names_col
        .iter()
        .map(|e| {
            if let Some(e) = name_to_kind.get(e) {
                e.clone()
            } else {
                fatal!("Missing kind for room {e}");
            }
        })
        .collect();

    let kind_name_to_id = utils::int_encode(kinds.clone(), |e| RoomKind(e));

    let mut set_kinds = HashSet::new();

    set_kinds.extend(kind_name_to_id.values());

    Rooms {
        adjacent: mat,
        names: names_row,
        kinds: kinds
            .iter()
            .map(|e| *kind_name_to_id.get(e).unwrap())
            .collect(),
        set_kinds,
        kind_name_to_id,
    }
}

mod test {
    use super::*;

    #[test]
    fn test_parse_rooms() {
        let rooms = parse_rooms("./demo");
        dbg!(rooms);
    }
}
