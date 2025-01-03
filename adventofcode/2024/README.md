Answers for https://adventofcode.com/2024

## Difficulty

- E: Easy
- N: Normal
- H: Hard

| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 |
|---|---|---|---|---|---|---|---|---|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|
| E | E | E | E | E | N | E | E | N | E  | N  | H  | E  | N  | N  | N  | H  | E  | N  | N  | H  | E  | N  | H  | E  |

## Usage

Run solve all puzzles and check diffs:

```sh
make
```

[watchexec][] is useful:

```sh
watchexec make
```

Create answer:

```sh
# Part 2 of day 3
make ans/3-2
```

Clean results:

```sh
make clean
```

Without using `make`,

```sh
# Read inputs from stdin and write the result to stdout
cargo run --release --bin 3 -- 2
```

## Directories

- `in/` : Input files
  - If `in/N-2` doesn't exist, `in/N-1` is used for the answer for part 2
  - If `in/N-s2` doesn't exist, `in/N-s1` is used for the sample of part 2
- `ans/` : Expected answer files
- `out/` : Actual output files
- `diff/` : Diff of expected/actual outputs

## Files

- `*/N-1` : Part 1 for day N
- `*/N-2` : Part 2 for day N
- `*/N-s1` : Sample of part 1 for day N
- `*/N-s2` : Sample of part 2 for day N

## Fetch input files automatically

You can put your input files in `in/` directory. Alternatively, `Makefile` provides a rule to download
them via `curl` command. To run it, save your cookie session in `AOC_SESSION` environment variable and
run `make in/N-1` (for day N input).

```sh
# Find your cookie session via Chrome DevTools
export AOC_SESSION=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
# Download the input file for day 4
make in/4-1
```

[watchexec]: https://github.com/watchexec/watchexec
