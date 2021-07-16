package semaphore_bench

import (
	"context"
	"testing"

	"golang.org/x/sync/semaphore"
)

const N = 8

func BenchmarkSemaphoreWeighted(b *testing.B) {
	for i := 0; i < b.N; i++ {

		sema := semaphore.NewWeighted(N)
		ctx := context.Background()
		ret := make(chan int)

		c := make(chan int)
		cs := make([]chan struct{}, 0, N*2)
		for i := 0; i < N*2; i++ {
			done := make(chan struct{})
			cs = append(cs, done)
			go func() {
				total := 0
			Loop:
				for {
					select {
					case x := <-c:
						sema.Acquire(ctx, 1)
						total += x
						sema.Release(1)
					case <-done:
						break Loop
					}
				}
				ret <- total
			}()
		}

		count := 10000
		for count > 0 {
			c <- count
			count--
		}

		result := 0
		for _, done := range cs {
			done <- struct{}{}
			result += <-ret
		}
		if result != 50005000 {
			panic(result)
		}
	}
}

func BenchmarkSemaphoreChannel(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sema := make(chan struct{}, N)
		ret := make(chan int)

		c := make(chan int)
		cs := make([]chan struct{}, 0, N*2)
		for i := 0; i < N*2; i++ {
			done := make(chan struct{})
			cs = append(cs, done)
			go func() {
				total := 0
			Loop:
				for {
					select {
					case x := <-c:
						sema <- struct{}{}
						total += x
						<-sema
					case <-done:
						break Loop
					}
				}
				ret <- total
			}()
		}

		count := 10000
		for count > 0 {
			c <- count
			count--
		}

		result := 0
		for _, done := range cs {
			done <- struct{}{}
			result += <-ret
		}
		if result != 50005000 {
			panic(result)
		}
	}
}
