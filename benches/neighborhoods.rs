use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam::channel::unbounded;
use ntimetable::{
    neighborhoods::{greedy_room, relocation, swap},
    project::Project,
};

macro_rules! bench_function {
    ($c:expr, $s:expr, $project:expr, $n:tt, $f:expr) => {
        $c.bench_function($n, |b| {
            b.iter(|| {
                let (tx, _rx) = unbounded();
                $f($s.clone(), &$project, tx);
            });
        });
    };
}

fn bench_neighborhoods(c: &mut Criterion) {
    let project = Project::parse("./demo");

    let s = ntimetable::initial::find_initial_solution(&project, false).unwrap();
    let mut s = ntimetable::optimize::Solution::new(s);
    s.fill_counter(&project);

    bench_function!(c, s, project, "nhd_relocation", relocation::neighborhoods);
    bench_function!(c, s, project, "nhd_swap_room_only", swap::room_only);
    bench_function!(c, s, project, "nhd_swap_time_only", swap::time_only);
    bench_function!(c, s, project, "nhd_swap_time_and_room", swap::time_and_room);
    bench_function!(c, s, project, "nhd_greedy_room", greedy_room::neighborhoods);
}

criterion_group!(neighborhoods, bench_neighborhoods);
criterion_main!(neighborhoods);
