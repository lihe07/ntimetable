use crate::optimize::Solution;
use crate::project::Project;
use std::sync::mpsc::Sender;

pub fn room_only(s: Solution, project: &Project, tx: &Sender<Solution>) {
    let events = s.iter_all();

    for (i, (t1, e1, r1)) in events.iter().enumerate() {
        for (t2, e2, r2) in events[(i + 1)..].iter() {
            // If e1 can not use r2, drop
            if project.rooms.room_kind(r1) != project.rooms.room_kind(r2) {
                continue;
            }

            let mut xx = s.clone();

            // Swap two events
            xx.events_in_slot_mut(*t1).retain(|(e, _)| e != e1);
            xx.events_in_slot_mut(*t2).retain(|(e, _)| e != e2);

            // Check only for room conflict
            let mut flag = false;

            for (_, r) in xx.events_in_slot(*t1) {
                // Room conflict
                if *r2 == *r {
                    flag = true;
                    break;
                }
            }
            if flag {
                continue;
            }

            for (_, r) in xx.events_in_slot(*t2) {
                // Room conflict
                if *r1 == *r {
                    flag = true;
                    break;
                }
            }
            if flag {
                continue;
            }

            xx.events_in_slot_mut(*t1).push((*e1, *r2));
            xx.events_in_slot_mut(*t2).push((*e2, *r1));

            tx.send(xx).unwrap();
        }
    }
}

pub fn time_only(s: Solution, project: &Project, tx: &Sender<Solution>) {
    let events = s.iter_all();

    for (i, (t1, e1, r1)) in events.iter().enumerate() {
        let day_1 = project.config.slots_to_day(*t1);
        let max_per_day_1 = project.events.max_per_day(e1);
        let kind_1 = project.events.kind(e1);

        for (t2, e2, r2) in events[(i + 1)..].iter() {
            let day_2 = project.config.slots_to_day(*t2);
            let max_per_day_2 = project.events.max_per_day(e2);
            let kind_2 = project.events.kind(e2);

            if day_1 != day_2 && kind_1 != kind_2 {
                // Check for max_per_day constraint

                if s.same_kind_events(day_1, kind_2) >= max_per_day_2
                    || s.same_kind_events(day_2, kind_1) >= max_per_day_1
                {
                    continue;
                }
            }

            let mut xx = s.clone();

            // Swap two events
            xx.events_in_slot_mut(*t1).retain(|(e, _)| e != e1);
            xx.events_in_slot_mut(*t2).retain(|(e, _)| e != e2);

            if xx.event_can_not_fit_in(e1, r1, *t2, project) {
                continue;
            }
            if xx.event_can_not_fit_in(e2, r2, *t1, project) {
                continue;
            }

            xx.events_in_slot_mut(*t1).push((*e2, *r2));
            xx.events_in_slot_mut(*t2).push((*e1, *r1));

            tx.send(xx).unwrap();
        }
    }
}

pub fn time_and_room(s: Solution, project: &Project, tx: &Sender<Solution>) {
    let events = s.iter_all();

    for (i, (t1, e1, r1)) in events.iter().enumerate() {
        let day_1 = project.config.slots_to_day(*t1);
        let max_per_day_1 = project.events.max_per_day(e1);
        let kind_1 = project.events.kind(e1);

        for (t2, e2, r2) in events[(i + 1)..].iter() {
            // If e1 can not use r2, drop
            if project.rooms.room_kind(r1) != project.rooms.room_kind(r2) {
                continue;
            }

            let day_2 = project.config.slots_to_day(*t2);
            let max_per_day_2 = project.events.max_per_day(e2);
            let kind_2 = project.events.kind(e2);

            if day_1 != day_2 && kind_1 != kind_2 {
                // Check for max_per_day constraint

                if s.same_kind_events(day_1, kind_2) >= max_per_day_2
                    || s.same_kind_events(day_2, kind_1) >= max_per_day_1
                {
                    continue;
                }
            }

            let mut xx = s.clone();

            // Swap two events
            xx.events_in_slot_mut(*t1).retain(|(e, _)| e != e1);
            xx.events_in_slot_mut(*t2).retain(|(e, _)| e != e2);

            if xx.event_can_not_fit_in(e1, r2, *t2, &project) {
                continue;
            }
            if xx.event_can_not_fit_in(e2, r1, *t1, &project) {
                continue;
            }

            xx.events_in_slot_mut(*t1).push((*e2, *r1));
            xx.events_in_slot_mut(*t2).push((*e1, *r2));

            tx.send(xx).unwrap();
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_swap() {
        let proj = crate::project::Project::parse("./demo");

        let sol = crate::initial::find_initial_solution(&proj, false).unwrap();

        dbg!(sol.len());
        let mut sol = crate::optimize::Solution::new(sol);
        sol.fill_counter(&proj);

        let (tx, rx) = std::sync::mpsc::channel();

        time_and_room(sol.clone(), &proj, &tx);
        time_only(sol.clone(), &proj, &tx);
        room_only(sol, &proj, &tx);

        let solutions: Vec<Solution> = rx.iter().collect();

        dbg!(solutions.len());

        for mut s in solutions {
            s.is_valid(&proj).unwrap();
        }
    }
}
