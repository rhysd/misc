package semaphore_bench

import (
	"context"
	"sync"
	"testing"

	"golang.org/x/sync/semaphore"
)

func BenchmarkWorkers8Groutines(b *testing.B) {
	for i := 0; i < b.N; i++ {
		ret := make(chan int)

		c := make(chan int)
		done := make(chan struct{})
		for i := 0; i < 8; i++ {
			go func() {
				total := 0
				for {
					select {
					case x := <-c:
						total += x
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
		for i := 0; i < 8; i++ {
			result += <-ret
		}
		if result != 50005000 {
			panic(result)
		}
	}
}

func BenchmarkWorkersGoroutinePerTask(b *testing.B) {
	for i := 0; i < b.N; i++ {
		result := 0
		mu := sync.Mutex{}

		sema := semaphore.NewWeighted(8)
		ctx := context.Background()
		wg := sync.WaitGroup{}

		count := 10000
		for count > 0 {
			wg.Add(1)
			go func(c int) {
				sema.Acquire(ctx, 1)
				mu.Lock()
				result += c
				mu.Unlock()
				sema.Release(1)
				wg.Done()
			}(count)
			count--
		}

		wg.Wait()

		if result != 50005000 {
			panic(result)
		}
	}
}
