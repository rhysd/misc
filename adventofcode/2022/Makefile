SHELL := /bin/bash -e -o pipefail
ALL := $(filter-out template,$(patsubst %.rs,%,$(wildcard *.rs)))
STDOUT := $(patsubst %.want,%.stdout,$(wildcard test/*.want))

test/%_1.stdout: %.rs test/% test/%_1.want
	cargo run --bin $* 1 < test/$* > test/$*_1.stdout
	diff -u test/$*_1.stdout test/$*_1.want | tee test/$*_1.diff

test/%_2.stdout: %.rs test/% test/%_2.want
	cargo run --bin $* 2 < test/$* > test/$*_2.stdout
	diff -u test/$*_2.stdout test/$*_2.want | tee test/$*_2.diff

test: $(STDOUT)

clean:
	rm -f test/*.stdout test/*.diff

default: test

.PHONY: test clean
