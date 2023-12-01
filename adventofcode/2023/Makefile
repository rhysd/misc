SHELL := /bin/bash -e -o pipefail
STDOUT := $(patsubst ans/%,out/%,$(wildcard ans/*))

default: $(STDOUT)

out/ diff/:
	mkdir -p out diff

out/%-1: %.rs in/%-1 ans/%-1 out/ diff/
	cargo run --release --bin $* -- 1 < in/$*-1 > out/$*-1
	diff -u out/$*-1 ans/$*-1 | tee diff/$*-1

out/%-2: %.rs in/%-2 ans/%-2 out/ diff/
	cargo run --release --bin $* -- 2 < in/$*-2 > out/$*-2
	diff -u out/$*-2 ans/$*-2 | tee diff/$*-2

out/%-2: %.rs in/%-1 ans/%-2 out/ diff/
	cargo run --release --bin $* -- 2 < in/$*-1 > out/$*-2
	diff -u out/$*-2 ans/$*-2 | tee diff/$*-2

ans/%-1: %.rs in/%-1 out/
	cargo run --release --bin $* -- 1 < in/$*-1 | tee ans/$*-1

ans/%-2: %.rs in/%-2 out/
	cargo run --release --bin $* -- 2 < in/$*-2 | tee ans/$*-2

ans/%-2: %.rs in/%-1 out/
	cargo run --release --bin $* -- 2 < in/$*-1 | tee ans/$*-2

out/%-s1: %.rs in/%-s1 ans/%-s1 out/ diff/
	cargo run --release --bin $* -- 1 < in/$*-s1 > out/$*-s1
	diff -u out/$*-s1 ans/$*-s1 | tee diff/$*-s1

out/%-s2: %.rs in/%-s2 ans/%-s2 out/ diff/
	cargo run --release --bin $* -- 2 < in/$*-s2 > out/$*-s2
	diff -u out/$*-s2 ans/$*-s2 | tee diff/$*-s2

clean:
	rm -f out/* diff/*

.PHONY: default clean
