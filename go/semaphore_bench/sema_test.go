package semaphore_bench

import (
	"context"
	"testing"

	marusama "github.com/marusama/semaphore/v2"
	"golang.org/x/sync/semaphore"
)

const N = 8

func BenchmarkSemaphoreWeighted(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sema := semaphore.NewWeighted(N)
		ctx := context.Background()
		ret := make(chan int)

		c := make(chan int)
		done := make(chan struct{})
		for i := 0; i < N*2; i++ {
			go func() {
				total := 0
				for {
					select {
					case x := <-c:
						sema.Acquire(ctx, 1)
						total += x
						sema.Release(1)
					case <-done:
						ret <- total
						return
					}
				}
			}()
		}

		count := 10000
		for count > 0 {
			c <- count
			count--
		}

		close(done)
		result := 0
		for i := 0; i < N*2; i++ {
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
		done := make(chan struct{})
		for i := 0; i < N*2; i++ {
			go func() {
				total := 0
				for {
					select {
					case x := <-c:
						sema <- struct{}{}
						total += x
						<-sema
					case <-done:
						ret <- total
						return
					}
				}
			}()
		}

		count := 10000
		for count > 0 {
			c <- count
			count--
		}

		close(done)
		result := 0
		for i := 0; i < N*2; i++ {
			result += <-ret
		}
		if result != 50005000 {
			panic(result)
		}
	}
}

func BenchmarkSemaphoreMarusama(b *testing.B) {
	for i := 0; i < b.N; i++ {
		sema := marusama.New(N)
		ctx := context.Background()
		ret := make(chan int)

		c := make(chan int)
		done := make(chan struct{})
		for i := 0; i < N*2; i++ {
			go func() {
				total := 0
				for {
					select {
					case x := <-c:
						sema.Acquire(ctx, 1)
						total += x
						sema.Release(1)
					case <-done:
						ret <- total
						return
					}
				}
			}()
		}

		count := 10000
		for count > 0 {
			c <- count
			count--
		}

		close(done)
		result := 0
		for i := 0; i < N*2; i++ {
			result += <-ret
		}
		if result != 50005000 {
			panic(result)
		}
	}
}
