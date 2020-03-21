### `i32`

```
U'w') { cargo +nightly bench i32
    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/benches-c430e2c632bf9f26

running 7 tests
test tests::i32_from_bits                          ... bench:      12,117 ns/iter (+/- 671)
test tests::read_i32_from_bytes_copy_from_slice_be ... bench:      89,886 ns/iter (+/- 4,013)
test tests::read_i32_from_bytes_copy_from_slice_le ... bench:      87,204 ns/iter (+/- 15,479)
test tests::read_i32_from_bytes_for_loop_be        ... bench:      13,203 ns/iter (+/- 356)
test tests::read_i32_from_bytes_for_loop_le        ... bench:      13,162 ns/iter (+/- 688)
test tests::read_i32_from_bytes_try_into_be        ... bench:      13,317 ns/iter (+/- 644)
test tests::read_i32_from_bytes_try_into_le        ... bench:      13,225 ns/iter (+/- 560)

test result: ok. 0 passed; 0 failed; 0 ignored; 7 measured; 21 filtered out
```

### `i64`

```
U'w') { cargo +nightly bench i64
    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/benches-c430e2c632bf9f26

running 7 tests
test tests::i64_from_bits                          ... bench:      23,376 ns/iter (+/- 1,820)
test tests::read_i64_from_bytes_copy_from_slice_be ... bench:      44,078 ns/iter (+/- 5,890)
test tests::read_i64_from_bytes_copy_from_slice_le ... bench:      43,721 ns/iter (+/- 1,722)
test tests::read_i64_from_bytes_for_loop_be        ... bench:      14,271 ns/iter (+/- 676)
test tests::read_i64_from_bytes_for_loop_le        ... bench:      13,102 ns/iter (+/- 948)
test tests::read_i64_from_bytes_try_into_be        ... bench:      14,350 ns/iter (+/- 773)
test tests::read_i64_from_bytes_try_into_le        ... bench:      12,956 ns/iter (+/- 439)

test result: ok. 0 passed; 0 failed; 0 ignored; 7 measured; 21 filtered out
```

### `f32`

```
U'w') { cargo +nightly bench f32
    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/benches-c430e2c632bf9f26

running 7 tests
test tests::f32_from_bits                          ... bench:      15,643 ns/iter (+/- 1,536)
test tests::read_f32_from_bytes_copy_from_slice_be ... bench:     103,651 ns/iter (+/- 8,657)
test tests::read_f32_from_bytes_copy_from_slice_le ... bench:      95,905 ns/iter (+/- 7,221)
test tests::read_f32_from_bytes_for_loop_be        ... bench:      18,162 ns/iter (+/- 517)
test tests::read_f32_from_bytes_for_loop_le        ... bench:      18,005 ns/iter (+/- 535)
test tests::read_f32_from_bytes_try_into_be        ... bench:      18,234 ns/iter (+/- 2,255)
test tests::read_f32_from_bytes_try_into_le        ... bench:      18,157 ns/iter (+/- 731)

test result: ok. 0 passed; 0 failed; 0 ignored; 7 measured; 21 filtered out
```

### `f64`

```
U'w') { cargo +nightly bench f64
    Finished bench [optimized] target(s) in 0.00s
     Running target/release/deps/benches-c430e2c632bf9f26

running 7 tests
test tests::f64_from_bits                          ... bench:      28,291 ns/iter (+/- 1,769)
test tests::read_f64_from_bytes_copy_from_slice_be ... bench:      51,799 ns/iter (+/- 2,892)
test tests::read_f64_from_bytes_copy_from_slice_le ... bench:      48,870 ns/iter (+/- 2,803)
test tests::read_f64_from_bytes_for_loop_be        ... bench:      20,649 ns/iter (+/- 5,145)
test tests::read_f64_from_bytes_for_loop_le        ... bench:      18,032 ns/iter (+/- 709)
test tests::read_f64_from_bytes_try_into_be        ... bench:      20,577 ns/iter (+/- 698)
test tests::read_f64_from_bytes_try_into_le        ... bench:      18,085 ns/iter (+/- 822)

test result: ok. 0 passed; 0 failed; 0 ignored; 7 measured; 21 filtered out
```

