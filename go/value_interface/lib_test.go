package value_interface

import "testing"

type I interface {
	Calc() int8
}

type S struct {
	i int8
}

func (s S) Calc() int8 {
	return s.i + 1
}

func (s S) Double() int8 {
	return s.i * 2
}

type P struct {
	i int8
}

func (p *P) Calc() int8 {
	return p.i + 1
}

func (p *P) Triple() int8 {
	return p.i * 3
}

func calc(i I) int8 {
	x := i.Calc()
	switch v := i.(type) {
	case S:
		x = v.Double()
	case *P:
		x = v.Triple()
	}
	return x
}

func BenchmarkStructValue(b *testing.B) {
	s := S{1}
	for i := 0; i < b.N; i++ {
		x := calc(s)
		if x <= 0 {
			panic("oops")
		}
	}
}

func BenchmarkPointerValue(b *testing.B) {
	p := P{}
	for i := 0; i < b.N; i++ {
		p.i = 1
		x := calc(&p)
		if x <= 0 {
			panic("oops")
		}
	}
}
