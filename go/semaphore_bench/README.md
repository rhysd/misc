```console
$ go test -bench Semaphore -benchmem
goos: darwin
goarch: amd64
pkg: github.com/rhysd/misc/semaphore_bench
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkSemaphoreWeighted-20                130           9171097 ns/op             923 B/op          6 allocs/op
BenchmarkSemaphoreChannel-20                 122           9746041 ns/op             430 B/op          4 allocs/op
PASS
ok      github.com/rhysd/misc/semaphore_bench   4.394s
```
