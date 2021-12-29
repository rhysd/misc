use criterion::{criterion_group, criterion_main, Criterion};
use dyn_vs_func_ptr_bench::*;

fn bench(c: &mut Criterion) {
    let data = vec![0; 256];
    c.bench_function("no_inline", |b| {
        b.iter(|| {
            let mut mem = MemoryNoInline { v: data.clone() };
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("dyn", |b| {
        b.iter(|| {
            let mut mem: Box<dyn Addressable> = Box::new(MemoryDyn { v: data.clone() });
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("dyn-factory", |b| {
        b.iter(|| {
            let mut mem: Box<dyn Addressable> = new_addressable(data.clone());
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("dyn-no-state", |b| {
        b.iter(|| {
            let mut mem = MemoryDynNoState {
                v: data.clone(),
                a: Box::new(ReadWrite),
            };
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("dyn-no-state-factory", |b| {
        b.iter(|| {
            let mut mem = MemoryDynNoState::new(data.clone());
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("fn", |b| {
        b.iter(|| {
            let mut mem = MemoryFnPtr::new(data.clone());
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("inline", |b| {
        b.iter(|| {
            let mut mem = MemoryInline { v: data.clone() };
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
