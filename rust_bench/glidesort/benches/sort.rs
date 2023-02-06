use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use std::cmp::Reverse;
use std::iter::repeat_with;

const SEED: u64 = 2023_02_06_21_54;

fn random_ints(c: &mut Criterion) {
    for size in [10, 100, 10000] {
        c.bench_function(&format!("random::{size}::glidesort"), |b| {
            let mut rng = SmallRng::seed_from_u64(SEED);
            b.iter(move || {
                let mut input = repeat_with(|| rng.gen::<i32>())
                    .take(size)
                    .collect::<Vec<_>>();
                glidesort::sort(&mut input);
            });
        });
        c.bench_function(&format!("random::{size}::std_stable_sort"), |b| {
            let mut rng = SmallRng::seed_from_u64(SEED);
            b.iter(move || {
                let mut input = repeat_with(|| rng.gen::<i32>())
                    .take(size)
                    .collect::<Vec<_>>();
                input.sort();
            });
        });
        c.bench_function(&format!("random::{size}::std_unstable_sort"), |b| {
            let mut rng = SmallRng::seed_from_u64(SEED);
            b.iter(move || {
                let mut input = repeat_with(|| rng.gen::<i32>())
                    .take(size)
                    .collect::<Vec<_>>();
                input.sort_unstable();
            });
        });
    }
}

fn sorted_ints(c: &mut Criterion) {
    for size in [10, 100, 10000] {
        c.bench_function(&format!("sorted::{size}::glidesort"), |b| {
            let mut input = (1..).take(size).collect::<Vec<_>>();
            b.iter(move || {
                glidesort::sort(&mut input);
            });
        });
        c.bench_function(&format!("sorted::{size}::std_stable_sort"), |b| {
            let mut input = (1..).take(size).collect::<Vec<_>>();
            b.iter(move || {
                input.sort();
            });
        });
        c.bench_function(&format!("sorted::{size}::std_unstable_sort"), |b| {
            let mut input = (1..).take(size).collect::<Vec<_>>();
            b.iter(move || {
                input.sort_unstable();
            });
        });
    }
}

fn reversed_ints(c: &mut Criterion) {
    for size in [10, 100, 10000] {
        c.bench_function(&format!("reversed::{size}::glidesort"), |b| {
            let input = (1..=size).rev().take(size).collect::<Vec<_>>();
            b.iter(|| {
                let mut input = input.clone();
                glidesort::sort(&mut input);
            });
        });
        c.bench_function(&format!("reversed::{size}::std_stable_sort"), |b| {
            let input = (1..=size).rev().take(size).collect::<Vec<_>>();
            b.iter(|| {
                let mut input = input.clone();
                input.sort();
            });
        });
        c.bench_function(&format!("reversed::{size}::std_unstable_sort"), |b| {
            let input = (1..=size).rev().take(size).collect::<Vec<_>>();
            b.iter(|| {
                let mut input = input.clone();
                input.sort_unstable();
            });
        });
    }
}

fn random_pairs(c: &mut Criterion) {
    for size in [10, 100, 10000] {
        c.bench_function(&format!("pairs::{size}::glidesort"), |b| {
            let mut rng = SmallRng::seed_from_u64(SEED);
            b.iter(move || {
                let mut input = repeat_with(|| (rng.gen::<i32>(), rng.gen::<i32>()))
                    .take(size)
                    .collect::<Vec<_>>();
                glidesort::sort_by(&mut input, |(l, _), (r, _)| l.cmp(r));
                glidesort::sort_by(&mut input, |(_, l), (_, r)| l.cmp(r));
            });
        });
        c.bench_function(&format!("pairs::{size}::std_stable_sort"), |b| {
            let mut rng = SmallRng::seed_from_u64(SEED);
            b.iter(move || {
                let mut input = repeat_with(|| (rng.gen::<i32>(), rng.gen::<i32>()))
                    .take(size)
                    .collect::<Vec<_>>();
                input.sort_by(|(l, _), (r, _)| l.cmp(r));
                input.sort_by(|(_, l), (_, r)| l.cmp(r));
            });
        });
        c.bench_function(&format!("pairs::{size}::std_unstable_sort"), |b| {
            let mut rng = SmallRng::seed_from_u64(SEED);
            b.iter(move || {
                let mut input = repeat_with(|| (rng.gen::<i32>(), rng.gen::<i32>()))
                    .take(size)
                    .collect::<Vec<_>>();
                input.sort_unstable_by(|(l, _), (r, _)| l.cmp(r));
                input.sort_unstable_by(|(_, l), (_, r)| l.cmp(r));
            });
        });
    }
}

criterion_group!(
    benches,
    random_ints,
    sorted_ints,
    reversed_ints,
    random_pairs
);
criterion_main!(benches);
