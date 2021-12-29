Result:

```
std::fs::read           time:   [9.5639 us 9.5949 us 9.6251 us]
                        change: [+1.1074% +1.5761% +2.0620%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

std::io::Read::bytes    time:   [8.4919 ms 8.5170 ms 8.5441 ms]
                        change: [-0.7152% -0.3469% +0.0349%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

memmap                  time:   [15.285 us 15.350 us 15.420 us]
                        change: [-1.3156% -0.7021% -0.0857%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```
