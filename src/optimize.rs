use std::{borrow::BorrowMut, collections::HashSet, sync::mpsc::Sender};

use itertools::Itertools;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};
use rayon::iter::{IntoParallelRefMutIterator, ParallelBridge, ParallelIterator};

use crate::{
    pareto::CanDominate,
    project::{Event, EventKind, Project, Room},
    log::now_ms
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

impl CanDominate for (Vec<f32>, usize, Solution) {
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

fn collect_neighborhoods(
    f: &fn(Solution, &Project, &Sender<Solution>),
    s: Solution,
    project: &Project,
) -> Vec<Solution> {
    let (tx, rx) = std::sync::mpsc::channel();
    f(s, project, &tx);
    drop(tx);
    rx.into_iter().collect()
}

const NEIGHBORHOODS: [fn(Solution, &Project, &Sender<Solution>); 5] = [
    crate::neighborhoods::relocation::neighborhoods,
    crate::neighborhoods::greedy_room::neighborhoods,
    crate::neighborhoods::swap::room_only,
    crate::neighborhoods::swap::time_only,
    crate::neighborhoods::swap::time_and_room,
];

fn softmax_inplace(x: &mut [f32]) {
    let mut sum = 0.0;
    for e in x.iter() {
        sum += e.exp();
    }
    for e in x.iter_mut() {
        *e = e.exp() / sum;
    }
}

fn avg_inplace(x: &mut [f32]) {
    let mut sum = 0.0;
    for e in x.iter() {
        sum += e;
    }
    for e in x.iter_mut() {
        *e = *e / sum;
    }
}



pub fn optimize_solution(s: TIMEMAP, project: &'static Project) -> Vec<TIMEMAP> {
    let mut population = vec![Solution::new(s)];

    let population_size = 20;
    let mut temp = 1000.0;

    let avg = 1.0 / NEIGHBORHOODS.len() as f32;

    let penalty_thres = project.config.penalty_threshold;
    let decay_constant = penalty_thres / project.config.penalty_factor;
    let complementary_factor = avg * decay_constant;

    let warmup = 0;

    let mut factored_weights = vec![avg; NEIGHBORHOODS.len()];
    let mut last_weights = factored_weights.clone();

    let expect_graded_num = project.config.expected_graded_num;

    let mut history = HashSet::new();
    let history_max_size = project.config.history_size;

    let (ctrlc_send, ctrlc_recv) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        ctrlc_send.send(()).unwrap();
    }).expect("Error setting Ctrl-C handler");

    for i in 0..500 {
        if ctrlc_recv.try_recv().is_ok() {
            break;
        }

        let t00 = now_ms();
        let (tx, rx) = std::sync::mpsc::channel();

        population
            .par_iter_mut()
            .for_each(|s| s.fill_counter(project));

        let t0 = now_ms();
        let neighborhoods: Vec<(Vec<f32>, usize, Solution)> = population
            .into_iter()
            .cartesian_product(NEIGHBORHOODS.iter().enumerate())
            .par_bridge()
            .flat_map(move |(s, (i, f))| {
                let n = collect_neighborhoods(f, s, project);
                let size = n.len() as f32;
                tx.send((i, size)).unwrap();
            
                n.into_iter()
                    .choose_multiple(
                        &mut thread_rng(),
                        (size * factored_weights[i]).floor() as usize,
                    )
                    .into_iter()
                    .map(|s| (i, s))
                    .collect::<Vec<(usize, Solution)>>()
            })
            .map(|(i, s)| {
                let score = project.criteria().evaluate(&s, project);
                (score, i, s)
            })
            .collect();
        let time_grading = now_ms() - t0;

        let mut neighborhood_sizes = vec![0.0; NEIGHBORHOODS.len()];
        for (i, size) in rx {
            neighborhood_sizes[i] += size;
        }

        let graded_num = neighborhoods.len();

        let t0 = now_ms();
        let frontline = crate::pareto::random_mosa(neighborhoods, population_size, temp);
        let time_mosa = now_ms() - t0;

        // Count scores for each neighborhoods
        let mut max_scores = vec![f32::MIN; project.criteria().len()];
        let mut sum_scores = vec![0.0f32; project.criteria().len()];

        let mut neighborhoods_scores = vec![1.0f32; NEIGHBORHOODS.len()];

        population = vec![];

        for (scores, source, solution) in frontline {
            // Fill max and sum scores
            for (i, score) in scores.iter().enumerate() {
                max_scores[i] = max_scores[i].max(*score);
                sum_scores[i] += score;
            }

            if history.contains(solution.inner()) {
                // neighborhoods_scores[source] += 0.1; // Repeat, but optimal
            } else {
                neighborhoods_scores[source] += 2.0; // New
                history.insert(solution.clone().into_inner());
            }

            population.push(solution);
        }


        let pop_size = population.len() as f32;
        let avg_scores: Vec<f32> = sum_scores
            .into_iter()
            .map(|s| s / pop_size)
            .collect();


        if history.len() > history_max_size {
            // Trim half, randomly
            let target_size = history_max_size / 2;
            println!(
                "Trim history {} > {} => {}",
                history.len(),
                history_max_size,
                target_size
            );
            let mut v = history.into_iter().collect::<Vec<_>>();
            v.shuffle(&mut thread_rng());
            history = v.into_iter().take(target_size).collect();
        }


        dbg!(&neighborhood_sizes, &neighborhoods_scores);

        let mut average_scores = neighborhoods_scores.iter().enumerate().map(|(i, s)| s / neighborhood_sizes[i]).collect::<Vec<_>>();
        avg_inplace(&mut average_scores);

        // Penalty
        
        let mut factored_scores = average_scores.iter().enumerate().map(|(i, s)| {
            if *s > avg && last_weights[i] < penalty_thres {
                (1.0 - decay_constant) * (*s) + complementary_factor
            } else {
                *s
            }
        }).collect::<Vec<_>>();

        last_weights = factored_scores.clone();
        avg_inplace(&mut factored_scores);

        factored_weights = factored_scores;
        if i > warmup {
            // Adjust weights
            let mut expected_num = 0.0;
            for (i, w) in factored_weights.iter().enumerate() {
                expected_num += neighborhood_sizes[i] * *w;
            }

            let factor = expect_graded_num as f32 / expected_num;
            for w in factored_weights.iter_mut() {
                *w *= factor;
            }
        }


        crate::log::step(crate::log::Step{
            i,
            weights: last_weights.clone(),
            average_scores: avg_scores.clone(),
            max_scores: max_scores.clone(),
            neighborhood_grading_time: time_grading,
            mosa_time: time_mosa,
graded: graded_num,
            temperature: temp,
            history_size: history.len(),
neighborhood_average: average_scores.clone(),
        });

        println!(
            "{i} in {}ms (NG: {time_grading}, MOSA: {time_mosa}). Avg: {:?}. Max: {:?}.\nS: {:?}. W: {:?} T: {}. G: {}. P: {}",
            now_ms() - t00,
            avg_scores,
            max_scores,
            average_scores, 
            last_weights,
            temp,
            graded_num,
            pop_size
        );

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

        let test_event = proj
            .events
            .events_with_kind(test_kind)
            .into_iter()
            .next()
            .unwrap();

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
