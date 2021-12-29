package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"os"
	"strconv"

	"github.com/k0kubun/pp"
)

func treeFromFile(file string, fs *token.FileSet) (*ast.File, error) {
	src, err := os.ReadFile(file)
	if err != nil {
		return nil, err
	}

	t, err := parser.ParseFile(fs, file, src, parser.ParseComments)
	if err != nil {
		return nil, err
	}

	fmt.Println("===Go AST for", file, "===")
	pp.Println(t)

	return t, nil
}

func treeBuiltManually(fs *token.FileSet) (*ast.File, error) {
	type Person struct {
		Name string
		Age  int
		Job  string
	}

	data := []Person{
		{"Alice", 20, "designer"},
		{"Bob", 32, "programmer"},
	}

	mapElems := []ast.Expr{}
	for _, p := range data {
		kv := &ast.KeyValueExpr{
			Key: &ast.BasicLit{
				Kind:  token.STRING,
				Value: strconv.Quote(p.Name),
			},
			Value: &ast.UnaryExpr{
				Op: token.AND,
				X: &ast.CompositeLit{
					Type: &ast.Ident{Name: "Info"},
					Elts: []ast.Expr{
						&ast.KeyValueExpr{
							Key: &ast.Ident{Name: "Age"},
							Value: &ast.BasicLit{
								Kind:  token.INT,
								Value: strconv.Itoa(p.Age),
							},
						},
						&ast.KeyValueExpr{
							Key: &ast.Ident{Name: "Job"},
							Value: &ast.BasicLit{
								Kind:  token.STRING,
								Value: strconv.Quote(p.Job),
							},
						},
					},
				},
			},
		}
		mapElems = append(mapElems, kv)
	}

	mapTy := &ast.MapType{
		Key: &ast.Ident{Name: "string"},
		Value: &ast.StarExpr{
			X: &ast.Ident{Name: "SomeStruct"},
		},
	}

	mapLit := &ast.CompositeLit{
		Type: mapTy,
		Elts: mapElems,
	}

	varSpec := &ast.ValueSpec{
		Names: []*ast.Ident{
			{Name: "TestVariable"},
		},
		Values: []ast.Expr{mapLit},
	}

	decl := &ast.GenDecl{
		Tok:   token.VAR,
		Specs: []ast.Spec{varSpec},
	}

	comment := &ast.CommentGroup{
		List: []*ast.Comment{
			{Text: "// This is test comment"},
		},
	}

	root := &ast.File{
		Doc: comment,
		Name: &ast.Ident{
			Name: "this_is_test",
		},
		Decls:    []ast.Decl{decl},
		Comments: []*ast.CommentGroup{comment},
	}

	fmt.Println("=== Go AST generated manually ===")
	pp.Println(root)

	return root, nil
}

func main() {
	fs := token.NewFileSet()

	var t *ast.File
	var err error
	if len(os.Args) > 1 {
		t, err = treeFromFile(os.Args[1], fs)
	} else {
		t, err = treeBuiltManually(fs)
	}

	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		fmt.Fprintln(os.Stderr, "Usage: generate_go_code {file}")
		os.Exit(1)
	}

	fmt.Println("\n=== Printed Go source ===")
	if err := printer.Fprint(os.Stdout, fs, t); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
