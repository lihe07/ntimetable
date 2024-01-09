use std::{collections::HashSet, process::exit, sync::Arc};

use itertools::Itertools;

use crate::{
    optimize::{Solution, TIMEMAP},
    project::{Event, Project, Room},
};

pub fn neighborhoods(s: Solution, project: &Project, tx: crossbeam::channel::Sender<Solution>) {
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
            let mut xx = s.clone();

            let mut to_be_rearranged = vec![];
            let mut to_be_rearranged_set = HashSet::new();

            let mut other_events: TIMEMAP = project.config.iter_slots().map(|_| vec![]).collect();

            for (t, e, r) in xx.events_of_day_drain(day, &project) {
                if attended.contains(&e) {
                    to_be_rearranged.push((t, r));
                    to_be_rearranged_set.insert(r);
                }
                other_events[t].push((e, r));
            }

            if to_be_rearranged_set.len() <= 2 {
                continue;
            }

            // Solve TSP problem
            let room_arrangement = crate::tsp::solve(to_be_rearranged.clone(), &project);

            // Precondition: no duplicate t
            for ((t_origin, _), (t, _)) in room_arrangement
                .into_iter()
                .zip(to_be_rearranged.into_iter())
            {
                xx.events_in_slot_mut(t)
                    .extend(other_events[t_origin].drain(..));
            }

            // Push other unrelated events
            for (t, v) in other_events.into_iter().enumerate() {
                xx.events_in_slot_mut(t).extend(v);
            }

            tx.send(xx).unwrap();
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_greedy_room() {
        let project = Project::parse("./demo");

        let solution = crate::initial::find_initial_solution(&project, false).unwrap();

        let mut solution = crate::optimize::Solution::new(solution);
        solution.fill_counter(&project);

        let (tx, rx) = crossbeam::channel::unbounded();

        neighborhoods(solution, &project, tx);

        let v: Vec<Solution> = rx.iter().collect();

        dbg!(v.len());

        for mut e in v {
            e.is_valid(&project).unwrap();
        }
    }
}
