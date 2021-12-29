```
$ go test -bench BenchmarkBytes -benchmem
goos: darwin
goarch: amd64
pkg: github.com/rhysd/misc/bytes_bench
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkBytesSlice/10-20       78577716                14.01 ns/op           16 B/op          1 allocs/op
BenchmarkBytesSlice/100-20      40627003                24.83 ns/op          112 B/op          1 allocs/op
BenchmarkBytesSlice/1000-20              9694864               124.9 ns/op          1024 B/op          1 allocs/op
BenchmarkBytesBuffer/10-20              46666028                23.33 ns/op           64 B/op          1 allocs/op
BenchmarkBytesBuffer/100-20             32540277                35.88 ns/op          112 B/op          1 allocs/op
BenchmarkBytesBuffer/1000-20             8605777               138.8 ns/op          1024 B/op          1 allocs/op
PASS
ok      github.com/rhysd/misc/bytes_bench       7.198s
```
