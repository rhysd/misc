use big_match_statement::{baseline, flatten, two_phase};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::{hint::black_box, io::ErrorKind};

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

fn split_payloads(mut fixture: &[u8]) -> Vec<&[u8]> {
    let mut payloads = vec![];
    while !fixture.is_empty() {
        let skipped = baseline::skip_value(fixture).unwrap();
        payloads.push(&fixture[..skipped]);
        fixture = &fixture[skipped..];
    }
    payloads
}

fn complete(c: &mut Criterion) {
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

fn incomplete(c: &mut Criterion) {
    let complete = FIXTURES
        .iter()
        .find_map(|&(name, payload)| (name == "scroll").then_some(payload))
        .unwrap();
    let payloads = split_payloads(complete);
    let mut group = c.benchmark_group("incomplete");
    let bytes = complete.len() - payloads.len() * 2;
    group.throughput(Throughput::Bytes(bytes as u64));

    group.bench_function("baseline", |b| {
        b.iter(|| {
            for payload in payloads.iter() {
                let input = black_box(&payload[..payload.len() - 2]);
                let error = baseline::skip_value(input).unwrap_err();
                assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
                black_box(error);
            }
        });
    });

    group.bench_function("two_phase", |b| {
        b.iter(|| {
            for payload in payloads.iter() {
                let input = black_box(&payload[..payload.len() - 2]);
                let error = two_phase::skip_value(input).unwrap_err();
                assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
                black_box(error);
            }
        });
    });

    group.bench_function("flatten", |b| {
        b.iter(|| {
            for payload in payloads.iter() {
                let input = black_box(&payload[..payload.len() - 2]);
                let error = flatten::skip_value(input).unwrap_err();
                assert_eq!(error.kind(), ErrorKind::UnexpectedEof);
                black_box(error);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, complete, incomplete);
criterion_main!(benches);
