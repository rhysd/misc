package cmp_equal_vs_diff

import (
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/google/go-cmp/cmp/cmpopts"
	"github.com/rhysd/actionlint"
)

func prepareData() *actionlint.Workflow {
	b, err := os.ReadFile("test.yaml")
	if err != nil {
		panic(err)
	}
	w, errs := actionlint.Parse(b)
	if len(errs) != 0 {
		panic(errs)
	}
	return w
}

var data1 = prepareData()
var data2 = prepareData()

func BenchmarkCmpEqual(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if !cmp.Equal(data1, data2, cmpopts.IgnoreUnexported(actionlint.RawYAMLString{})) {
			b.Fatal("not equal")
		}
	}
}

func BenchmarkCmpDiff(b *testing.B) {
	for i := 0; i < b.N; i++ {
		diff := cmp.Diff(data1, data2, cmpopts.IgnoreUnexported(actionlint.RawYAMLString{}))
		if diff != "" {
			b.Fatal("not equal")
		}
	}
}
