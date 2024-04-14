UTF-8 index v.s. byte index on finding modified byte offset
===========================================================

Problem: Calculate a byte offset of the first modified position in two given strings taking care of UTF-8 character boundaries.

- `utf8` : Iterate two strings in UTF-8 character-wise
- `byte` : Iterate two strings in byte-wise then find the nearest character-wise offset
- `chunk_32_bytes`, `chunk_64_bytes`, `chunk_128_bytes`, `chunk_256_bytes` : Iterate two strings per 32/64/128/256 bytes chunks and find the first mismatch in the chunk in byte-wise, then find the nearest character-wise offset

How to run:

1. Prepare an input file as `test.txt` at the root of this repository
2. Run the following command
   ```sh
   cargo bench
   ```

Result summary:

|                   | Begin     | Middle    | End       | Unmodified |
|-------------------|-----------|-----------|-----------|------------|
| `utf8`            | 94.947 ns | 294.25 µs | 588.11 µs | 587.56 µs  |
| `byte`            | 49.136 ns | 147.56 µs | 294.36 µs | 293.93 µs  |
| `chunk_32_bytes`  | 7.6841 ns | 13.082 µs | 24.332 µs | 24.154 µs  |
| `chunk_64_bytes`  | 20.103 ns | 22.576 µs | 45.027 µs | 45.017 µs  |
| `chunk_128_bytes` | 47.887 ns | 15.963 µs | 32.143 µs | 31.843 µs  |
| `chunk_256_bytes` | 55.893 ns | 13.955 µs | 28.725 µs | 27.489 µs  |

- **Begin**: The `100`th byte is modified
- **Middle**: The `len / 2`th byte is modified
- **End**: The `len - 100`th byte is modified
- **Unmodified**: No byte is modified

Result details:

```
begin::utf8             time:   [94.593 ns 94.947 ns 95.380 ns]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  6 (6.00%) high severe

begin::byte             time:   [49.030 ns 49.136 ns 49.249 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

begin::chunk_32_bytes   time:   [7.6623 ns 7.6841 ns 7.7073 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe

begin::chunk_64_bytes   time:   [20.027 ns 20.103 ns 20.186 ns]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe

begin::chunk_128_bytes  time:   [47.786 ns 47.887 ns 47.992 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

begin::chunk_256_bytes  time:   [55.765 ns 55.893 ns 56.041 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

middle::utf8            time:   [293.18 µs 294.25 µs 295.54 µs]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  2 (2.00%) high severe

middle::byte            time:   [147.18 µs 147.56 µs 147.97 µs]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe

middle::chunk_32_bytes  time:   [12.861 µs 13.082 µs 13.301 µs]

middle::chunk_64_bytes  time:   [22.515 µs 22.576 µs 22.647 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

middle::chunk_128_bytes time:   [15.821 µs 15.963 µs 16.128 µs]

middle::chunk_256_bytes time:   [13.723 µs 13.955 µs 14.185 µs]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

end::utf8               time:   [585.82 µs 588.11 µs 590.78 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

end::byte               time:   [293.69 µs 294.36 µs 295.09 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

end::chunk_32_bytes     time:   [23.948 µs 24.332 µs 24.773 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

end::chunk_64_bytes     time:   [44.853 µs 45.027 µs 45.213 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

end::chunk_128_bytes    time:   [31.852 µs 32.143 µs 32.456 µs]

end::chunk_256_bytes    time:   [28.116 µs 28.725 µs 29.310 µs]

unmodified::utf8        time:   [585.93 µs 587.56 µs 589.37 µs]
Found 19 outliers among 100 measurements (19.00%)
  6 (6.00%) low mild
  10 (10.00%) high mild
  3 (3.00%) high severe

unmodified::byte        time:   [293.37 µs 293.93 µs 294.52 µs]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

unmodified::chunk_32_bytes
                        time:   [23.835 µs 24.154 µs 24.509 µs]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

unmodified::chunk_64_bytes
                        time:   [44.814 µs 45.017 µs 45.265 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

unmodified::chunk_128_bytes
                        time:   [31.514 µs 31.843 µs 32.204 µs]

unmodified::chunk_256_bytes
                        time:   [27.106 µs 27.489 µs 27.917 µs]
```
