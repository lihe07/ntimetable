use std::{collections::HashMap, hash::Hash};

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        println!("Fatal: {msg}");
        std::process::exit(1);
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

pub fn int_encode<T: Eq + Hash>(mut keys: Vec<T>) -> HashMap<T, usize> {
    let mut map = HashMap::new();
    keys.dedup();

    for (i, key) in keys.into_iter().enumerate() {
        map.insert(key, i);
    }

    map
}
