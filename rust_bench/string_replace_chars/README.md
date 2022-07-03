```
Benchmarking do nothing
Benchmarking do nothing: Warming up for 3.0000 s
Benchmarking do nothing: Collecting 100 samples in estimated 5.0005 s (45M iterations)
Benchmarking do nothing: Analyzing
do nothing              time:   [105.79 ns 107.21 ns 109.00 ns]

Benchmarking direct
Benchmarking direct: Warming up for 3.0000 s
Benchmarking direct: Collecting 100 samples in estimated 5.0031 s (3.2M iterations)
Benchmarking direct: Analyzing
direct                  time:   [1.5602 us 1.5691 us 1.5786 us]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking direct ascii
Benchmarking direct ascii: Warming up for 3.0000 s
Benchmarking direct ascii: Collecting 100 samples in estimated 5.0002 s (13M iterations)
Benchmarking direct ascii: Analyzing
direct ascii            time:   [379.85 ns 382.91 ns 386.24 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

Benchmarking copy
Benchmarking copy: Warming up for 3.0000 s
Benchmarking copy: Collecting 100 samples in estimated 5.0067 s (3.3M iterations)
Benchmarking copy: Analyzing
copy                    time:   [1.4979 us 1.5077 us 1.5186 us]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```
