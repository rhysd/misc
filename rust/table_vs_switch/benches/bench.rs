use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use table_vs_switch::Cpu;

const ITER: usize = 10000;

fn criterion_benchmark(c: &mut Criterion) {
    let cpu = {
        let mut rng = thread_rng();
        let mut insns = [0u8; 256];
        for i in 0..256 {
            insns[i] = rng.gen();
        }
        Cpu::new(&insns)
    };

    let (expected_pc, expected_a, expected_b) = {
        let mut cpu1 = cpu.clone();
        for _ in 0..ITER {
            cpu1.step_table();
        }
        let mut cpu2 = cpu.clone();
        for _ in 0..ITER {
            cpu2.step_switch();
        }
        assert_eq!(cpu1.pc, cpu2.pc);
        assert_eq!(cpu1.a, cpu2.a);
        assert_eq!(cpu1.b, cpu2.b);
        (cpu1.pc, cpu1.a, cpu1.b)
    };

    c.bench_function("table", |b| {
        b.iter(|| {
            let mut cpu = cpu.clone();
            for _ in 0..black_box(ITER) {
                cpu.step_table();
            }
            assert_eq!(cpu.pc, expected_pc);
            assert_eq!(cpu.a, expected_a);
            assert_eq!(cpu.b, expected_b);
        })
    });

    c.bench_function("switch", |b| {
        b.iter(|| {
            let mut cpu = cpu.clone();
            for _ in 0..black_box(ITER) {
                cpu.step_switch();
            }
            assert_eq!(cpu.pc, expected_pc);
            assert_eq!(cpu.a, expected_a);
            assert_eq!(cpu.b, expected_b);
        })
    });

    c.bench_function("switch_hi_lo", |b| {
        b.iter(|| {
            let mut cpu = cpu.clone();
            for _ in 0..black_box(ITER) {
                cpu.step_switch_hi_lo();
            }
            assert_eq!(cpu.pc, expected_pc);
            assert_eq!(cpu.a, expected_a);
            assert_eq!(cpu.b, expected_b);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
