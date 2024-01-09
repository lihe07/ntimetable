use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam::channel::unbounded;
use ntimetable::project::Project;

fn bench_neighborhoods(c: &mut Criterion) {
    let project = Project::parse("./demo");

    let s = ntimetable::initial::find_initial_solution(&project, false).unwrap();
    let s = ntimetable::optimize::Solution::new(s);

    c.bench_function("relocation", |b| {
        b.iter(|| {
            let (tx, _rx) = unbounded();
            ntimetable::neighborhoods::relocation::neighborhoods(s.clone(), &project, tx);
        });
    });

    c.bench_function("greedy_room", |b| {
        b.iter(|| {
            let (tx, _rx) = unbounded();
            ntimetable::neighborhoods::greedy_room::neighborhoods(s.clone(), &project, tx);
        });
    });
}

criterion_group!(neighborhoods, bench_neighborhoods);
criterion_main!(neighborhoods);
