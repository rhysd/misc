use anyhow::Result;
use std::env;
use std::fs;
use tree_sitter::{Language, Node, Parser, Tree, TreeCursor};

extern "C" {
    fn tree_sitter_rust() -> Language;
}

fn print_tree(tree: &Tree) {
    fn print_cursor(cursor: &mut TreeCursor<'_>, indent: u16) {
        for _ in 0..indent {
            print!(". ");
        }

        if let Some(field) = cursor.field_name() {
            print!("{} ", field);
        }

        let node = cursor.node();
        let start = node.start_position();
        let end = node.end_position();
        print!(
            "{} ({}:{}-{}:{})",
            node.kind(),
            start.row + 1,
            start.column + 1,
            end.row + 1,
            end.column + 1,
        );
        if node.is_named() {
            print!(" NAMED");
        }
        if node.is_error() {
            print!(" ERROR");
        }

        println!();

        if cursor.goto_first_child() {
            let indent = indent + 1;
            print_cursor(cursor, indent);
            while cursor.goto_next_sibling() {
                print_cursor(cursor, indent);
            }
            cursor.goto_parent();
        }
    }

    let mut cursor = tree.walk();
    print_cursor(&mut cursor, 0);
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
    println!("Sexp:\n{}", tree.root_node().to_sexp());

    println!("\nTree cursor:");
    print_tree(&tree);

    Ok(())
}
