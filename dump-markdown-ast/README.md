This is a small tool to dump a Markdown AST as JSON.

## How to build

```sh
cargo build --release
```

## How to run

To generate JSON output:

```sh
./target/release/dump-markdown-ast json input_example.md
```

To generate HTML for comparison:

```sh
./target/release/dump-markdown-ast html input_example.md
```

To dump paraser events for debugging:

```sh
./target/release/dump-markdown-ast events input_example.md
```

[`output_example.json`](output_example.json) was generated from [`input_example.json`](input_example.md).

## Performance

```
Benchmark 1: ./rust-cmark html test.md
  Time (mean ± σ):       2.0 ms ±   0.4 ms    [User: 0.8 ms, System: 0.5 ms]
  Range (min … max):     1.8 ms …   8.8 ms    663 runs

Benchmark 2: ./rust-cmark json test.md
  Time (mean ± σ):       2.2 ms ±   0.3 ms    [User: 1.0 ms, System: 0.5 ms]
  Range (min … max):     2.0 ms …   4.9 ms    632 runs
```
