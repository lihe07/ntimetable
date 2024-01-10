mod events_distance;
mod room_distance;

use std::{collections::HashMap, path::Path};

use enum_dispatch::enum_dispatch;
use serde::Deserialize;
use serde_json::value::RawValue;

use crate::{fatal, must_open, optimize::Solution, project::Project, warn};

#[enum_dispatch]
enum Criterion {
    RoomDistance(room_distance::RoomDistance),
    EventsDistance(events_distance::EventsDistance),
    EventsDistanceSameType(events_distance::EventsDistanceSameType),
}

#[enum_dispatch(Criterion)]
trait CriterionT {
    fn evaluate(&self, s: &Solution, project: &Project) -> f32;

    fn init(&mut self, _project: &Project) {}
}

pub struct Criteria(Vec<Criterion>);

impl Criteria {
    pub fn evaluate(&self, s: &Solution, project: &Project) -> Vec<f32> {
        self.0.iter().map(|c| c.evaluate(&s, &project)).collect()
    }

    pub fn init(&mut self, project: &Project) {
        self.0.iter_mut().for_each(|c| c.init(&project));
    }
}

#[derive(Deserialize)]
struct RawCriteria(HashMap<String, Vec<Box<RawValue>>>);

pub fn parse_criteria<P: AsRef<Path>>(path: P, project: &Project) -> Criteria {
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
            "events_distance" => events_distance::parse,
            _ => {
                warn!("Invalid criterion type: {k}");
                continue;
            }
        };

        boxed_criteria.extend(v.iter().map(|r| parser(&r.to_string())));
    }

    let mut c = Criteria(boxed_criteria);
    c.init(&project);

    c
}
