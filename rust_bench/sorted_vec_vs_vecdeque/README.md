Sorted `Vec` v.s. `VecDeque`
============================

How to run:

```sh
cargo bench
```

Result:

```
create::vec             time:   [321.67 µs 323.19 µs 324.80 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

create::vecdeque        time:   [321.98 µs 323.16 µs 324.68 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low mild
  7 (7.00%) high severe

Benchmarking insert::vec: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
insert::vec             time:   [1.2320 ms 1.2395 ms 1.2503 ms]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

insert::vecdeque        time:   [936.64 µs 940.59 µs 945.23 µs]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

search::vec             time:   [415.76 µs 417.16 µs 418.67 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

search::vecdeque        time:   [428.72 µs 431.43 µs 434.75 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

Benchmarking remove::vec: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.5s, enable flat sampling, or reduce sample count to 50.
remove::vec             time:   [1.2597 ms 1.3267 ms 1.4169 ms]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

remove::vecdeque        time:   [762.87 µs 767.42 µs 772.99 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
```
