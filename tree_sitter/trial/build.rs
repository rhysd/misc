use std::path::Path;

fn main() {
    let src = Path::new("tree-sitter-rust").join("src");

    cc::Build::new()
        .file(src.join("parser.c"))
        .file(src.join("scanner.c"))
        .include(src.join("tree_sitter"))
        .compile("tree-sitter-rust");
}
