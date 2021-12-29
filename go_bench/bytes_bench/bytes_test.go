package bytes_bench

import (
	"bytes"
	"fmt"
	"testing"
)

func BenchmarkBytesSlice(b *testing.B) {
	for _, sz := range []int{10, 100, 1000} {
		b.Run(fmt.Sprintf("%d", sz), func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				x := make([]byte, 0, sz)
				if len(x) != 0 {
					b.Fatal("oops")
				}
			}
		})
	}
}

func BenchmarkBytesBuffer(b *testing.B) {
	for _, sz := range []int{10, 100, 1000} {
		b.Run(fmt.Sprintf("%d", sz), func(b *testing.B) {
			for i := 0; i < b.N; i++ {
				var by bytes.Buffer
				by.Grow(sz)
				x := by.Bytes()
				if len(x) != 0 {
					b.Fatal("oops")
				}
			}
		})
	}
}
