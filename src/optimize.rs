use std::{borrow::BorrowMut, collections::HashSet, io::Write, sync::mpsc::Sender};

use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use rayon::iter::{
    IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator,
};

use crate::{
    pareto::CanDominate,
    project::{Event, EventKind, Project, Room},
};

pub type TIMEMAP = Vec<Vec<(Event, Room)>>;

#[derive(Debug, Clone)]
pub struct Solution {
    events: TIMEMAP,
    counter: Vec<Vec<usize>>,
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.events.eq(&other.events)
    }
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

    pub fn inner(&self) -> &TIMEMAP {
        &self.events
    }

    pub fn into_inner(self) -> TIMEMAP {
        self.events
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
        self.events[t].borrow_mut()
    }

    pub fn events_of_day(&self, day: usize, project: &Project) -> Vec<(usize, Event, Room)> {
        let mut events: Vec<(usize, Event, Room)> = vec![];

        for t in project.config.slots_of_day(day) {
            // events.extend(self.events_in_slot_mut(t).iter().map(|(e, r)| (t, *e, *r)));
            events.extend(self.events[t].iter().map(|(e, r)| (t, *e, *r)));
        }

        events
    }

    pub fn events_of_day_drain(
        &mut self,
        day: usize,
        project: &Project,
    ) -> Vec<(usize, Event, Room)> {
        let mut events: Vec<(usize, Event, Room)> = vec![];

        for t in project.config.slots_of_day(day) {
            // events.extend(self.events_in_slot_mut(t).iter().map(|(e, r)| (t, *e, *r)));
            events.extend(self.events[t].drain(..).map(|(e, r)| (t, e, r)));
        }

        events
    }

    pub fn events_in_slot_drain(&mut self, t: usize) -> Vec<(Event, Room)> {
        self.events[t].drain(..).collect()
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

    pub fn is_valid(&mut self, project: &Project) -> Result<(), String> {
        self.fill_counter(project);
        let mut seen = HashSet::new();

        for (_, e, _) in self.iter_all() {
            if seen.contains(&e) {
                return Err(format!("Multiple event: {:?}", e));
            }
            seen.insert(e);
        }

        if seen.len() != project.events.len() {
            return Err(format!(
                "Some events missing: {}",
                project.events.len() - seen.len()
            ));
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

impl CanDominate for (Vec<f32>, Solution) {
    fn sum(&self) -> f32 {
        self.0.iter().sum()
    }

    fn compare_first_element(&self, other: &Self) -> std::cmp::Ordering {
        self.0[0].partial_cmp(&other.0[0]).unwrap()
    }

    fn dominates(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().zip(&other.0) {
            if b >= a {
                return false;
            }
        }
        true
    }
}

const NEIGHBORHOODS: [fn(Solution, &Project, &Sender<Solution>); 2] = [
    crate::neighborhoods::relocation::neighborhoods,
    crate::neighborhoods::greedy_room::neighborhoods,
];

pub fn optimize_solution(s: TIMEMAP, project: &'static Project) -> Vec<TIMEMAP> {
    let mut population = vec![Solution::new(s)];

    // let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    //
    let population_size = 20;
    let mut temp = 1000.0;
    for i in 0..500 {
        println!("Begin iter {i}");
        // let mut neighborhoods = vec![];

        let (tx, rx) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();

        population
            .par_iter_mut()
            .for_each(|s| s.fill_counter(&project));

        rayon::join(
            move || {
                population
                    .into_iter()
                    .cartesian_product(NEIGHBORHOODS)
                    .par_bridge()
                    .for_each_with(&tx, |tx, (s, f)| {
                        f(s, &project, tx);
                    });
                drop(tx);
            },
            move || {
                for s in rx.iter() {
                    let tx2 = tx2.clone();
                    rayon::spawn(move || {
                        tx2.send((project.criteria().evaluate(&s, &project), s))
                            .unwrap()
                    });
                }
            },
        );

        let neighborhoods: Vec<(Vec<f32>, Solution)> = rx2.into_iter().collect();

        dbg!(neighborhoods.len());

        let frontline = crate::pareto::random_mosa(neighborhoods, population_size, temp);

        let sums: Vec<f32> = frontline
            .par_iter()
            .map(|(c, _)| c.par_iter().sum())
            .collect();

        dbg!(sums);

        population = frontline.into_iter().map(|(_, s)| s).collect();
        temp *= 0.998;
    }

    // s.into_inner()
    population.into_iter().map(|e| e.into_inner()).collect()
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

        sol.events_of_day_drain(0, &proj);

        assert_eq!(sol.events_of_day_drain(0, &proj).len(), 0);
    }
}
