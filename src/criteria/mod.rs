mod room_distance;

use std::{collections::HashMap, path::Path};

use serde::Deserialize;
use serde_json::value::RawValue;

use crate::{fatal, must_open, optimize::Solution, project::Project, warn};

trait Criterion {
    fn evaluate(&self, s: &Solution, project: &Project) -> f32;
}

pub struct Criteria(Vec<Box<dyn crate::criteria::Criterion>>);

impl Criteria {
    pub fn evaluate(&self, s: &Solution, proj: &Project) -> Vec<f32> {
        self.0.iter().map(|c| c.evaluate(&s, &proj)).collect()
    }
}

#[derive(Deserialize)]
struct RawCriteria(HashMap<String, Vec<Box<RawValue>>>);

pub fn parse_criteria<P: AsRef<Path>>(path: P) -> Criteria {
    let path = path.as_ref();
    let criteria_json = must_open!(path, "criteria.json");

    let criteria = serde_json::from_reader::<_, RawCriteria>(criteria_json);
    if let Err(e) = criteria {
        fatal!("Failed to parse events.json: {e}");
    }
    let criteria = criteria.unwrap().0;

    let mut boxed_criteria = vec![];
    for (k, v) in criteria.iter() {
        let parser = match k.as_str() {
            // "event_time_distance" =>
            "room_distance" => room_distance::parse,
            _ => {
                warn!("Invalid criterion type: {k}");
                continue;
            }
        };

        boxed_criteria.extend(v.iter().map(|r| parser(&r.to_string())));
    }

    Criteria(boxed_criteria)
}
