SHELL := /bin/bash -e -o pipefail
STDOUT := $(patsubst want/%,out/%,$(wildcard want/*))

default: $(STDOUT)

out/ diff/:
	mkdir -p out diff

out/%-1: %.rs in/%-1 want/%-1 out/ diff/
	cargo run --bin $* -- 1 < in/$*-1 > out/$*-1
	diff -u out/$*-1 want/$*-1 | tee diff/$*-1

out/%-2: %.rs in/%-2 want/%-2 out/ diff/
	cargo run --bin $* -- 2 < in/$*-2 > out/$*-2
	diff -u out/$*-2 want/$*-2 | tee diff/$*-2

out/%-2: %.rs in/%-1 want/%-2 out/ diff/
	cargo run --bin $* -- 2 < in/$*-1 > out/$*-2
	diff -u out/$*-2 want/$*-2 | tee diff/$*-2

out/%-s1: %.rs in/%-s1 want/%-s1 out/ diff/
	cargo run --bin $* -- 1 < in/$*-s1 > out/$*-s1
	diff -u out/$*-s1 want/$*-s1 | tee diff/$*-s1

out/%-s2: %.rs in/%-s2 want/%-s2 out/ diff/
	cargo run --bin $* -- 2 < in/$*-s2 > out/$*-s2
	diff -u out/$*-s2 want/$*-s2 | tee diff/$*-s2

clean:
	rm -f out/* diff/*

.PHONY: default clean
