package concurrent_map

import (
	"strconv"
	"sync"
	"testing"

	"github.com/alphadose/haxmap"
	cmap "github.com/orcaman/concurrent-map/v2"
	"github.com/puzpuzpuz/xsync/v3"
)

type RWMutexMap struct {
	mu sync.RWMutex
	m  map[string]*X
}

func (m *RWMutexMap) Load(k string) (*X, bool) {
	m.mu.RLock()
	x, ok := m.m[k]
	m.mu.RUnlock()
	return x, ok
}

func (m *RWMutexMap) Store(k string, v *X) {
	m.mu.Lock()
	m.m[k] = v
	m.mu.Unlock()
}

type X struct {
	Value int
}

func BenchmarkBaseline(b *testing.B) {
	for range b.N {
		m := map[string]*X{}
		for i := 0; i < 100; i++ {
			k := "foo-" + strconv.Itoa(i)
			if _, ok := m[k]; ok {
				b.Fatal("cache should not hit")
			}
			m[k] = &X{i}
		}
		for i := 0; i < 1000; i++ {
			k := "foo-" + strconv.Itoa(i%100)
			x, ok := m[k]
			if !ok {
				b.Fatal("cache should hit")
			}
			if x.Value != i%100 {
				b.Fatal("unexpected value")
			}
		}
	}
}

func BenchmarkRWMutexMap(b *testing.B) {
	for range b.N {
		m := RWMutexMap{m: map[string]*X{}}
		for i := 0; i < 100; i++ {
			k := "foo-" + strconv.Itoa(i)
			if _, ok := m.Load(k); ok {
				b.Fatal("cache should not hit")
			}
			m.Store(k, &X{i})
		}
		for i := 0; i < 1000; i++ {
			k := "foo-" + strconv.Itoa(i%100)
			x, ok := m.Load(k)
			if !ok {
				b.Fatal("cache should hit")
			}
			if x.Value != i%100 {
				b.Fatal("unexpected value")
			}
		}
	}
}

func BenchmarkSyncMap(b *testing.B) {
	for range b.N {
		m := sync.Map{}
		for i := 0; i < 100; i++ {
			k := "foo-" + strconv.Itoa(i)
			if _, ok := m.Load(k); ok {
				b.Fatal("cache should not hit")
			}
			m.Store(k, &X{i})
		}
		for i := 0; i < 1000; i++ {
			k := "foo-" + strconv.Itoa(i%100)
			x, ok := m.Load(k)
			if !ok {
				b.Fatal("cache should hit")
			}
			if x.(*X).Value != i%100 {
				b.Fatal("unexpected value")
			}
		}
	}
}

func BenchmarkXsyncMapOf(b *testing.B) {
	for range b.N {
		m := xsync.NewMapOf[string, *X]()
		for i := 0; i < 100; i++ {
			k := "foo-" + strconv.Itoa(i)
			if _, ok := m.Load(k); ok {
				b.Fatal("cache should not hit")
			}
			m.Store(k, &X{i})
		}
		for i := 0; i < 1000; i++ {
			k := "foo-" + strconv.Itoa(i%100)
			x, ok := m.Load(k)
			if !ok {
				b.Fatal("cache should hit")
			}
			if x.Value != i%100 {
				b.Fatal("unexpected value")
			}
		}
	}
}

func BenchmarkHaxmapMap(b *testing.B) {
	for range b.N {
		m := haxmap.New[string, *X]()
		for i := 0; i < 100; i++ {
			k := "foo-" + strconv.Itoa(i)
			if _, ok := m.Get(k); ok {
				b.Fatal("cache should not hit")
			}
			m.Set(k, &X{i})
		}
		for i := 0; i < 1000; i++ {
			k := "foo-" + strconv.Itoa(i%100)
			x, ok := m.Get(k)
			if !ok {
				b.Fatal("cache should hit")
			}
			if x.Value != i%100 {
				b.Fatal("unexpected value")
			}
		}
	}
}

func BenchmarkCmapMap(b *testing.B) {
	for range b.N {
		m := cmap.New[*X]()
		for i := 0; i < 100; i++ {
			k := "foo-" + strconv.Itoa(i)
			if _, ok := m.Get(k); ok {
				b.Fatal("cache should not hit")
			}
			m.Set(k, &X{i})
		}
		for i := 0; i < 1000; i++ {
			k := "foo-" + strconv.Itoa(i%100)
			x, ok := m.Get(k)
			if !ok {
				b.Fatal("cache should hit")
			}
			if x.Value != i%100 {
				b.Fatal("unexpected value")
			}
		}
	}
}
