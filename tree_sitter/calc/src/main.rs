use anyhow::{bail, Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use tree_sitter::{InputEdit, Language, Node, Parser, Range, Tree};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

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

/*
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
*/

enum Edit {
    Char(char),
    Del,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    source: String,
    sexp: String,
    parser: Parser,
    tree: Option<Tree>,
}

impl App {
    fn new() -> Result<Self> {
        let lang = tree_sitter_calc::language();
        let mut parser = Parser::new();
        parser.set_language(lang)?;
        Ok(App {
            source: String::new(),
            sexp: String::new(),
            parser,
            tree: None,
        })
    }

    fn update_sexp(&mut self) {
        self.sexp = if let Some(tree) = &self.tree {
            tree.root_node().to_sexp()
        } else {
            "Could not parse input (Parser::parse returned None)".to_string()
        };
    }

    fn reparse(&mut self) -> bool {
        // Grammar always requires \n at end of statement
        self.source.push('\n');
        let tree = self.parser.parse(&self.source, self.tree.as_ref());
        self.source.pop(); // pop \n

        let changed = match (self.tree.as_ref(), tree.as_ref()) {
            (Some(before), Some(after)) => before.changed_ranges(after).next().is_some(),
            (None, None) => false,
            _ => true,
        };

        self.tree = tree;

        changed
    }

    fn calc(&mut self, edit: Edit) {
        match edit {
            Edit::Char(c) => {
                if let Some(tree) = &mut self.tree {
                    let node = tree.root_node();
                    let old_pos = node.end_position();
                    let new_pos = {
                        let mut p = old_pos;
                        p.column += 1;
                        p
                    };
                    let old = self.source.len();
                    let new = old + c.len_utf8();
                    let edit = InputEdit {
                        start_byte: old,
                        old_end_byte: old,
                        new_end_byte: new,
                        start_position: old_pos,
                        old_end_position: old_pos,
                        new_end_position: new_pos,
                    };
                    tree.edit(&edit);
                }
                self.source.push(c);
                if self.reparse() {
                    self.update_sexp();
                }
            }
            Edit::Del => {
                if let Some(c) = self.source.pop() {
                    if let Some(tree) = &mut self.tree {
                        let node = tree.root_node();
                        let old_pos = node.end_position();
                        let new_pos = {
                            let mut p = old_pos;
                            p.column = p.column.saturating_sub(1);
                            p
                        };
                        let new = self.source.len();
                        let old = new + c.len_utf8();
                        let edit = InputEdit {
                            start_byte: old,
                            old_end_byte: old,
                            new_end_byte: new,
                            start_position: old_pos,
                            old_end_position: old_pos,
                            new_end_position: new_pos,
                        };
                        tree.edit(&edit);
                    }
                    if self.reparse() {
                        self.update_sexp();
                    }
                }
            }
        }
    }

    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let msg = vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit"),
        ];
        let mut text = Text::from(Spans::from(msg));
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, layout[0]);

        let input = Paragraph::new(self.source.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, layout[1]);
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        f.set_cursor(
            // Put cursor past the end of the input text
            layout[1].x + self.source.width() as u16 + 1,
            // Move one line down, from the border to the input line
            layout[1].y + 1,
        );

        let sexp: Vec<ListItem> = self
            .sexp
            .lines()
            .map(|line| {
                let content = vec![Spans::from(Span::raw(line))];
                ListItem::new(content)
            })
            .collect();
        let sexp =
            List::new(sexp).block(Block::default().borders(Borders::ALL).title("S-expression"));
        f.render_widget(sexp, layout[2]);
    }

    fn run<B: Backend>(mut self, term: &mut Terminal<B>) -> Result<()> {
        loop {
            term.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => self.calc(Edit::Char(c)),
                    KeyCode::Backspace => self.calc(Edit::Del),
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    // create app and run it
    let app = App::new()?;
    let res = app.run(&mut term);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    res
}
