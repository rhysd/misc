## How to run

```sh
cargo bench
```

## Environment

- Machine: iMac (Retina 5K, 27-inch, 2020)
- OS: macOS 11
- Rust: 1.67.0

## Benchmarks

- `random::N::method`: Sort random N integers
- `sorted::N::method`: Sort sorted N integers
- `reversed::N::method`: Sort reverse-sorted N integers
- `pairs::N::method`: Sort random N integer pairs by first element then by second element

Methods:

- `glidesort`: [glidesort](https://docs.rs/glidesort/latest/glidesort/)
- `std_stable_sort`: `[T]::sort()`
- `std_unstable_sort`: `[T]::sort_unstable()`

## Results

```
random::10::glidesort   time:   [97.694 ns 98.049 ns 98.501 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

random::10::std_stable_sort
                        time:   [123.57 ns 123.88 ns 124.26 ns]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe

random::10::std_unstable_sort
                        time:   [115.77 ns 116.23 ns 116.80 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

random::100::glidesort  time:   [1.2983 µs 1.3021 µs 1.3063 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

random::100::std_stable_sort
                        time:   [1.9200 µs 1.9265 µs 1.9335 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

random::100::std_unstable_sort
                        time:   [1.4759 µs 1.4835 µs 1.4933 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

random::10000::glidesort
                        time:   [181.88 µs 182.26 µs 182.70 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

random::10000::std_stable_sort
                        time:   [301.65 µs 302.25 µs 302.97 µs]
Found 15 outliers among 100 measurements (15.00%)
  10 (10.00%) high mild
  5 (5.00%) high severe

random::10000::std_unstable_sort
                        time:   [188.65 µs 189.59 µs 190.85 µs]
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  7 (7.00%) high severe

sorted::10::glidesort   time:   [24.020 ns 24.155 ns 24.305 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

sorted::10::std_stable_sort
                        time:   [6.0803 ns 6.0997 ns 6.1260 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

sorted::10::std_unstable_sort
                        time:   [5.6331 ns 5.6423 ns 5.6528 ns]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe

sorted::100::glidesort  time:   [59.632 ns 59.776 ns 59.931 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

sorted::100::std_stable_sort
                        time:   [176.27 ns 178.03 ns 179.95 ns]
Found 13 outliers among 100 measurements (13.00%)
  13 (13.00%) high severe

sorted::100::std_unstable_sort
                        time:   [48.319 ns 48.450 ns 48.593 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

sorted::10000::glidesort
                        time:   [4.2847 µs 4.3048 µs 4.3297 µs]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe

sorted::10000::std_stable_sort
                        time:   [4.3095 µs 4.3217 µs 4.3353 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

sorted::10000::std_unstable_sort
                        time:   [3.2039 µs 3.2128 µs 3.2226 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

reversed::10::glidesort time:   [75.782 ns 76.333 ns 77.023 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

reversed::10::std_stable_sort
                        time:   [73.790 ns 74.093 ns 74.419 ns]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

reversed::10::std_unstable_sort
                        time:   [73.703 ns 73.897 ns 74.104 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

reversed::100::glidesort
                        time:   [155.63 ns 156.79 ns 158.29 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

reversed::100::std_stable_sort
                        time:   [287.35 ns 289.36 ns 291.55 ns]
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe

reversed::100::std_unstable_sort
                        time:   [147.68 ns 148.21 ns 148.78 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

reversed::10000::glidesort
                        time:   [7.1194 µs 7.7161 µs 8.9946 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

reversed::10000::std_stable_sort
                        time:   [8.2370 µs 8.2577 µs 8.2809 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

reversed::10000::std_unstable_sort
                        time:   [5.9223 µs 5.9663 µs 6.0207 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

pairs::10::glidesort    time:   [160.61 ns 161.05 ns 161.54 ns]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

pairs::10::std_stable_sort
                        time:   [212.26 ns 212.88 ns 213.52 ns]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low severe
  5 (5.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe

pairs::10::std_unstable_sort
                        time:   [196.36 ns 197.51 ns 199.36 ns]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

pairs::100::glidesort   time:   [2.5910 µs 2.5969 µs 2.6038 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

pairs::100::std_stable_sort
                        time:   [3.8615 µs 3.8763 µs 3.8928 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

pairs::100::std_unstable_sort
                        time:   [3.0737 µs 3.0880 µs 3.1046 µs]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

pairs::10000::glidesort time:   [376.61 µs 377.83 µs 379.10 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

pairs::10000::std_stable_sort
                        time:   [630.17 µs 633.16 µs 637.19 µs]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

pairs::10000::std_unstable_sort
                        time:   [392.50 µs 394.14 µs 396.12 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

```
