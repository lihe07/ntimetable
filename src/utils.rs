use std::{collections::HashMap, hash::Hash};

use crate::{optimize::TIMEMAP, project::Project};

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        use colored::Colorize;

        let msg = format!($($arg)*);
        println!("{} {msg}", "Fatal".color("red").bold());
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use colored::Colorize;

        let msg = format!($($arg)*);
        println!("{} {msg}", "Warn".color("yellow").bold());
    }};
}

#[macro_export]
macro_rules! must_open {
    ($root:tt, $name:tt) => {{
        let file = std::fs::File::open($root.join($name));

        if let Err(e) = file {
            crate::fatal!("Failed to open {}: {:?}", $name, e);
        }

        file.unwrap()
    }};
}

pub fn int_encode<K: Eq + Hash, V, F: Fn(usize) -> V>(
    mut keys: Vec<K>,
    wrapper: F,
) -> HashMap<K, V> {
    let mut map = HashMap::new();
    keys.dedup();

    for (i, key) in keys.into_iter().enumerate() {
        map.insert(key, wrapper(i));
    }

    map
}

pub fn make_table(x: &TIMEMAP, project: &Project, day: Option<usize>) -> comfy_table::Table {
    let mut table = comfy_table::Table::new();

    let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    if let Some(d) = day {
        table.set_header(&days[d..d + 1]);
    } else {
        table.set_header(days);
    }

    for i in 0..project.config.slots_per_day {
        let mut row = vec![];
        for j in project.config.days() {
            if let Some(d) = day {
                if j != d {
                    continue;
                }
            }

            let mut cell = String::new();
            for (e, r) in x[i + j * project.config.slots_per_day].iter() {
                // row.push(format!("{}: {} ({})", t, r, e));

                cell += format!(
                    "{} ({}) {}\n",
                    project.events.kind_name(e),
                    e.0,
                    project.rooms.room_name(r)
                )
                .as_str();
            }
            row.push(cell);
        }
        table.add_row(row);
    }

    table
}
