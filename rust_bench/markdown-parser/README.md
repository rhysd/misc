How to run:

```
cargo bench
```

Result:

```
pulldown-cmark          time:   [5.9998 ms 6.0253 ms 6.0536 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

comrak                  time:   [48.436 ms 48.601 ms 48.776 ms]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

cmark-gfm               time:   [19.790 ms 19.846 ms 19.901 ms]
```
