How to run this:

```
cargo +nightly bench 2> /dev/null
```

Results:

```
running 4 tests
test tests::buf_and_lock_output ... bench:   2,033,404 ns/iter (+/- 134,374)
test tests::buf_output          ... bench:   2,019,787 ns/iter (+/- 81,177)
test tests::lock_output         ... bench: 113,606,530 ns/iter (+/- 22,673,178)
test tests::raw_output          ... bench: 121,663,646 ns/iter (+/- 7,673,365)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```
