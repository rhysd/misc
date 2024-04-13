UTF-8 index v.s. byte index on finding modified byte offset
===========================================================

Problem: Calculate a byte offset of the first modified position in two given strings taking care of UTF-8 character boundaries.

- `utf8` : Iterate two strings in UTF-8 character-wise
- `byte` : Iterate two strings in byte-wise then find the nearest character-wise offset
- `chunk_64bytes` : Iterate two strings per 64 bytes chunks and find the first mismatch in the chunk, then find the nearest character-wise offset
- `chunk_128bytes` : Iterate two strings per 128 bytes chunks and find the first mismatch in the chunk, then find the nearest character-wise offset

How to run:

1. Prepare an input file as `test.txt` at the root of this repository
2. Run the following command
   ```sh
   cargo bench
   ```

Result:

```
begin::utf8             time:   [95.365 ns 95.533 ns 95.737 ns]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

begin::byte             time:   [40.124 ns 40.187 ns 40.252 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

begin::chunk_64bytes    time:   [20.141 ns 20.182 ns 20.228 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

begin::chunk_128bytes   time:   [55.567 ns 55.641 ns 55.724 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

middle::utf8            time:   [295.70 µs 296.21 µs 296.73 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

middle::byte            time:   [147.66 µs 147.89 µs 148.14 µs]

middle::chunk_64bytes   time:   [21.503 µs 21.579 µs 21.661 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

middle::chunk_128bytes  time:   [17.160 µs 17.260 µs 17.359 µs]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

end::utf8               time:   [590.92 µs 591.69 µs 592.54 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

end::byte               time:   [198.64 µs 199.26 µs 199.98 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

end::chunk_64bytes      time:   [42.883 µs 42.990 µs 43.096 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

end::chunk_128bytes     time:   [34.024 µs 34.136 µs 34.265 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

unmodified::utf8        time:   [591.32 µs 592.48 µs 593.94 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

unmodified::byte        time:   [198.22 µs 198.49 µs 198.78 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

unmodified::chunk_64bytes
                        time:   [42.694 µs 42.821 µs 42.969 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

unmodified::chunk_128bytes
                        time:   [34.634 µs 34.898 µs 35.188 µs]
```
