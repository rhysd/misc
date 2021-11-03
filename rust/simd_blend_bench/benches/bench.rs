use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_blend_bench::*;

fn bench(c: &mut Criterion) {
    c.bench_function("scalar", |b| {
        b.iter(|| {
            for ratio in (0..=255).step_by(10) {
                for r in (0..=255).step_by(30) {
                    for g in (3..=255).step_by(30) {
                        for b in (6..=255).step_by(30) {
                            let c1 = (r, g, b);
                            for r in (0..=255).step_by(30) {
                                for g in (3..=255).step_by(30) {
                                    for b in (6..=255).step_by(30) {
                                        let c2 = (r, g, b);
                                        black_box(blend_scalar(c1, c2, ratio));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    });

    c.bench_function("simd", |b| {
        b.iter(|| {
            for ratio in (0..=255).step_by(10) {
                for r in (0..=255).step_by(30) {
                    for g in (3..=255).step_by(30) {
                        for b in (6..=255).step_by(30) {
                            let c1 = (r, g, b);
                            for r in (0..=255).step_by(30) {
                                for g in (3..=255).step_by(30) {
                                    for b in (6..=255).step_by(30) {
                                        let c2 = (r, g, b);
                                        black_box(blend_simd(c1, c2, ratio));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
