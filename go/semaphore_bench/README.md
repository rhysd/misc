```console
$ go test -bench . -benchmem
goos: darwin
goarch: amd64
pkg: github.com/rhysd/misc/semaphore_bench
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkSemaphoreWeighted-20                129           9332678 ns/op             867 B/op          6 allocs/op
BenchmarkSemaphoreChannel-20                 122           9774523 ns/op             542 B/op          5 allocs/op
BenchmarkSemaphoreMarusama-20                100          10733502 ns/op          960698 B/op      10006 allocs/op
PASS
ok      github.com/rhysd/misc/semaphore_bench   5.600s
```
