// Code for initial solution

use std::{collections::VecDeque, io::Write, ops::Range};

use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    fatal,
    optimize::TIMEMAP,
    project::{Event, Project},
};

#[derive(Clone, Debug)]
struct InitialSolution {
    events: TIMEMAP,
    eject_list: Vec<Event>,
}

impl InitialSolution {
    /// Create a new solution
    /// timeslots is
    pub fn new(mut all_events: Vec<Event>, slots: Range<usize>) -> Self {
        let mut events = vec![];

        for _ in slots {
            events.push(vec![]);
        }

        all_events.shuffle(&mut thread_rng());

        InitialSolution {
            events,
            eject_list: all_events,
        }
    }

    pub fn unassigned(&self) -> usize {
        self.eject_list.len()
    }
}

impl PartialEq for InitialSolution {
    fn eq(&self, other: &Self) -> bool {
        self.events == other.events
    }
}

impl Eq for InitialSolution {}

impl std::hash::Hash for InitialSolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.events.hash(state);
    }
}

/// Gives local_best and original x
#[inline]
fn find_local_best(project: &Project, x: &mut InitialSolution) -> InitialSolution {
    let mut local_best = x.clone();

    let e_in = x.eject_list.pop().unwrap();
    let e_in_kind = project.events.kind(&e_in);
    let max_per_day = project.events.max_per_day(&e_in);

    let prod = project.config.iter_slots().cartesian_product(
        project
            .rooms
            .rooms_with_kind(&project.events.room_kind(&e_in)),
    );

    for (t, r_in) in prod {
        let mut xx = x.clone();
        let mut events = vec![];

        // Check for C1 and C3
        while let Some((e, r)) = xx.events[t].pop() {
            if r == r_in {
                // Room conflict
                xx.eject_list.push(e);
                continue;
            }

            if project.events.have_people_conflict(e_in, e) {
                // Attendee conflict
                xx.eject_list.push(e);
                continue;
            }

            events.push((e, r));
        }

        // Check for C4: Evenly Distribute
        let mut count = 0;
        for tt in project.config.slots_of_same_day(t) {
            let mut events = vec![];
            while let Some((e, r)) = xx.events[tt].pop() {
                if project.events.kind(&e) == e_in_kind {
                    count += 1;
                    if count >= max_per_day {
                        // Eject
                        xx.eject_list.push(e);
                        continue;
                    }
                }

                events.push((e, r));
            }
            xx.events[tt] = events;
        }

        // Every hard constraints satisfied
        events.push((e_in, r_in));

        xx.events[t] = events;

        // let mut lb = local_best.lock().unwrap();
        if xx.unassigned() <= local_best.unassigned() {
            local_best = xx;
        }
    }

    x.eject_list.push(e_in);
    local_best
}

pub fn find_initial_solution_tabu(project: &Project, verbose: bool) -> Option<TIMEMAP> {
    let mut x = InitialSolution::new(
        project.events.iter_all().collect(),
        project.config.iter_slots(),
    );
    let mut best = x.clone();

    // let mut tabu: FxHashSet<InitialSolution> = FxHashSet::default();
    let mut tabu = VecDeque::with_capacity(project.config.tabu_size);

    for _ in 0..project.config.max_iter_initial {
        // let mut local_best = Mutex::new(x.clone());

        // Shuffle is quite time expensive
        // prod.shuffle(&mut thread_rng());

        // prod.par_iter().for_each(|(t, r_in)| {
        //

        // let local_best = local_best.into_inner().unwrap();

        let local_best = find_local_best(&project, &mut x);

        if local_best.unassigned() < best.unassigned() {
            best = local_best.clone();
            if verbose {
                print!("{} ", best.unassigned());
            }
            std::io::stdout().flush().unwrap();
        }

        if best.unassigned() == 0 {
            break;
        }

        if !tabu.contains(&local_best) {
            x = local_best.clone();
            if tabu.len() == project.config.tabu_size {
                tabu.pop_front();
            }
            tabu.push_back(local_best);
        } else {
            // Asp criterion
        }
    }

    if verbose {
        println!();
    }

    if best.unassigned() == 0 {
        Some(best.events)
    } else {
        None
    }
}

pub fn find_initial_solution_constructive(project: &Project, verbose: bool) -> Option<TIMEMAP> {
    let mut x = InitialSolution::new(
        project.events.iter_all().collect(),
        project.config.iter_slots(),
    );
    let mut best = x.clone();

    for _ in 0..project.config.max_iter_initial {
        let local_best = find_local_best(&project, &mut x);

        if local_best.unassigned() < best.unassigned() {
            best = local_best.clone();
            x = local_best;
            if verbose {
                print!("{} ", best.unassigned());
            }
            std::io::stdout().flush().unwrap();
        }

        if best.unassigned() == 0 {
            break;
        }
    }

    if verbose {
        println!();
    }

    if best.unassigned() == 0 {
        Some(best.events)
    } else {
        None
    }
}

pub fn find_initial_solution(project: &Project, verbose: bool) -> Option<TIMEMAP> {
    let f = match project.config.initial_method.as_str() {
        "tabu" => find_initial_solution_tabu,
        "constructive" => find_initial_solution_constructive,
        _ => fatal!("Invalid initial method"),
    };

    for i in 0..project.config.initial_attempts {
        if let Some(s) = f(&project, verbose) {
            return Some(s);
        } else if verbose {
            println!(
                "Attempts {} / {} failed",
                i + 1,
                project.config.initial_attempts
            );
        }
    }
    None
}

mod test {
    use crate::optimize::Solution;

    use super::*;

    #[test]
    fn test_find_initial_solution_tabu() {
        let project = Project::parse("./demo");

        for _ in 0..5 {
            if let Some(s) = find_initial_solution_tabu(&project, true) {
                Solution::new(s).is_valid(&project).unwrap();
                return;
            }
        }

        panic!("find_initial_solution_tabu failed")
    }

    #[test]
    fn test_find_initial_solution_constructive() {
        let project = Project::parse("./demo");

        for _ in 0..5 {
            if let Some(s) = find_initial_solution_constructive(&project, true) {
                Solution::new(s).is_valid(&project).unwrap();
                return;
            }
        }

        panic!("find_initial_solution_constructive failed")
    }
}
