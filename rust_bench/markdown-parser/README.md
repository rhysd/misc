How to run:

```
cargo bench
```

Result:

```
pulldown-cmark          time:   [5.5270 ms 5.7169 ms 5.9147 ms]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

comrak                  time:   [19.020 ms 19.162 ms 19.317 ms]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

cmark-gfm               time:   [12.953 ms 13.049 ms 13.159 ms]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

markdown-rs             time:   [407.42 ms 411.27 ms 415.19 ms]
```
