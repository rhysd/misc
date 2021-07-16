```
> go test -bench Semaphore -benchmem
goos: darwin
goarch: amd64
pkg: github.com/rhysd/misc/semaphore_bench
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkSemaphoreWeighted-20                139           8670369 ns/op            2293 B/op         21 allocs/op
BenchmarkSemaphoreChannel-20                 130           9129535 ns/op            1984 B/op         20 allocs/op
PASS
ok      github.com/rhysd/misc/semaphore_bench   4.394s
```
