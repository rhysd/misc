use std::fmt;
use std::io::{self, Stdin, Stdout, Write};
use std::iter;
use std::mem;
use std::str;

// TODO: This must be allocated on heap since table row structure will be dynamic
struct InlineString<const N: usize>([u8; N]);

impl<const N: usize> Default for InlineString<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> InlineString<N> {
    fn new(s: &str) -> Self {
        let s = s.as_bytes();
        assert!(s.len() <= N);

        Self(std::array::from_fn(|i| s.get(i).copied().unwrap_or(0)))
    }

    fn as_str(&self) -> &'_ str {
        let end = self.0.iter().take_while(|&&b| b != 0).count();
        str::from_utf8(&self.0[..end]).unwrap()
    }
}

enum ReplInput<'input> {
    Meta(&'input str),
    Statement(&'input str),
}

struct Repl {
    buffer: String,
    stdout: Stdout,
    stdin: Stdin,
}

impl Default for Repl {
    fn default() -> Self {
        Self { buffer: String::new(), stdout: io::stdout(), stdin: io::stdin() }
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

enum ParseError<'input> {
    Unknown(&'input str),
    Syntax(&'static str),
}

impl<'input> fmt::Display for ParseError<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(cmd) => write!(f, "Unrecognized command: {cmd:?}"),
            Self::Syntax(usage) => write!(f, "Syntax error: {usage:?}"),
        }
    }
}

enum ExecuteError {
    TableFull,
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TableFull => write!(f, "Table is full"),
        }
    }
}

#[derive(Debug)]
struct Row<'a> {
    id: u32,
    user_name: &'a str,
    email: &'a str,
}

impl<'a> fmt::Display for Row<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:?}, {:?})", self.id, self.user_name, self.email)
    }
}

#[repr(packed(1))]
#[derive(Default)]
struct SerializedRow {
    id: u32,                     // integer
    user_name: InlineString<32>, // varchar(32)
    email: InlineString<255>,    // varchar(255)
}
const _: () = {
    assert!(mem::size_of::<SerializedRow>() == 291,);
};

impl SerializedRow {
    fn serialize(row: &Row<'_>) -> Self {
        Self {
            id: row.id,
            user_name: InlineString::new(row.user_name),
            email: InlineString::new(row.email),
        }
    }

    fn deserialize(&self) -> Row<'_> {
        Row { id: self.id, user_name: self.user_name.as_str(), email: self.email.as_str() }
    }
}

#[derive(Default)]
struct Page(Vec<SerializedRow>);

impl Page {
    const PAGE_SIZE: usize = 4096;
    const ROWS_PER_PAGE: usize = Self::PAGE_SIZE / mem::size_of::<SerializedRow>();

    fn row_at(&mut self, idx: usize) -> &mut SerializedRow {
        self.0.reserve_exact(Self::ROWS_PER_PAGE);
        if idx + 1 > self.0.len() {
            let diff = idx + 1 - self.0.len();
            self.0.extend(iter::repeat_with(SerializedRow::default).take(diff));
        }
        &mut self.0[idx as usize]
    }
}

struct Table {
    num_rows: u32,
    pages: [Page; Self::MAX_PAGES],
}

impl Default for Table {
    fn default() -> Self {
        Self { num_rows: 0, pages: std::array::from_fn(|_| Page::default()) }
    }
}

impl Table {
    const MAX_PAGES: usize = 100;
    const MAX_ROWS: usize = Page::ROWS_PER_PAGE * Self::MAX_PAGES;

    fn row_slot(&mut self, row_num: u32) -> &mut SerializedRow {
        let page_num = row_num as usize / Page::ROWS_PER_PAGE;
        let page = &mut self.pages[page_num];
        page.row_at(row_num as usize % Page::ROWS_PER_PAGE)
    }
}

#[derive(Debug)]
enum Statement<'input> {
    Insert(Row<'input>),
    Select,
}

impl<'input> Statement<'input> {
    fn parse(input: &'input str) -> Result<Self, ParseError<'input>> {
        let mut tokens = input.split_whitespace();
        let Some(cmd) = tokens.next() else {
            return Err(ParseError::Unknown(""));
        };
        match cmd {
            "insert" => {
                let mut parse = || {
                    let id = tokens.next()?.parse().ok()?;
                    let user_name = tokens.next()?;
                    let email = tokens.next()?;
                    let row = Row { id, user_name, email };
                    Some(Statement::Insert(row))
                };
                parse().ok_or_else(|| ParseError::Syntax("insert {id} {user} {email}"))
            }
            "select" => Ok(Self::Select),
            c => Err(ParseError::Unknown(c)),
        }
    }

    fn execute(&self, table: &mut Table) -> Result<(), ExecuteError> {
        match self {
            Self::Insert(row) => {
                if table.num_rows as usize >= Table::MAX_ROWS {
                    return Err(ExecuteError::TableFull);
                }

                let slot = table.row_slot(table.num_rows);
                *slot = SerializedRow::serialize(row);
                table.num_rows += 1;

                Ok(())
            }
            Self::Select => {
                for i in 0..table.num_rows {
                    let row = table.row_slot(i).deserialize();
                    println!("{row}");
                }
                Ok(())
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut table = Table::default();
    let mut repl = Repl::default();

    loop {
        match repl.prompt()? {
            ReplInput::Meta(input) => {
                let Some(cmd) = MetaCommand::parse(input) else {
                    eprintln!("Unrecognized meta command: {input:?}");
                    continue;
                };
                match cmd {
                    MetaCommand::Exit => break,
                }
            }
            ReplInput::Statement(input) => match Statement::parse(input) {
                Ok(statement) => match statement.execute(&mut table) {
                    Ok(()) => eprintln!("Executed."),
                    Err(err) => eprintln!("Error while executing {statement:?}: {err}"),
                },
                Err(err) => eprintln!("Error while executing {input:?}: {err}"),
            },
        }
    }

    Ok(())
}
