use std::path::Path;

use crate::{fatal, must_open};
// Code for config
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    session_per_day: usize,
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> Config {
    let path = path.as_ref();
    let config_json = must_open!(path, "config.json");

    match serde_json::from_reader(config_json) {
        Ok(c) => c,
        Err(e) => fatal!("Failed to parse config.json: {e}"),
    }
}
