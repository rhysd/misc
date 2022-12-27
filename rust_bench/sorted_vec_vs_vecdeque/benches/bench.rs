use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use sorted_vec_vs_vecdeque::*;
use std::iter::repeat_with;

fn create(c: &mut Criterion) {
    c.bench_function("create::vec", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        b.iter(|| {
            let v: SortedVec<i32> = repeat_with(|| rng.gen::<i32>()).take(10000).collect();
            assert!(v.iter().zip(v.iter().skip(1)).all(|(l, r)| l <= r));
        })
    });
    c.bench_function("create::vecdeque", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        b.iter(|| {
            let v: SortedVecDeque<i32> = repeat_with(|| rng.gen::<i32>()).take(10000).collect();
            assert!(v.iter().zip(v.iter().skip(1)).all(|(l, r)| l <= r));
        })
    });
}

fn insert(c: &mut Criterion) {
    c.bench_function("insert::vec", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        b.iter(|| {
            let mut v = SortedVec::<i32>::default();
            for _ in 0..10000 {
                v.insert(rng.gen::<i32>());
            }
            assert!(v.iter().zip(v.iter().skip(1)).all(|(l, r)| l <= r));
        })
    });
    c.bench_function("insert::vecdeque", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        b.iter(|| {
            let mut v = SortedVecDeque::<i32>::default();
            for _ in 0..10000 {
                v.insert(rng.gen::<i32>());
            }
            assert!(v.iter().zip(v.iter().skip(1)).all(|(l, r)| l <= r));
        })
    });
}

fn search(c: &mut Criterion) {
    c.bench_function("search::vec", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        let v: SortedVec<i32> = repeat_with(|| rng.gen_range(-256..=256))
            .take(10000)
            .collect();
        b.iter(|| {
            for _ in 0..10000 {
                let _ = black_box(v.binary_search(&rng.gen_range(-256..256)));
            }
        })
    });
    c.bench_function("search::vecdeque", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        let v: SortedVecDeque<i32> = repeat_with(|| rng.gen_range(-256..=256))
            .take(10000)
            .collect();
        b.iter(|| {
            for _ in 0..10000 {
                let _ = black_box(v.binary_search(&rng.gen_range(-256..256)));
            }
        })
    });
}

fn remove(c: &mut Criterion) {
    c.bench_function("remove::vec", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        let v: SortedVec<i32> = repeat_with(|| rng.gen_range(-256..=256))
            .take(10000)
            .collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..10000 {
                v.remove(&rng.gen_range(-256..256));
            }
        })
    });
    c.bench_function("remove::vecdeque", |b| {
        let mut rng = StdRng::seed_from_u64(2022_12_27_13_25_32);
        let v: SortedVecDeque<i32> = repeat_with(|| rng.gen_range(-256..=256))
            .take(10000)
            .collect();
        b.iter(|| {
            let mut v = v.clone();
            for _ in 0..10000 {
                v.remove(&rng.gen_range(-256..256));
            }
        })
    });
}

criterion_group!(benches, create, insert, search, remove);
criterion_main!(benches);
