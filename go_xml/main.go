package main

import (
	"bytes"
	"encoding/xml"
	"fmt"
	"io"
	"os"
	"strings"
)

func must[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

func indent(l int) string {
	return strings.Repeat("  ", l)
}

func tag(n xml.Name) string {
	t := n.Local
	if len(n.Space) > 0 {
		t = n.Space + ":" + t
	}
	return t
}

func main() {
	in := must(io.ReadAll(os.Stdin))
	d := xml.NewDecoder(bytes.NewReader(in))
	d.Strict = false
	start := int64(0)
	ind := 0
	line, col := 1, 1
	stack := []bool{}

	for {
		t, err := d.RawToken()
		if err == io.EOF {
			break
		} else if err != nil {
			panic(err)
		}

		end := d.InputOffset()
		footer := fmt.Sprintf("(line: %d, col: %d, raw: %q)", line, col, in[start:end])
		start = end
		line, col = d.InputPos()

		switch t := t.(type) {
		case xml.StartElement:
			fmt.Printf("%sStart: %q %s\n", indent(ind), tag(t.Name), footer)
			ind++
			if len(t.Attr) > 0 {
				for _, a := range t.Attr {
					fmt.Printf("%sAttr: %s=%q\n", indent(ind), tag(a.Name), a.Value)
				}
				ind++
			}
			stack = append(stack, len(t.Attr) > 0)
		case xml.EndElement:
			ind--
			if stack[len(stack)-1] {
				ind--
			}
			stack = stack[:len(stack)-1]
			fmt.Printf("%sEnd: %q %s\n", indent(ind), tag(t.Name), footer)
		case xml.CharData:
			fmt.Printf("%sCharData: %q %s\n", indent(ind), t, footer)
		case xml.Comment:
			fmt.Printf("%sComment: %q %s\n", indent(ind), t, footer)
		case xml.ProcInst:
			fmt.Printf("%sProcInst: %q %s\n", indent(ind), t, footer)
		case xml.Directive:
			fmt.Printf("%sDirective: %q %s\n", indent(ind), t, footer)
		default:
			panic("unreachable")
		}
	}
}
