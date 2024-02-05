use std::collections::HashSet;

use itertools::Itertools;
use serde::Deserialize;

use crate::{
    fatal,
    optimize::Solution,
    project::{Event, Project},
};

use super::{Criterion, CriterionT};

fn default_weight() -> f32 {
    1.0
}

#[derive(Debug, Deserialize)]
pub struct RoomDistance {
    #[serde(default = "default_weight")]
    weight: f32,
}

impl CriterionT for RoomDistance {
    fn evaluate(&self, s: &Solution, project: &Project) -> f32 {
        let mut score = 0;

        for (person, room_kind) in project
            .people
            .iter_all()
            .cartesian_product(project.rooms.iter_kinds())
        {
            let events_with_room_kind = project.events.events_with_room_kind(*room_kind);

            let attended: HashSet<&Event> = project
                .people
                .events_attended_by(person)
                .intersection(&events_with_room_kind)
                .collect();

            for day in project.config.days() {
                let mut last_room = None;
                for (_, e, r) in s.events_of_day(day, project) {
                    if attended.contains(&e) {
                        // rooms.push((t, e, r));
                        // rooms_set.insert(r);
                        if let Some(last_r) = last_room {
                            score += project.rooms.distance(&last_r, &r)
                        }

                        last_room = Some(r)
                    }
                }
            }
        }

        -score as f32 * self.weight
    }
}

pub fn parse(config: &str) -> Criterion {
    // dbg!(config);
    if let Ok(e) = serde_json::from_str(config) {
        Criterion::RoomDistance(e)
    } else {
        fatal!("Failed to parse room_distance criterion")
    }
}

mod test {
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn test_criterion_room_distance() {
        // return;
        let project = Project::parse("./demo");

        let c = RoomDistance { weight: 1.0 };
        let s = crate::initial::find_initial_solution(&project, true);
        let s = Solution::new(s.unwrap());
        let original_score = c.evaluate(&s, &project);
        dbg!(original_score);

        let (tx, rx) = mpsc::channel();

        crate::neighborhoods::greedy_room::neighborhoods(s, &project, &tx);
        drop(tx);

        let v: Vec<Solution> = rx.iter().collect();

        dbg!(v.len());

        let mut min_score = original_score;
        for s in v.iter().take(10) {
            let score = c.evaluate(&s, &project);
            min_score = min_score.min(score);
        }

        dbg!(min_score);
        assert_ne!(min_score, original_score);
    }
}
