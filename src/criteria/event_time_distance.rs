use std::collections::HashSet;

use serde::Deserialize;

use crate::{fatal, project::Event};

use super::{Criterion, CriterionT};

fn default_weight() -> f32 {
    1.0
}

#[derive(Debug, Deserialize)]
pub struct EventTimeDistance {
    #[serde(default = "default_weight")]
    weight: f32,
    event: String,
    time: usize,
    kind: String,

    #[serde(skip)]
    events_set: HashSet<Event>,
}

impl CriterionT for EventTimeDistance {
    fn init(&mut self, project: &crate::project::Project) {
        self.events_set.extend(
            project
                .events
                .events_with_kind(project.events.kind_name_to_id(&self.event)),
        );
    }

    fn evaluate(&self, s: &crate::optimize::Solution, project: &crate::project::Project) -> f32 {
        let mut score = 0.0;
        for d in project.config.days() {
            for (t, e, _) in s.events_of_day(d, project) {
                if self.events_set.contains(&e) {
                    score += (t as f32 - self.time as f32).powi(2);
                }
            }
        }

        if self.kind == "max" {
            -score / self.weight
        } else {
            score / self.weight
        }
    }
}

pub fn parse(config: &str) -> Criterion {
    if let Ok(e) = serde_json::from_str::<EventTimeDistance>(config) {
        Criterion::EventTimeDistance(e)
    } else {
        fatal!("Failed to parse event_time_distance criterion")
    }
}

mod test {
    use super::*;
}
