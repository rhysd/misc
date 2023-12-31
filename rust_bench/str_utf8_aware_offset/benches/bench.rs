use criterion::{criterion_group, criterion_main, Criterion};
use std::cmp;
use std::fs;

fn load() -> String {
    fs::read_to_string("rust-sso.md").unwrap()
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
}

criterion_group!(benches, begin, middle, end, unmodified);
criterion_main!(benches);
