```
$ go test -bench Workers -benchmem
goos: darwin
goarch: amd64
pkg: github.com/rhysd/misc/workers_bench
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkWorkers8Groutines-20                202           5924730 ns/op             553 B/op          4 allocs/op
BenchmarkWorkersGoroutinePerTask-20           91          12718825 ns/op         1714801 B/op      31051 allocs/op
PASS
ok      github.com/rhysd/misc/workers_bench     3.980s
```
