use std::{ops::Range, path::Path};

use crate::{fatal, must_open};
// Code for config
use serde::Deserialize;

fn default_tabu_size() -> usize {
    20
}

fn default_initial_attempts() -> usize {
    3
}

#[derive(Debug, Deserialize)]
pub struct Config {
    slots_per_day: usize,
    days: usize,

    #[serde(skip)]
    num_slots: usize,

    #[serde(default = "default_tabu_size")]
    pub tabu_size: usize,

    pub initial_method: String,
    #[serde(default = "default_initial_attempts")]
    pub initial_attempts: usize,

    pub max_iter_initial: usize,
    pub max_iter: usize,
}

impl Config {
    pub fn iter_slots(&self) -> Range<usize> {
        0..self.num_slots
    }

    pub fn slots_to_day(&self, t: usize) -> usize {
        t / self.slots_per_day
    }

    pub fn slots_of_day(&self, day: usize) -> Range<usize> {
        let begin = day * self.slots_per_day;
        begin..(begin + self.slots_per_day)
    }

    pub fn slots_of_same_day(&self, t: usize) -> Range<usize> {
        let day = t / self.slots_per_day * self.slots_per_day;
        day..(day + self.slots_per_day)
    }

    pub fn days(&self) -> Range<usize> {
        0..self.days
    }
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> Config {
    let path = path.as_ref();
    let config_json = must_open!(path, "config.json");

    match serde_json::from_reader::<_, Config>(config_json) {
        Ok(mut c) => {
            c.num_slots = c.days * c.slots_per_day;
            c
        }
        Err(e) => fatal!("Failed to parse config.json: {e}"),
    }
}

mod test {
    use super::*;

    #[test]
    fn test_basic_arithmetics() {
        let c = Config {
            slots_per_day: 10,
            days: 5,
            max_iter_initial: 0,
            max_iter: 0,
            num_slots: 0,
            tabu_size: 0,
            initial_attempts: 0,
            initial_method: String::new(),
        };
        assert!(c.slots_of_day(1).contains(&10));
        assert!(c.slots_of_day(1).contains(&19));
        assert!(!c.slots_of_day(1).contains(&20));
    }
}
