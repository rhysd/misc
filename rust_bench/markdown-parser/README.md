How to run:

```
cargo bench
```

Result:

```
pulldown-cmark          time:   [6.3585 ms 6.6526 ms 6.9476 ms]

comrak                  time:   [19.936 ms 20.104 ms 20.276 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

cmark-gfm               time:   [13.172 ms 13.324 ms 13.518 ms]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking markdown-rs: Warming up for 3.0000 s
markdown-rs             time:   [409.44 ms 414.25 ms 419.10 ms]
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
