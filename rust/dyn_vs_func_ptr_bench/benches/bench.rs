use criterion::{criterion_group, criterion_main, Criterion};
use dyn_vs_func_ptr_bench::*;

fn bench(c: &mut Criterion) {
    let data = vec![0; 256];
    c.bench_function("base", |b| {
        b.iter(|| {
            let mut mem = MemoryBase { v: data.clone() };
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("dyn", |b| {
        b.iter(|| {
            let mut mem: Box<dyn Addressable> = Box::new(Memory1 { v: data.clone() });
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("dyn2", |b| {
        b.iter(|| {
            let mut mem = Memory3 {
                v: data.clone(),
                a: Box::new(Access),
            };
            for i in 0..=255 {
                mem.write(i as usize, i);
                let j = mem.read(i as usize);
                assert_eq!(i, j);
            }
        })
    });
    c.bench_function("fn", |b| {
        b.iter(|| {
            let mut mem = Memory2::new(data.clone());
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
