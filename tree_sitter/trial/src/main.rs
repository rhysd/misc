use anyhow::Result;
use std::env;
use std::fs;
use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_rust() -> Language;
}

fn main() -> Result<()> {
    let lang = unsafe { tree_sitter_rust() };
    let mut parser = Parser::new();
    parser.set_language(lang)?;

    let file = env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow::anyhow!("source file must be specified at 1st argument"))?;
    if !file.ends_with(".rs") {
        anyhow::bail!("1st argument must be a Rust file");
    }
    let source = fs::read_to_string(&file)?;

    let tree = parser
        .parse(&source, None)
        .ok_or_else(|| anyhow::anyhow!("could not parse {}", &file))?;
    let root = tree.root_node();
    let sexp = root.to_sexp();
    println!("{}", sexp);
    Ok(())
}
