use std::collections::HashSet;

use serde::Deserialize;

use crate::{fatal, project::Event};

use super::{Criterion, CriterionT};

fn default_weight() -> f32 {
    1.0
}

#[derive(Debug, Deserialize)]
pub struct EventsDistance {
    #[serde(default = "default_weight")]
    weight: f32,
    events: Vec<String>,
    kind: String,

    #[serde(skip)]
    events_set: HashSet<Event>,
}

impl CriterionT for EventsDistance {
    fn init(&mut self, project: &crate::project::Project) {
        for e in self.events.iter() {
            self.events_set.extend(
                project
                    .events
                    .events_with_kind(project.events.kind_name_to_id(&e)),
            );
        }
    }

    fn evaluate(&self, s: &crate::optimize::Solution, project: &crate::project::Project) -> f32 {
        let mut score = 0.0;
        for d in project.config.days() {
            let mut last = None;

            for (t, e, _) in s.events_of_day(d, &project) {
                if self.events_set.contains(&e) {
                    if let Some((t_l, e_l)) = last {
                        if e_l != e {
                            score += ((t - t_l) as f32).powi(2);
                        }
                    }

                    last = Some((t, e))
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

pub struct EventsDistanceSameType {
    weight: f32,
    event: String,
    events: HashSet<Event>,
    kind: String,
}

impl CriterionT for EventsDistanceSameType {
    fn init(&mut self, project: &crate::project::Project) {
        self.events = project
            .events
            .events_with_kind(project.events.kind_name_to_id(&self.event));
    }

    fn evaluate(&self, s: &crate::optimize::Solution, project: &crate::project::Project) -> f32 {
        let mut score = 0.0;
        for d in project.config.days() {
            let mut last = None;

            for (t, e, _) in s.events_of_day(d, &project) {
                if self.events.contains(&e) {
                    if let Some(t_l) = last {
                        score += ((t - t_l) as f32).powi(2);
                    }

                    last = Some(t)
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
    if let Ok(e) = serde_json::from_str::<EventsDistance>(config) {
        let mut events = e.events;

        if events.len() == 0 {
            fatal!("Criterion events_distance need at least one event");
        }

        events.dedup();

        if events.len() == 1 {
            Criterion::EventsDistanceSameType(EventsDistanceSameType {
                weight: e.weight,
                event: events[0].clone(),
                events: HashSet::new(),
                kind: e.kind,
            })
        } else {
            Criterion::EventsDistance(EventsDistance {
                weight: e.weight,
                events,
                events_set: HashSet::new(),
                kind: e.kind,
            })
        }
    } else {
        fatal!("Failed to parse events_distance criterion")
    }
}
