use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng};

use crate::project::{Event, EventKind, Project, Room};

pub type TIMEMAP = Vec<Vec<(Event, Room)>>;

#[derive(Clone)]
pub struct Solution {
    events: TIMEMAP,
    counter: Vec<Vec<usize>>,
}

impl Solution {
    pub fn new(events: TIMEMAP) -> Solution {
        Solution {
            events,
            counter: vec![],
        }
    }
    pub fn iter_all(&self) -> Vec<(usize, Event, Room)> {
        let mut events = vec![];

        for (i, v) in self.events.iter().enumerate() {
            for (e, r) in v {
                events.push((i, *e, *r));
            }
        }

        events
    }

    pub fn iter_all_shuffle(&self) -> Vec<(usize, Event, Room)> {
        let mut all = self.iter_all();

        all.shuffle(&mut thread_rng());

        all
    }

    // Fill counter using events
    pub fn fill_counter(&mut self, project: &Project) {
        self.counter.clear();
        for day in project.config.days() {
            let mut counter_per_type = vec![0; project.events.kinds_len()];

            for t in project.config.slots_of_day(day) {
                for (e, _) in &self.events[t] {
                    counter_per_type[project.events.kind(e).0] += 1;
                }
            }

            self.counter.push(counter_per_type);
        }
    }

    pub fn same_kind_events(&self, day: usize, kind: EventKind) -> usize {
        self.counter[day][kind.0]
    }

    pub fn events_in_slot(&self, t: usize) -> &Vec<(Event, Room)> {
        &self.events[t]
    }

    pub fn events_in_slot_mut(&mut self, t: usize) -> &mut Vec<(Event, Room)> {
        self.events[t].as_mut()
    }

    pub fn event_can_not_fit_in(&self, e: &Event, r: &Room, t: usize, project: &Project) -> bool {
        for (e2, r2) in self.events_in_slot(t) {
            // Room conflict
            if *r2 == *r {
                return true;
            }

            // People conflict
            if project.events.have_people_conflict(*e, *e2) {
                return true;
            }
        }

        false
    }

    pub fn is_valid(&self, project: &Project) -> Result<(), String> {
        let mut seen = HashSet::new();

        for (_, e, _) in self.iter_all() {
            if seen.contains(&e) {
                return Err(format!("Multiple event: {:?}", e));
            }
            seen.insert(e);
        }

        let mut event_kind_and_max_per_day = HashSet::new();

        for t in project.config.iter_slots() {
            for (e1, r1) in self.events_in_slot(t).iter() {
                event_kind_and_max_per_day
                    .insert((project.events.kind(e1), project.events.max_per_day(e1)));

                for (e2, r2) in self.events_in_slot(t).iter() {
                    if e1 == e2 {
                        continue;
                    }

                    if r1 == r2 {
                        // return false;
                        return Err(format!(
                            "Room conflict at {t}: {:?} {:?} for {:?}",
                            e1, e2, r1
                        ));
                    }

                    if project.events.have_people_conflict(*e1, *e2) {
                        return Err(format!("People conflict at {t}: {:?} {:?}", e1, e2));
                    }
                }
            }
        }

        for day in project.config.days() {
            for (k, m) in event_kind_and_max_per_day.iter() {
                let n = self.same_kind_events(day, *k);
                if n > *m {
                    return Err(format!("Excess num of {:?} on day {day}: {n} > {m}", k));
                }
            }
        }

        Ok(())
    }
}

pub fn optimize(initial: TIMEMAP) -> TIMEMAP {
    let s = Solution {
        events: initial,
        counter: vec![],
    };

    s.events
}

mod test {
    use super::*;

    #[test]
    fn test_counter() {
        let proj = Project::parse("./demo");

        let mut dummy_events = vec![];

        let test_kind = EventKind(1);

        let test_event = proj.events.events_with_kind(test_kind)[0];

        dummy_events.push(vec![(test_event, Room(0))]);
        dummy_events.push(vec![]);
        dummy_events.push(vec![]);
        dummy_events.push(vec![]);
        dummy_events.push(vec![(test_event, Room(0))]);
        dummy_events.push(vec![]);
        dummy_events.push(vec![(test_event, Room(0))]);

        for _ in 0..(proj.config.iter_slots().len() - dummy_events.len()) {
            dummy_events.push(vec![]);
        }

        let mut sol = Solution {
            events: dummy_events,
            counter: vec![],
        };

        sol.fill_counter(&proj);

        assert_eq!(sol.same_kind_events(0, test_kind), 3);
    }
}
