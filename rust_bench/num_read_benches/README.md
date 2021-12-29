```
U'w') { cargo +nightly bench
   Compiling num_read_benches v0.1.0 (/Users/rhysd/Develop/github.com/rhysd/misc/rust/num_read_benches)
    Finished bench [optimized] target(s) in 1.57s
     Running target/release/deps/num_read_benches-4ce2b292578c0890

running 32 tests
test tests::read_f32_enum                          ... bench:      19,471 ns/iter (+/- 1,441)
test tests::read_f32_from_bits                     ... bench:      16,020 ns/iter (+/- 7,977)
test tests::read_f32_from_bytes_copy_from_slice_be ... bench:     103,548 ns/iter (+/- 2,649)
test tests::read_f32_from_bytes_copy_from_slice_le ... bench:      96,487 ns/iter (+/- 6,664)
test tests::read_f32_from_bytes_for_loop_be        ... bench:      18,236 ns/iter (+/- 1,252)
test tests::read_f32_from_bytes_for_loop_le        ... bench:      18,144 ns/iter (+/- 370)
test tests::read_f32_from_bytes_try_into_be        ... bench:      18,179 ns/iter (+/- 1,080)
test tests::read_f32_from_bytes_try_into_le        ... bench:      16,851 ns/iter (+/- 551)

test tests::read_f64_enum                          ... bench:      20,887 ns/iter (+/- 2,118)
test tests::read_f64_from_bits                     ... bench:      28,557 ns/iter (+/- 1,673)
test tests::read_f64_from_bytes_copy_from_slice_be ... bench:      49,247 ns/iter (+/- 1,173)
test tests::read_f64_from_bytes_copy_from_slice_le ... bench:      46,724 ns/iter (+/- 3,802)
test tests::read_f64_from_bytes_for_loop_be        ... bench:      20,775 ns/iter (+/- 1,134)
test tests::read_f64_from_bytes_for_loop_le        ... bench:      18,187 ns/iter (+/- 2,736)
test tests::read_f64_from_bytes_try_into_be        ... bench:      20,757 ns/iter (+/- 1,857)
test tests::read_f64_from_bytes_try_into_le        ... bench:      18,106 ns/iter (+/- 806)

test tests::read_i32_enum                          ... bench:      15,546 ns/iter (+/- 1,477)
test tests::read_i32_from_bits                     ... bench:      11,884 ns/iter (+/- 898)
test tests::read_i32_from_bytes_copy_from_slice_be ... bench:      88,535 ns/iter (+/- 4,021)
test tests::read_i32_from_bytes_copy_from_slice_le ... bench:      85,608 ns/iter (+/- 1,873)
test tests::read_i32_from_bytes_for_loop_be        ... bench:      13,070 ns/iter (+/- 1,205)
test tests::read_i32_from_bytes_for_loop_le        ... bench:      12,935 ns/iter (+/- 393)
test tests::read_i32_from_bytes_try_into_be        ... bench:      13,068 ns/iter (+/- 1,208)
test tests::read_i32_from_bytes_try_into_le        ... bench:      12,920 ns/iter (+/- 461)

test tests::read_i64_enum                          ... bench:      15,843 ns/iter (+/- 12,399)
test tests::read_i64_from_bits                     ... bench:      23,359 ns/iter (+/- 596)
test tests::read_i64_from_bytes_copy_from_slice_be ... bench:      41,531 ns/iter (+/- 1,009)
test tests::read_i64_from_bytes_copy_from_slice_le ... bench:      41,312 ns/iter (+/- 2,454)
test tests::read_i64_from_bytes_for_loop_be        ... bench:      14,543 ns/iter (+/- 1,388)
test tests::read_i64_from_bytes_for_loop_le        ... bench:      13,029 ns/iter (+/- 747)
test tests::read_i64_from_bytes_try_into_be        ... bench:      14,324 ns/iter (+/- 564)
test tests::read_i64_from_bytes_try_into_le        ... bench:      12,912 ns/iter (+/- 756)

test result: ok. 0 passed; 0 failed; 0 ignored; 32 measured; 0 filtered out
```

with `rustc 1.44.0-nightly (f4c675c47 2020-03-19)`
