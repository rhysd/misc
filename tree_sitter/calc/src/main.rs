use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use tree_sitter::{Language, Node, Parser};

fn read_input(path: Option<&str>) -> Result<String> {
    if let Some(path) = path {
        fs::read_to_string(&path).with_context(|| format!("could not read file from '{}'", &path))
    } else {
        let stdin = io::stdin();
        let mut buf = String::new();
        stdin
            .lock()
            .read_to_string(&mut buf)
            .context("could not read from stdin")?;
        Ok(buf)
    }
}

struct NodeKinds {
    binary_expression: u16,
    unary_expression: u16,
    constant: u16,
}

impl NodeKinds {
    fn new(lang: &Language) -> Self {
        Self {
            binary_expression: lang.id_for_node_kind("binary_expression", true),
            unary_expression: lang.id_for_node_kind("unary_expression", true),
            constant: lang.id_for_node_kind("constant", true),
        }
    }
}

struct NodeFields {
    operator: u16,
    operand: u16,
    left: u16,
    right: u16,
}

impl NodeFields {
    fn new(lang: &Language) -> Self {
        Self {
            operator: lang.field_id_for_name("operator").unwrap(),
            operand: lang.field_id_for_name("operand").unwrap(),
            left: lang.field_id_for_name("left").unwrap(),
            right: lang.field_id_for_name("right").unwrap(),
        }
    }
}

struct Interpreter<'a> {
    kinds: NodeKinds,
    fields: NodeFields,
    file: &'a str,
    src: &'a str,
}

impl<'a> Interpreter<'a> {
    fn new(lang: &Language, file: &'a str, src: &'a str) -> Self {
        Self {
            kinds: NodeKinds::new(lang),
            fields: NodeFields::new(lang),
            file,
            src,
        }
    }

    fn token(&self, node: &Node) -> &'a str {
        &self.src[node.start_byte()..node.end_byte()]
    }

    fn eval_bin_expr(&self, node: &Node) -> Result<f64> {
        let operator = node.child_by_field_id(self.fields.operator).unwrap();
        let left = node.child_by_field_id(self.fields.left).unwrap();
        let left = self.eval(&left)?;
        let right = node.child_by_field_id(self.fields.right).unwrap();
        let right = self.eval(&right)?;
        match self.token(&operator) {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" if right == 0.0 => {
                let s = node.start_position();
                bail!(
                    "divide by zero at {}:{}:{}",
                    self.file,
                    s.row + 1,
                    s.column + 1
                )
            }
            "/" => Ok(left / right),
            t => {
                let s = node.start_position();
                bail!(
                    "unexpected binary operator '{}' at {}:{}:{}",
                    t,
                    self.file,
                    s.row + 1,
                    s.column + 1
                )
            }
        }
    }

    fn eval_unary_expr(&self, node: &Node) -> Result<f64> {
        let operator = node.child_by_field_id(self.fields.operator).unwrap();
        let operand = node.child_by_field_id(self.fields.operand).unwrap();
        let operand = self.eval(&operand)?;
        match self.token(&operator) {
            "+" => Ok(operand),
            "-" => Ok(-operand),
            t => {
                let s = node.start_position();
                bail!(
                    "unexpected unary operator '{}' at {}:{}:{}",
                    t,
                    self.file,
                    s.row + 1,
                    s.column + 1
                )
            }
        }
    }

    fn eval_const(&self, node: &Node) -> Result<f64> {
        let tok = self.token(node);
        tok.parse().with_context(|| {
            let s = node.start_position();
            format!(
                "could not parse constant '{}' as number at {}:{}:{}",
                tok,
                self.file,
                s.row + 1,
                s.column + 1,
            )
        })
    }

    fn eval(&self, node: &Node) -> Result<f64> {
        let kind = node.kind_id();
        if kind == self.kinds.binary_expression {
            self.eval_bin_expr(node)
        } else if kind == self.kinds.unary_expression {
            self.eval_unary_expr(node)
        } else if kind == self.kinds.constant {
            self.eval_const(node)
        } else if let Some(node) = node.child(0) {
            self.eval(&node)
        } else {
            let s = node.start_position();
            bail!(
                "Cannot eval node '{}' at {}:{}:{}",
                node.kind(),
                self.file,
                s.row + 1,
                s.column + 1,
            )
        }
    }
}

fn main() -> Result<()> {
    let file = env::args().skip(1).next();
    let file = file.as_ref().map(String::as_str);
    let input = read_input(file)?;
    let file = file.unwrap_or("<stdin>");
    let lang = tree_sitter_calc::language();
    let interpreter = Interpreter::new(&lang, &file, &input);

    let mut parser = Parser::new();
    parser.set_language(lang)?;
    let tree = parser
        .parse(&input, None)
        .ok_or_else(|| anyhow::anyhow!("could not parse {}", &file))?;

    let ret = interpreter.eval(&tree.root_node())?;
    println!("{}", ret);
    Ok(())
}
