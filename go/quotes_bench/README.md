```
goos: darwin
goarch: amd64
pkg: github.com/rhysd/quotes
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkQuotesBaseline-20                311409              3359 ns/op            3456 B/op          1 allocs/op
BenchmarkQuotesNaive-20                    32328             36974 ns/op           18952 B/op        945 allocs/op
BenchmarkQuotesAppendQuote-20              54199             22132 ns/op            8368 B/op          4 allocs/op
BenchmarkQuotesUnsafe-20                   58532             20532 ns/op            8320 B/op          2 allocs/op
BenchmarkQuotes-20                         68354             17464 ns/op            8320 B/op          2 allocs/op
PASS
ok      github.com/rhysd/quotes 7.163s
```
