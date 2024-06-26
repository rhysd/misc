use criterion::{criterion_group, criterion_main, Criterion};
use std::cmp;
use std::fs;
use std::iter;

fn load() -> String {
    fs::read_to_string("test.txt").unwrap()
}

fn modify(s: &str, mut off: usize) -> String {
    let mut s = s.to_string();
    while off < s.len() && !s.is_char_boundary(off) {
        off += 1;
    }
    let c = s[off..].chars().next().unwrap();
    assert_ne!(c, '★');
    s.replace_range(off..off + c.len_utf8(), "★");
    s
}

fn by_utf8_index(prev: &str, now: &str) -> Option<usize> {
    prev.char_indices()
        .zip(now.chars())
        .find_map(|((idx, a), b)| (a != b).then_some(idx))
        .or_else(|| {
            let (a, b) = (prev.len(), now.len());
            (a != b).then_some(cmp::min(a, b))
        })
}

fn by_byte_index(prev: &str, now: &str) -> Option<usize> {
    prev.as_bytes()
        .iter()
        .copied()
        .enumerate()
        .zip(now.as_bytes().iter().copied())
        .find_map(|((mut i, a), b)| {
            if a != b {
                while !now.is_char_boundary(i) {
                    i -= 1;
                }
                Some(i)
            } else {
                None
            }
        })
        .or_else(|| {
            let (a, b) = (prev.len(), now.len());
            (a != b).then_some(cmp::min(a, b))
        })
}

fn by_byte_chunks<const N: usize>(prev_str: &str, now_str: &str) -> Option<usize> {
    let prev = prev_str.as_bytes();
    let now = now_str.as_bytes();

    let offset = iter::zip(prev.chunks_exact(N), now.chunks_exact(N))
        .take_while(|(x, y)| x == y)
        .count()
        * N;
    let mut index = offset
        + iter::zip(&prev[offset..], &now[offset..])
            .take_while(|(x, y)| x == y)
            .count();
    let min = cmp::min(prev.len(), now.len());
    if index == min {
        return (prev.len() != now.len()).then_some(min);
    }
    while !now_str.is_char_boundary(index) {
        index -= 1;
    }
    Some(index)
}

fn begin(c: &mut Criterion) {
    let prev = load();
    let modified = modify(&prev, 100);
    let expected = by_utf8_index(&prev, &modified);
    c.bench_function("begin::utf8", |b| {
        b.iter(|| {
            let actual = by_utf8_index(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("begin::byte", |b| {
        b.iter(|| {
            let actual = by_byte_index(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("begin::chunk_32_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<32>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("begin::chunk_64_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<64>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("begin::chunk_128_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<128>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("begin::chunk_256_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<256>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
}

fn middle(c: &mut Criterion) {
    let prev = load();
    let modified = modify(&prev, prev.len() / 2);
    let expected = by_utf8_index(&prev, &modified);
    c.bench_function("middle::utf8", |b| {
        b.iter(|| {
            let actual = by_utf8_index(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("middle::byte", |b| {
        b.iter(|| {
            let actual = by_byte_index(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("middle::chunk_32_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<32>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("middle::chunk_64_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<64>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("middle::chunk_128_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<128>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("middle::chunk_256_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<256>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
}

fn end(c: &mut Criterion) {
    let prev = load();
    let modified = modify(&prev, prev.len() - 100);
    let expected = by_utf8_index(&prev, &modified);
    c.bench_function("end::utf8", |b| {
        b.iter(|| {
            let actual = by_utf8_index(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("end::byte", |b| {
        b.iter(|| {
            let actual = by_byte_index(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("end::chunk_32_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<32>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("end::chunk_64_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<64>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("end::chunk_128_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<128>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
    c.bench_function("end::chunk_256_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<256>(&prev, &modified);
            assert_eq!(actual, expected);
        })
    });
}

fn unmodified(c: &mut Criterion) {
    let prev = load();
    let unmodified = prev.clone();
    c.bench_function("unmodified::utf8", |b| {
        b.iter(|| {
            let actual = by_utf8_index(&prev, &unmodified);
            assert_eq!(actual, None);
        })
    });
    c.bench_function("unmodified::byte", |b| {
        b.iter(|| {
            let actual = by_byte_index(&prev, &unmodified);
            assert_eq!(actual, None);
        })
    });
    c.bench_function("unmodified::chunk_32_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<32>(&prev, &unmodified);
            assert_eq!(actual, None);
        })
    });
    c.bench_function("unmodified::chunk_64_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<64>(&prev, &unmodified);
            assert_eq!(actual, None);
        })
    });
    c.bench_function("unmodified::chunk_128_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<128>(&prev, &unmodified);
            assert_eq!(actual, None);
        })
    });
    c.bench_function("unmodified::chunk_256_bytes", |b| {
        b.iter(|| {
            let actual = by_byte_chunks::<256>(&prev, &unmodified);
            assert_eq!(actual, None);
        })
    });
}

criterion_group!(benches, begin, middle, end, unmodified);
criterion_main!(benches);
