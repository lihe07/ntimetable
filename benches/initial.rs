use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_initial(c: &mut Criterion) {
    let project = ntimetable::project::Project::parse("./demo");
    c.bench_function("find_initial_solution_tabu", |b| {
        b.iter(|| {
            black_box(ntimetable::initial::find_initial_solution_tabu(
                &project, false,
            ))
        })
    });
    c.bench_function("find_initial_solution_constructive", |b| {
        b.iter(|| {
            black_box(ntimetable::initial::find_initial_solution_constructive(
                &project, false,
            ))
        })
    });
}

criterion_group!(initial, bench_initial);
criterion_main!(initial);
