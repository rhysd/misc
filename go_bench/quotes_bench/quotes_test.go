package quotes

import "testing"

var data = []string{
	"Doggo", "ipsu\\m", "what", "a", "nice", "floof", "vvv", "you", "are", "doin", "\\me", "a", "concern", "very", "good", "spot", "dat", "tungg", "tho", "s\\mol", "borking", "doggo", "with", "a", "long", "snoot", "for", "pats", "You", "are", "doin", "\\me", "a", "concern", "heckin", "boof", "doing", "\\me", "a", "frighten", "bork", "boof", "blop", "Waggy", "wags", "s\\mol", "borking", "doggo", "with", "a", "long", "snoot", "for", "pats", "blop", "pats", "doggo", "big", "ol", "Shoober", "super", "chub", "long", "woofer", "boof", "\\much", "ruin", "diet", "adorable", "doggo", "woofer", "ur", "givin", "\\me", "a", "spook", "super", "chub", "length", "boy", "such", "treat", "very", "good", "spot", "the", "neighborhood", "pupper", "boofers", "s\\mol", "long", "woofer", "\\most", "angery", "pupper", "I", "have", "ever", "seen", "you", "are", "doin", "\\me", "a", "concern", "H", "ck", "lotsa", "pats", "tungg", "wow", "such", "te\\mpt", "\\much", "ruin", "diet", "adorable", "doggo", "shoober", "snoot", "waggy", "wags",
	"Floofs", "ruff", "big", "ol", "you", "are", "doing", "\\me", "the", "shock", "vvv", "yapper", "long", "water", "shoob", "wow", "such", "te\\mpt", "very", "taste", "wow", "\\maxi\\mu\\m", "borkdrive", "puggorino", "Sub", "woofer", "extre\\mely", "cuuuuuute", "puggo", "sub", "woofer", "Shoober", "I", "a\\m", "beko\\m", "fat", "yapper", "extre\\mely", "cuuuuuute", "s\\mol", "borking", "doggo", "with", "a", "long", "snoot", "for", "pats", "blep", "doggo", "heckin", "angery", "woofer", "floofs", "\\mle\\m", "wow", "such", "te\\mpt", "doing", "\\me", "a", "frighten", "Floofs", "\\many", "pats", "the", "neighborhood", "pupper", "big", "ol", "pupper", "long", "bois", "doge", "Borking", "doggo", "what", "a", "nice", "floof", "such", "treat", "shibe", "Doggorino", "such", "treat", "\\most", "angery", "pupper", "I", "have", "ever", "seen", "shibe", "pats", "long", "doggo", "h", "ck", "what", "a", "nice", "floof", "floofs", "dat", "tungg", "tho", "Sub", "woofer", "he", "\\made", "\\many", "woofs", "blop", "long", "water", "shoob", "heckin", "vvv", "thicc", "he", "\\made", "\\many", "woofs", "\\mle\\m", "Doggo", "vvv", "very", "good", "spot", "blep", "ruff", "big", "ol", "pupper", "bork", "shoober", "floofs", "clouds",
	"Thicc", "length", "boy", "wow", "such", "te\\mpt", "extre\\mely", "cuuuuuute", "boofers", "length", "boy", "you", "are", "doing", "\\me", "a", "frighten", "s\\mol", "borking", "doggo", "with", "a", "long", "snoot", "for", "pats", "Very", "taste", "wow", "heckin", "good", "boys", "and", "girls", "shibe", "he", "\\made", "\\many", "woofs", "doggorino", "shoob", "he", "\\made", "\\many", "woofs", "tungg", "woofer", "heck", "wow", "very", "biscit", "\\much", "ruin", "diet", "long", "woofer", "Heck", "very", "good", "spot", "pupperino", "he", "\\made", "\\many", "woofs", "doge", "doing", "\\me", "a", "frighten", "corgo", "wow", "such", "te\\mpt", "blep", "Doing", "\\me", "a", "frighten", "shoober", "fat", "boi", "pupperino", "lotsa", "pats", "heckin", "good", "boys", "and", "girls", "you", "are", "doing", "\\me", "the", "shock", "\\most", "angery", "pupper", "I", "have", "ever", "seen", "the", "neighborhood", "pupper", "you", "are", "doin", "\\me", "a", "concern", "boof", "Long", "bois", "pupperino", "long", "doggo", "heckin", "good", "boys", "and", "girls", "bork", "Lotsa", "pats", "\\most", "angery", "pupper", "I", "have", "ever", "seen", "heckin", "good", "boys", "you", "are", "doing", "\\me", "a", "frighten", "super", "chub",
}

func BenchmarkQuotesBaseline(b *testing.B) {
	for i := 0; i < b.N; i++ {
		QuotesBaseline(data)
	}
}

func BenchmarkQuotesNaive(b *testing.B) {
	for i := 0; i < b.N; i++ {
		QuotesNaive(data)
	}
}

func BenchmarkQuotesAppendQuote(b *testing.B) {
	for i := 0; i < b.N; i++ {
		QuotesAppendQuote(data)
	}
}

func BenchmarkQuotesUnsafe(b *testing.B) {
	for i := 0; i < b.N; i++ {
		QuotesUnsafe(data)
	}
}

func BenchmarkQuotes(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Quotes(data)
	}
}
