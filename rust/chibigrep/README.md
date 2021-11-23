A minimal example for [ripgrep](https://github.com/BurntSushi/ripgrep) as a library for [Rust LT Online 5](https://rust.connpass.com/event/228732/).

## Build

```
cargo build --release
cp ./target/release/chibigrep /usr/local/bin/
```

## Usage

```
chibigrep pattern [path...]
```

## Example

```sh
# Search \w+ in all test.txt
chibigrep '\w+' ./test.txt

# Search \w+ in all files in ./dir1 and ./dir2 recursively
chibigrep '\w+' ./dir1 ./dir2

# Search \w+ in all files in the current directory recursively
chibigrep '\w+'
```

## License

Public domain.
