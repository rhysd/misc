// This is package for dummy
package main

type Info struct {
	Age int
	Job string
}

type Person struct {
	Name string
	Info *Info
}

var ThisIsTest = map[string]*Person{
	"a": {
		Name: "Alice",
		Info: &Info{
			Age: 20,
			Job: "designer",
		},
	},
	"b": {
		Name: "Bob",
		Info: &Info{
			Age: 32,
			Job: "programmer",
		},
	},
}
