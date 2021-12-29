use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memmap::Mmap;
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("std::fs::read", |b| {
        b.iter(|| {
            let v = fs::read("Cargo.lock").unwrap();
            let mut c = 0;
            for b in v.into_iter() {
                if b == b'a' {
                    c += 1;
                }
            }
            black_box(c);
        })
    });
    c.bench_function("std::io::Read::bytes", |b| {
        b.iter(|| {
            use std::io::Read;
            let f = fs::File::open("Cargo.lock").unwrap();
            let mut c = 0;
            for b in f.bytes().map(Result::unwrap) {
                if b == b'a' {
                    c += 1;
                }
            }
            black_box(c);
        })
    });
    c.bench_function("memmap", |b| {
        b.iter(|| {
            let f = fs::File::open("Cargo.lock").unwrap();
            let m = unsafe { Mmap::map(&f).unwrap() };
            let mut c = 0;
            for b in m.iter().copied() {
                if b == b'a' {
                    c += 1;
                }
            }
            black_box(c);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
