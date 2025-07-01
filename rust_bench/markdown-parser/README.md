How to run:

```
cargo bench
```

Result:

```
pulldown-cmark          time:   [7.0763 ms 7.3601 ms 7.6302 ms]

comrak                  time:   [19.790 ms 20.091 ms 20.459 ms]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

cmark-gfm               time:   [12.905 ms 12.961 ms 13.022 ms]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

markdown-rs             time:   [423.35 ms 426.78 ms 430.35 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```

How to avoid cmake v4 error on macOS:

```
brew unlink cmake
brew install ./formula/cmake3.rb
```

After running the benchmark, ensure to remove the older version of cmake:

```
brew uninstall cmake3
brew link cmake
```
