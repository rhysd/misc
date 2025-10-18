package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"io"
	"os"
)

func main() {
	var file string
	var src []byte
	switch len(os.Args) {
	case 1:
		b, err := io.ReadAll(os.Stdin)
		if err != nil {
			panic(err)
		}
		src = b
	case 2:
		b, err := os.ReadFile(os.Args[1])
		if err != nil {
			panic(err)
		}
		src = b
	default:
		panic("only one file can be passed")
	}

	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, file, src, 0)
	if err != nil {
		panic(err)
	}

	ast.Print(fset, f)

}
