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
# Create answer for part 1 of day 1
make ans/1-1
```

Clean results:

```sh
make clean
```

## Directories

- `in/` : Input files
- `ans/` : Expected answer files
- `out/` : Actual output files
- `diff/` : Diff of expected/actual outputs

## Files

- `*/N-1` : Part 1 answer for day N
- `*/N-2` : Part 2 answer for day N
- `*/N-s1` : Sample of part 1 for day N
- `*/N-s2` : Sample of part 2 for day N

[watchexec]: https://github.com/watchexec/watchexec
