use std::collections::HashSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roaring::RoaringBitmap;

fn bench_hashset(c: &mut Criterion) {
    let mut set1: HashSet<u64> = HashSet::new();

    let mut set2: HashSet<u64> = HashSet::new();

    for i in 0..600 {
        set1.insert(i);
    }

    for i in 500..1000 {
        set2.insert(i);
    }

    c.bench_function("std_hashset_union", |b| {
        b.iter(|| black_box(set1.union(&set2)))
    });

    c.bench_function("std_hashset_intersection", |b| {
        b.iter(|| black_box(set1.intersection(&set2)))
    });
}

fn bench_bitmap(c: &mut Criterion) {
    let mut set1 = RoaringBitmap::new();

    let mut set2 = RoaringBitmap::new();

    for i in 0..600 {
        set1.insert(i);
    }

    for i in 500..1000 {
        set2.insert(i);
    }

    c.bench_function("roaring_bitmap_union", |b| {
        b.iter(|| black_box(&set1 & &set2))
    });

    c.bench_function("roaring_bitmap_intersection", |b| {
        b.iter(|| black_box(&set1 | &set2))
    });
}

criterion_group!(bitmap, bench_hashset, bench_bitmap);
criterion_main!(bitmap);
