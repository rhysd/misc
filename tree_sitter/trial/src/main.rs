use anyhow::Result;
use std::env;
use std::fs;
use tree_sitter::{InputEdit, Language, Parser, Query, QueryCursor, Tree, TreeCursor};

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

    let mut tree = parser
        .parse(&source, None)
        .ok_or_else(|| anyhow::anyhow!("could not parse {}", &file))?;
    println!("Sexp:\n{}", tree.root_node().to_sexp());

    println!("\nTree cursor:");
    print_tree(&tree);

    println!("\nQuery (all function names):");

    let query = Query::new(
        lang,
        r#"(function_item "fn"
            name: (identifier) @func-name (parameters)
            body: (block)
        )"#,
    )?;

    let mut cursor = QueryCursor::new();
    let mut last_node = None;
    for mat in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
        assert_eq!(mat.captures.len(), 1);
        let node = mat.captures[0].node;
        let name = &source[node.start_byte()..node.end_byte()];
        println!("{:?}", name);
        last_node = Some(node);
    }
    let last_node = last_node.unwrap();

    println!("\nEdit:");
    let edit = {
        let name = "wanwan";
        InputEdit {
            start_byte: last_node.start_byte(),
            old_end_byte: last_node.end_byte(),
            new_end_byte: last_node.start_byte() + name.len(),
            start_position: last_node.start_position(),
            old_end_position: last_node.end_position(),
            new_end_position: {
                let mut p = last_node.start_position();
                p.column += name.len();
                p
            },
        }
    };
    tree.edit(&edit);

    let source = source.replace("fn main()", "fn wanwan()");
    let tree = parser
        .parse(&source, Some(&tree))
        .ok_or_else(|| anyhow::anyhow!("could not parse"))?;

    for mat in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
        assert_eq!(mat.captures.len(), 1);
        let node = mat.captures[0].node;
        let name = &source[node.start_byte()..node.end_byte()];
        println!("{:?}", name);
    }

    Ok(())
}
