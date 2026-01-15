use criterion::{Criterion, criterion_group, criterion_main};
use std::fs;

fn zstd(c: &mut Criterion) {
    let bytes = fs::read("input.txt").unwrap();

    for level in 1..=19 {
        c.bench_function(&format!("encode::level::{level}"), |b| {
            b.iter(|| zstd::encode_all(bytes.as_slice(), level).unwrap());
        });
    }

    for level in 1..=19 {
        let encoded = zstd::encode_all(bytes.as_slice(), level).unwrap();
        c.bench_function(&format!("decode::level::{level}"), |b| {
            b.iter(|| zstd::decode_all(encoded.as_slice()).unwrap());
        });
    }

    c.bench_function("compress::bulk", |b| {
        b.iter(|| zstd::bulk::compress(&bytes, 11).unwrap());
    });
    c.bench_function("compress::stream", |b| {
        b.iter(|| zstd::encode_all(bytes.as_slice(), 11).unwrap());
    });

    {
        let encoded = zstd::bulk::compress(&bytes, 11).unwrap();
        c.bench_function("decompress::bulk", |b| {
            b.iter(|| zstd::bulk::decompress(&encoded, bytes.len()).unwrap());
        });
        c.bench_function("decompress::stream", |b| {
            b.iter(|| zstd::decode_all(encoded.as_slice()).unwrap());
        });
    }
}

criterion_group!(benches, zstd);
criterion_main!(benches);
