use std::io::{self, Stdin, Stdout, Write};

enum ReplInput<'a> {
    Meta(&'a str),
    Statement(&'a str),
}

struct Repl {
    buffer: String,
    stdout: Stdout,
    stdin: Stdin,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            stdout: io::stdout(),
            stdin: io::stdin(),
        }
    }
}

impl Repl {
    fn prompt(&mut self) -> io::Result<ReplInput<'_>> {
        self.stdout.write_all(b"db > ")?;
        self.stdout.flush()?;
        self.buffer.clear();
        self.stdin.read_line(&mut self.buffer)?;
        if self.buffer.ends_with('\n') {
            self.buffer.pop();
        }
        if let Some(meta) = self.buffer.strip_prefix('.') {
            Ok(ReplInput::Meta(meta))
        } else {
            Ok(ReplInput::Statement(self.buffer.as_str()))
        }
    }
}

#[derive(Debug)]
enum MetaCommand {
    Exit,
}

impl MetaCommand {
    fn parse(input: &str) -> Option<Self> {
        if input == "exit" {
            Some(Self::Exit)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Statement {
    Insert,
    Select,
}

impl Statement {
    fn parse(input: &str) -> Option<Self> {
        match input.split_whitespace().next()? {
            "insert" => Some(Self::Insert),
            "select" => Some(Self::Select),
            _ => None,
        }
    }

    fn execute(self) {
        println!("TODO: {self:?}");
    }
}

fn main() -> io::Result<()> {
    let mut repl = Repl::default();

    loop {
        match repl.prompt()? {
            ReplInput::Meta(input) => {
                let Some(cmd) = MetaCommand::parse(input) else {
                    println!("Unrecognized meta command: {input:?}");
                    continue;
                };
                match cmd {
                    MetaCommand::Exit => break,
                }
            }
            ReplInput::Statement(input) => {
                let Some(statement) = Statement::parse(input) else {
                    println!("Unrecognized statement: {input:?}");
                    continue;
                };
                statement.execute();
                println!("Executed.");
            }
        }
    }

    Ok(())
}
