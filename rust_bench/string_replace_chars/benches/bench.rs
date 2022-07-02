use criterion::{criterion_group, criterion_main, Criterion};
use string_replace_chars::*;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn bench(c: &mut Criterion) {
    c.bench_function("direct", |b| {
        b.iter(|| {
            let mut lorem = LOREM.to_string();
            replace_chars_direct(&mut lorem, ' ', '_');
            assert!(!lorem.contains(' '));
        })
    });

    c.bench_function("copy", |b| {
        b.iter(|| {
            let lorem = LOREM.to_string();
            let lorem = replace_chars_copied(&lorem, ' ', '_');
            assert!(!lorem.contains(' '));
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
