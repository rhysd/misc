Answers for https://adventofcode.com/2023

## Usage

Run tests:

```sh
make
```

[watchexec][] is useful:

```sh
watchexec make
```

Answer:

```sh
# Create answer for part 2 of day 3
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

[watchexec]: https://github.com/watchexec/watchexec
