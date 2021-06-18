package quotes

import (
	"strconv"
	"strings"
	"unicode/utf8"
	"unsafe"
)

func QuotesNaive(ss []string) string {
	qs := make([]string, 0, len(ss))
	for _, s := range ss {
		qs = append(qs, strconv.Quote(s))
	}
	return strings.Join(qs, ", ")
}

func QuotesBaseline(ss []string) string {
	l := len(ss)
	if l == 0 {
		return ""
	}

	n := 0
	for _, s := range ss {
		n += len(s) + 2 // + 2 for "
	}
	n += (l - 1) * 2 // for comma separators

	var b strings.Builder
	b.Grow(n)

	for i, s := range ss {
		if i > 0 {
			b.WriteString(", \"")
		} else {
			b.WriteByte('"')
		}
		b.WriteString(s)
		b.WriteByte('"')
	}

	return b.String()
}

func QuotesAppendQuote(ss []string) string {
	l := len(ss)
	if l == 0 {
		return ""
	}

	n := 0
	max := 0
	for _, s := range ss {
		m := len(s) + 2
		n += m // + 2 for "
		if m > max {
			max = m
		}
	}
	n += (l - 1) * 2 // for comma separators

	var b strings.Builder
	b.Grow(n)

	buf := make([]byte, 0, max)

	buf = strconv.AppendQuote(buf, ss[0])
	b.Write(buf)

	for _, s := range ss[1:] {
		b.WriteString(", ")
		buf = strconv.AppendQuote(buf[:0], s)
		b.Write(buf)
	}

	return b.String()
}

func QuotesUnsafe(ss []string) string {
	l := len(ss)
	if l == 0 {
		return ""
	}

	n := 0
	for _, s := range ss {
		m := len(s) + 2
		n += m // + 2 for "
	}
	n += (l - 1) * 2 // for comma separators

	buf := make([]byte, 0, n)

	for i, s := range ss {
		if i > 0 {
			buf = append(buf, ',', ' ')
		}
		buf = strconv.AppendQuote(buf, s)
	}

	return *(*string)(unsafe.Pointer(&buf))
}

const lowerhex = "0123456789abcdef"

func Quotes(ss []string) string {
	l := len(ss)
	if l == 0 {
		return ""
	}

	n := 0
	for _, s := range ss {
		m := len(s) + 2
		n += m // + 2 for "
	}
	n += (l - 1) * 2 // for comma separators

	var b strings.Builder
	b.Grow(n)

	b.WriteByte('"')
	for i, s := range ss {
		if i > 0 {
			b.WriteString("\", \"")
		}

		for width := 0; len(s) > 0; s = s[width:] {
			r := rune(s[0])
			width = 1
			if r >= utf8.RuneSelf {
				r, width = utf8.DecodeRuneInString(s)
			}
			if width == 1 && r == utf8.RuneError {
				b.WriteString(`\x`)
				b.WriteByte(lowerhex[s[0]>>4])
				b.WriteByte(lowerhex[s[0]&0xF])
				continue
			}

			if r == '"' || r == '\\' { // always backslashed
				b.WriteByte('\\')
				b.WriteRune(r)
				continue
			}
			if strconv.IsPrint(r) {
				b.WriteRune(r)
				continue
			}

			switch r {
			case '\a':
				b.WriteString(`\a`)
			case '\b':
				b.WriteString(`\b`)
			case '\f':
				b.WriteString(`\f`)
			case '\n':
				b.WriteString(`\n`)
			case '\r':
				b.WriteString(`\r`)
			case '\t':
				b.WriteString(`\t`)
			case '\v':
				b.WriteString(`\v`)
			default:
				switch {
				case r < ' ':
					b.WriteString(`\x`)
					b.WriteByte(lowerhex[byte(r)>>4])
					b.WriteByte(lowerhex[byte(r)&0xF])
				case r > utf8.MaxRune:
					r = 0xFFFD
					fallthrough
				case r < 0x10000:
					b.WriteString(`\u`)
					for s := 12; s >= 0; s -= 4 {
						b.WriteByte(lowerhex[r>>uint(s)&0xF])
					}
				default:
					b.WriteString(`\U`)
					for s := 28; s >= 0; s -= 4 {
						b.WriteByte(lowerhex[r>>uint(s)&0xF])
					}
				}
			}
		}
	}
	b.WriteByte('"')

	return b.String()
}
