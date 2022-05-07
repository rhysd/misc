use std::path::Path;

fn main() {
    let src_dir = Path::new("tree-sitter-rust").join("src");
    let sources = &[src_dir.join("parser.c"), src_dir.join("scanner.c")];

    cc::Build::new()
        .files(sources)
        .include(src_dir.join("tree_sitter"))
        .compile("tree-sitter-rust");

    for src in sources {
        println!("cargo:rerun-if-changed={}", src.to_str().unwrap());
    }
}
