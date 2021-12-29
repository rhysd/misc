use byteorder_test::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn bench(c: &mut Criterion) {
    c.bench_function("shift", |b| {
        b.iter(|| {
            let mut r = Regs1::default();
            for _ in 0..100 {
                let x = random::<u8>();
                r.set_b(x);
                assert_eq!(r.b(), x);
                r.set_c(x);
                assert_eq!(r.c(), x);
                r.set_d(x);
                assert_eq!(r.d(), x);
                r.set_e(x);
                assert_eq!(r.e(), x);
                r.set_h(x);
                assert_eq!(r.h(), x);
                r.set_l(x);
                assert_eq!(r.l(), x);
                r.set_a(x);
                assert_eq!(r.a(), x);
            }

            let x = random::<u16>();
            r.set_bc(x);
            r.set_de(x);
            r.set_hl(x);
            assert_eq!(r.bc(), x);
            assert_eq!(r.de(), x);
            assert_eq!(r.hl(), x);
        })
    });

    c.bench_function("byteorder", |b| {
        b.iter(|| {
            let mut r = Regs2::default();
            for _ in 0..100 {
                let x = random::<u8>();
                r.set_b(x);
                assert_eq!(r.b(), x);
                r.set_c(x);
                assert_eq!(r.c(), x);
                r.set_d(x);
                assert_eq!(r.d(), x);
                r.set_e(x);
                assert_eq!(r.e(), x);
                r.set_h(x);
                assert_eq!(r.h(), x);
                r.set_l(x);
                assert_eq!(r.l(), x);
                r.set_a(x);
                assert_eq!(r.a(), x);
            }

            let x = random::<u16>();
            r.set_bc(x);
            r.set_de(x);
            r.set_hl(x);
            assert_eq!(r.bc(), x);
            assert_eq!(r.de(), x);
            assert_eq!(r.hl(), x);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
