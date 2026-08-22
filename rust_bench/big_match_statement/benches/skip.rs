use big_match_statement::{baseline, flatten, two_phase};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

const FIXTURES: &[(&str, &[u8])] = &[
    ("commands", include_bytes!("assets/commands.bin")),
    ("rust_coding", include_bytes!("assets/rust_coding.bin")),
    ("scroll", include_bytes!("assets/scroll.bin")),
    ("set_lines", include_bytes!("assets/set_lines.bin")),
    ("ui_init", include_bytes!("assets/ui_init.bin")),
    (
        "ui_init_400x100",
        include_bytes!("assets/ui_init_400x100.bin"),
    ),
];

fn benchmark(c: &mut Criterion) {
    for &(filename, payload) in FIXTURES {
        let mut group = c.benchmark_group(filename);
        group.throughput(Throughput::Bytes(payload.len() as u64));

        group.bench_function("baseline", |b| {
            b.iter(|| {
                let mut input = black_box(payload);
                while !input.is_empty() {
                    let skipped = baseline::skip_value(input).unwrap();
                    input = &input[skipped..];
                }
                black_box(input);
            });
        });

        group.bench_function("two_phase", |b| {
            b.iter(|| {
                let mut input = black_box(payload);
                while !input.is_empty() {
                    let skipped = two_phase::skip_value(input).unwrap();
                    input = &input[skipped..];
                }
                black_box(input);
            });
        });

        group.bench_function("flatten", |b| {
            b.iter(|| {
                let mut input = black_box(payload);
                while !input.is_empty() {
                    let skipped = flatten::skip_value(input).unwrap();
                    input = &input[skipped..];
                }
                black_box(input);
            });
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
