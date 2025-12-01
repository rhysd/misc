SHELL := /bin/bash -e -o pipefail
ANS := $(wildcard ans/*)
OUT := $(patsubst ans/%,out/%,$(ANS))
ALL := $(patsubst ans/%,diff/%,$(ANS))

default: $(ALL)

out:
	mkdir -p out
diff:
	mkdir -p diff
in:
	mkdir -p in
ans:
	mkdir -p ans

in/%-1: in
	[ -n "${AOC_SESSION}" ]
	curl -A 'https://github.com/rhysd/misc/tree/master/adventofcode by lin90162@yahoo.co.jp' -L -o in/$*-1 --cookie "session=${AOC_SESSION}" 'https://adventofcode.com/2025/day/$*/input'

out/%-1: %.rs in/%-1 out
	cargo run --release --bin $* -- 1 < in/$*-1 > out/$*-1
out/%-2: %.rs in/%-2 ans/%-2 out
	cargo run --release --bin $* -- 2 < in/$*-2 > out/$*-2
out/%-2: %.rs in/%-1 ans/%-2 out
	cargo run --release --bin $* -- 2 < in/$*-1 > out/$*-2
out/%-s1: %.rs in/%-s1 ans/%-s1 out
	cargo run --bin $* -- 1 < in/$*-s1 > out/$*-s1
out/%-s2: %.rs in/%-s2 ans/%-s2 out
	cargo run --bin $* -- 2 < in/$*-s2 > out/$*-s2
out/%-s2: %.rs in/%-s1 ans/%-s2 out
	cargo run --bin $* -- 2 < in/$*-s1 > out/$*-s2

diff/%: out/% ans/% diff
	diff -u out/$* ans/$* | tee diff/$*

ans/%-1: in/%-1 ans
	cargo run --release --bin $* -- 1 < in/$*-1 | tee ans/$*-1
ans/%-2: in/%-2 ans
	cargo run --release --bin $* -- 2 < in/$*-2 | tee ans/$*-2
ans/%-2: in/%-1 ans
	cargo run --release --bin $* -- 2 < in/$*-1 | tee ans/$*-2

clean:
	rm -f out/* diff/*

.PHONY: default clean
.PRECIOUS: $(OUT) in/%-1 in/%-2
