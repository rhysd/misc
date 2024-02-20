UTF-8 index v.s. byte index on finding modified byte offset
===========================================================

Problem: Calculate a byte offset of the first modified position in two given strings taking care of UTF-8 character boundaries.

- `utf8` : Iterate two strings in UTF-8 character-wise
- `byte` : Iterate two strings in byte-wise then find the nearest character-wise offset

How to run:

1. Prepare an input file as `test.txt` at the root of this repository
2. Run the following command
   ```sh
   cargo bench
   ```

Result:

```
begin::utf8             time:   [175.01 ns 178.57 ns 182.80 ns]
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe

begin::byte             time:   [75.509 ns 76.734 ns 78.347 ns]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) high mild
  14 (14.00%) high severe

middle::utf8            time:   [22.866 µs 23.207 µs 23.624 µs]
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) high mild
  9 (9.00%) high severe

middle::byte            time:   [7.8988 µs 8.0545 µs 8.2496 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

end::utf8               time:   [46.815 µs 48.217 µs 50.310 µs]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

end::byte               time:   [15.858 µs 16.214 µs 16.638 µs]
Found 18 outliers among 100 measurements (18.00%)
  5 (5.00%) high mild
  13 (13.00%) high severe

unmodified::utf8        time:   [46.718 µs 47.467 µs 48.430 µs]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

unmodified::byte        time:   [15.776 µs 16.093 µs 16.505 µs]
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) high mild
  16 (16.00%) high severe
```
