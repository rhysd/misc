use anyhow::{bail, Context, Result};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;
use std::io::Write;
use std::str::Chars;
use tree_sitter::{InputEdit, Language, Node, Parser, Tree};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::{Frame, Terminal};
use unicode_width::UnicodeWidthStr;

struct NodeKinds {
    binary_expression: u16,
    unary_expression: u16,
    constant: u16,
}

impl NodeKinds {
    fn new(lang: Language) -> Self {
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
    fn new(lang: Language) -> Self {
        Self {
            operator: lang.field_id_for_name("operator").unwrap(),
            operand: lang.field_id_for_name("operand").unwrap(),
            left: lang.field_id_for_name("left").unwrap(),
            right: lang.field_id_for_name("right").unwrap(),
        }
    }
}

struct Interpreter<'a> {
    kinds: &'a NodeKinds,
    fields: &'a NodeFields,
    src: &'a str,
}

impl<'a> Interpreter<'a> {
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
                let s = operator.start_position();
                bail!("divide by zero at line:{},col:{}", s.row + 1, s.column + 1)
            }
            "/" => Ok(left / right),
            t => {
                let s = operator.start_position();
                bail!(
                    "unexpected binary operator '{}' at line:{},col:{}",
                    t,
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
                let s = operator.start_position();
                bail!(
                    "unexpected unary operator '{}' at line:{},col:{}",
                    t,
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
                "could not parse constant '{}' as number at line:{},col:{}",
                tok,
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
                "Cannot eval node '{}' at line:{},col:{}",
                node.kind(),
                s.row + 1,
                s.column + 1,
            )
        }
    }
}

pub struct SexpFormatter<'a> {
    src: &'a str,
    cur: Chars<'a>,
    indent: usize,
}

impl<'a> SexpFormatter<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            cur: src.chars(),
            indent: 0,
        }
    }
}

impl<'a> Iterator for SexpFormatter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.cur.next() {
            if c == '(' {
                break;
            }
        }

        let mut next_indent = self.indent + 1;
        while let Some(c) = self.cur.next() {
            match c {
                '(' => {
                    let mut idx = self.src.len() - self.cur.as_str().len() - 1;
                    let mut line = self.src[..idx].trim_end();
                    if line.ends_with(':') {
                        if let Some(i) = line
                            .char_indices()
                            .rev()
                            .find_map(|(i, c)| (c == ' ').then(|| i))
                        {
                            idx = i;
                            line = line[..idx].trim_end();
                        }
                    }

                    self.src = &self.src[idx..];
                    self.cur = self.src.chars();
                    let indent = self.indent;
                    self.indent = next_indent;
                    return Some((indent, line));
                }
                ')' => {
                    next_indent = next_indent.saturating_sub(1);
                }
                _ => continue,
            }
        }

        if !self.src.is_empty() {
            let src = self.src;
            self.src = "";
            self.cur = "".chars();
            return Some((self.indent, src));
        }

        None
    }
}

enum Edit {
    Char(char),
    Del,
}

#[repr(transparent)]
struct ResultText(std::result::Result<String, String>);

impl Default for ResultText {
    fn default() -> Self {
        ResultText(Ok(String::new()))
    }
}

impl ResultText {
    fn ok(msg: String) -> Self {
        Self(Ok(msg))
    }

    fn err(msg: String) -> Self {
        Self(Err(msg))
    }

    fn text(&self) -> &str {
        match self.0.as_ref() {
            Ok(t) => t,
            Err(t) => t,
        }
    }

    fn style(&self) -> Style {
        if self.0.is_err() {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }
    }
}

struct App {
    kinds: NodeKinds,
    fields: NodeFields,
    source: String,
    sexp: ResultText,
    parser: Parser,
    tree: Option<Tree>,
    result: ResultText,
}

impl App {
    fn new() -> Result<Self> {
        let lang = tree_sitter_calc::language();
        let mut parser = Parser::new();
        parser.set_language(lang)?;
        Ok(App {
            kinds: NodeKinds::new(lang),
            fields: NodeFields::new(lang),
            source: String::new(),
            sexp: ResultText::default(),
            parser,
            tree: None,
            result: ResultText::default(),
        })
    }

    fn inspect_sexp(&mut self) {
        self.sexp = if let Some(tree) = &self.tree {
            let root = tree.root_node();
            let sexp = root.to_sexp();
            if root.has_error() {
                ResultText::err(sexp)
            } else {
                ResultText::ok(sexp)
            }
        } else {
            ResultText::err("Could not parse input (Parser::parse returned None)".to_string())
        };
    }

    // Note: The result must be updated even if tree structure did not change. For example, modifying
    // `1 + 2` to `1 + 23` does not change its structure but the result changes.
    fn eval_tree(&mut self) {
        self.result = if let Some(tree) = &self.tree {
            let interpreter = Interpreter {
                kinds: &self.kinds,
                fields: &self.fields,
                src: &self.source,
            };
            let root = tree.root_node();
            if root.child_count() > 0 {
                match interpreter.eval(&root) {
                    Ok(ret) => ResultText::ok(ret.to_string()),
                    Err(err) => ResultText::err(format!("{}", err)),
                }
            } else {
                ResultText::default()
            }
        } else {
            ResultText::default()
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
                    self.inspect_sexp();
                }
                self.eval_tree();
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
                        self.inspect_sexp();
                    }
                    self.eval_tree();
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
        let text = Text::from(Spans::from(msg));
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, layout[0]);

        let input = Paragraph::new(self.source.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, layout[1]);
        f.set_cursor(
            layout[1].x + self.source.width() as u16 + 1,
            layout[1].y + 1,
        );

        let result = Paragraph::new(self.result.text())
            .style(self.result.style())
            .block(Block::default().borders(Borders::ALL).title("Result"));
        f.render_widget(result, layout[2]);

        let items = SexpFormatter::new(self.sexp.text())
            .map(|(indent, line)| {
                let mut spans = Vec::with_capacity(indent + 1);
                for _ in 0..indent {
                    spans.push(Span::raw("  "));
                }
                spans.push(Span::raw(line));
                ListItem::new(Spans::from(spans))
            })
            .collect::<Vec<_>>();
        let sexp = List::new(items)
            .style(self.sexp.style())
            .block(Block::default().borders(Borders::ALL).title("S-expression"));
        f.render_widget(sexp, layout[3]);
    }

    fn run<B: Backend>(mut self, term: &mut Terminal<B>) -> Result<()> {
        loop {
            term.draw(|f| self.render(f))?;

            if let Event::Key(key) = crossterm::event::read()? {
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

struct RawMode<W: Write> {
    term: Terminal<CrosstermBackend<W>>,
}

impl<W: Write> RawMode<W> {
    fn new(mut w: W) -> Result<Self> {
        // setup terminal
        enable_raw_mode()?;
        crossterm::execute!(w, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(w);
        let term = Terminal::new(backend)?;
        Ok(Self { term })
    }
}

impl<W: Write> Drop for RawMode<W> {
    fn drop(&mut self) {
        // restore terminal
        disable_raw_mode().expect("disable raw mode");
        crossterm::execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("leave alternate screen");
        self.term.show_cursor().expect("restore cursor");
    }
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut raw_mode = RawMode::new(stdout.lock())?;
    // create app and run it
    let app = App::new()?;
    app.run(&mut raw_mode.term)
}
