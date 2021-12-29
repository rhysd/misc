```
goos: darwin
goarch: amd64
pkg: github.com/rhysd/quotes
cpu: Intel(R) Core(TM) i9-10910 CPU @ 3.60GHz
BenchmarkQuotesBaseline-20                408708              2714 ns/op            3456 B/op          1 allocs/op
BenchmarkQuotesNaive-20                    32478             36960 ns/op           18952 B/op        945 allocs/op
BenchmarkQuotesAppendQuote-20              54133             22168 ns/op            8368 B/op          4 allocs/op
BenchmarkQuotesUnsafe-20                   57135             21097 ns/op           11776 B/op          3 allocs/op
BenchmarkQuotes-20                         92425             12698 ns/op            8320 B/op          2 allocs/op
PASS
ok      github.com/rhysd/quotes 7.068s
```
