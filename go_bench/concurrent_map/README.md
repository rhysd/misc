Benchmarks for measuring the latency of several concurrent map implementations in Go.

- `BenchmarkBaseline`: Standard `map` type that isn't thread-safe (base line)
- `BenchmarkRWMutexMap`: Naive thread-safe map implemented with `sync.RWMutex`
- `BenchmarkSyncMap`: Standard [`sync.Map`](https://pkg.go.dev/sync#Map)
- `BenchmarkXsyncMapOf`: [`github.com/puzpuzpuz/xsync/v3.MapOf`](https://github.com/puzpuzpuz/xsync)
- `BenchmarkHaxmapMap`: [`github.com/alphadose/haxmap`](https://github.com/alphadose/haxmap)
- `BenchmarkCmapMap`: [`github.com/orcaman/concurrent-map/v2`](https://github.com/orcaman/concurrent-map)

Run:

```sh
go test -bench . -benchmem
```

Result on my machine:

```
goos: darwin
goarch: amd64
pkg: github.com/rhysd/misc/go_bench/concurrent_map
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkBaseline-20               24967             48264 ns/op            9419 B/op        207 allocs/op
BenchmarkRWMutexMap-20             22066             53873 ns/op            9703 B/op        210 allocs/op
BenchmarkSyncMap-20                15448             77335 ns/op           15625 B/op        616 allocs/op
BenchmarkXsyncMapOf-20             23679             51989 ns/op           14951 B/op       1309 allocs/op
BenchmarkHaxmapMap-20              18586             63692 ns/op           19720 B/op       1416 allocs/op
BenchmarkCmapMap-20                16986             71182 ns/op           17936 B/op       1293 allocs/op
PASS
ok      github.com/rhysd/misc/go_bench/concurrent_map   11.019s
```
