use std::sync::Arc;

use crate::project::Project;

use crate::optimize::Solution;

pub fn neighborhoods(s: Solution, project: Arc<Project>, tx: crossbeam::channel::Sender<Solution>) {
    for (t, e, r) in s.iter_all_shuffle() {
        let max_per_day = project.events.max_per_day(&e);
        let day_in = project.config.slots_to_day(t);
        let e_in_kind = project.events.kind(&e);

        for day in project.config.days() {
            if day != day_in {
                // Need to check max_per_day
                if s.same_kind_events(day, e_in_kind) >= max_per_day {
                    continue;
                }
            }

            for t2 in project.config.slots_of_day(day) {
                if t2 == t {
                    continue;
                }

                if s.event_can_not_fit_in(&e, &r, t2, &project) {
                    continue;
                }

                let mut xx = s.clone();
                xx.events_in_slot_mut(t).retain(|(e2, _)| *e2 != e);
                xx.events_in_slot_mut(t2).push((e, r));

                // Produces xx
                tx.send(xx).unwrap();
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_relocation() {
        let proj = crate::project::Project::parse("./demo");

        let sol = crate::initial::find_initial_solution(&proj, false).unwrap();

        dbg!(sol.len());
        let mut sol = crate::optimize::Solution::new(sol);
        sol.fill_counter(&proj);

        let (tx, rx) = crossbeam::channel::unbounded();
        let proj = Arc::new(proj);

        neighborhoods(sol, proj.clone(), tx);

        let solutions: Vec<Solution> = rx.iter().collect();

        dbg!(solutions.len());

        for s in solutions {
            s.is_valid(&proj).unwrap();
        }

        println!("Ok Everyting is vaild");
    }
}
