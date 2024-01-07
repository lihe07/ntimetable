use std::cmp::Ordering;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ntimetable::pareto::*;
use rand::{Rng, SeedableRng};

#[derive(Clone)]
struct MOSA(Vec<f32>);

impl CanDominate for MOSA {
    fn dominates(&self, other: &Self) -> bool {
        for (i, v) in other.0.iter().enumerate() {
            if *v > self.0[i] {
                return false;
            }
        }

        true
    }

    fn compare_first_element(&self, other: &Self) -> Ordering {
        self.0[0].partial_cmp(&other.0[0]).unwrap()
    }

    fn avg(&self) -> f32 {
        self.0.iter().sum::<f32>() / self.0.len() as f32
    }
}

fn bench_mosa(c: &mut Criterion) {
    let dims = 100;
    let size = 1000;

    let mut data = vec![];
    let mut rng = rand::rngs::StdRng::from_seed([0u8; 32]);

    for _ in 0..size {
        let mut v = vec![0.0; dims];
        for x in v.iter_mut() {
            *x = rng.gen::<f32>() * 10.0;
        }
        data.push(MOSA(v));
    }

    c.bench_function("kung_recursive_mosa", |b| {
        b.iter(|| {
            black_box(kung_recursive_mosa(data.clone(), 10.0));
        })
    });

    c.bench_function("kung_recursive", |b| {
        b.iter(|| {
            black_box(kung_recursive(data.clone()));
        })
    });

    c.bench_function("naive", |b| {
        b.iter(|| {
            black_box(naive(data.clone()));
        })
    });
}

criterion_group!(benches, bench_mosa);
criterion_main!(benches);
