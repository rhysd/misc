Performance comparison of `cmp.Equal` and `cmp.Diff` in context of https://github.com/rhysd/actionlint/pull/475.

How to run:

```sh
go test -bench Benchmark -benchmem
```

Result:

```
BenchmarkCmpEqual-24                 913           1312699 ns/op          654725 B/op      14681 allocs/op
BenchmarkCmpDiff-24                  922           1333190 ns/op          654690 B/op      14681 allocs/op
PASS
```
