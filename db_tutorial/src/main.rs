use std::fmt;
use std::io::{self, BufRead, Write};
use std::iter;
use std::mem;
use std::str;

enum Error<'input> {
    Unknown(&'input str),
    Syntax(&'static str),
    TableFull,
    StringTooLong(u32),
}

impl<'input> fmt::Display for Error<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(cmd) => write!(f, "Unrecognized command: {cmd:?}"),
            Self::Syntax(usage) => write!(f, "Syntax error: {usage:?}"),
            Self::TableFull => write!(f, "Table is full"),
            Self::StringTooLong(max) => write!(f, "String length exceeds max length {max}"),
        }
    }
}

type Result<'a, T> = std::result::Result<T, Error<'a>>;

// TODO: This must be allocated on heap since table row structure will be dynamic
struct InlineString<const N: usize>([u8; N]);

impl<const N: usize> Default for InlineString<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> InlineString<N> {
    fn new(s: &str) -> Result<'_, Self> {
        let s = s.as_bytes();
        if s.len() > N {
            return Err(Error::StringTooLong(N as u32));
        }

        Ok(Self(std::array::from_fn(|i| s.get(i).copied().unwrap_or(0))))
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

#[derive(Default)]
struct Prompt {
    buffer: String,
}

impl Prompt {
    fn input<R: BufRead, W: Write>(
        &mut self,
        mut stdin: R,
        mut stdout: W,
    ) -> io::Result<ReplInput<'_>> {
        stdout.write_all(b"db > ")?;
        stdout.flush()?;
        self.buffer.clear();
        stdin.read_line(&mut self.buffer)?;
        let line = self.buffer.trim();
        if let Some(meta) = line.strip_prefix('.') {
            Ok(ReplInput::Meta(meta))
        } else {
            Ok(ReplInput::Statement(line))
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
    fn serialize<'a>(row: &Row<'a>) -> Result<'a, Self> {
        Ok(Self {
            id: row.id,
            user_name: InlineString::new(row.user_name)?,
            email: InlineString::new(row.email)?,
        })
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
        &mut self.0[idx]
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
    fn prepare(input: &'input str) -> Result<'input, Self> {
        let mut tokens = input.split_whitespace();
        let Some(cmd) = tokens.next() else {
            return Err(Error::Unknown(""));
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
                parse().ok_or(Error::Syntax("insert {id} {user} {email}"))
            }
            "select" => Ok(Self::Select),
            c => Err(Error::Unknown(c)),
        }
    }

    fn execute<W: Write>(&self, table: &mut Table, mut w: W) -> Result<'input, ()> {
        match self {
            Self::Insert(row) => {
                if table.num_rows as usize >= Table::MAX_ROWS {
                    return Err(Error::TableFull);
                }

                let slot = table.row_slot(table.num_rows);
                *slot = SerializedRow::serialize(row)?;
                table.num_rows += 1;

                Ok(())
            }
            Self::Select => {
                for i in 0..table.num_rows {
                    let row = table.row_slot(i).deserialize();
                    writeln!(w, "{row}").unwrap();
                }
                Ok(())
            }
        }
    }
}

fn run<R: BufRead, W: Write>(mut stdin: R, mut stdout: W) -> io::Result<()> {
    let mut table = Table::default();
    let mut prompt = Prompt::default();

    loop {
        match prompt.input(&mut stdin, &mut stdout)? {
            ReplInput::Meta(input) => {
                let Some(cmd) = MetaCommand::parse(input) else {
                    writeln!(stdout, "Unrecognized meta command: {input:?}").unwrap();
                    continue;
                };
                match cmd {
                    MetaCommand::Exit => {
                        writeln!(stdout, "Bye.").unwrap();
                        break;
                    }
                }
            }
            ReplInput::Statement(input) => match Statement::prepare(input) {
                Ok(statement) => match statement.execute(&mut table, &mut stdout) {
                    Ok(()) => writeln!(stdout, "Executed: {input:?}").unwrap(),
                    Err(err) => {
                        writeln!(stdout, "Error while executing {statement:?}: {err}").unwrap();
                    }
                },
                Err(err) => writeln!(stdout, "Error while executing {input:?}: {err}").unwrap(),
            },
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    run(io::stdin().lock(), io::stdout())
}

#[cfg(test)]
#[rustfmt::skip::macros(format)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::fmt::Write as _;
    use std::io::BufReader;

    #[track_caller]
    fn run_test(stdin: impl AsRef<str>) -> io::Result<String> {
        let stdin = stdin.as_ref();
        let mut stdout = Vec::<u8>::new();
        let mut stdin = BufReader::new(stdin.as_bytes());
        run(&mut stdin, &mut stdout)?;
        Ok(String::from_utf8(stdout).unwrap())
    }

    #[test]
    fn single_row_insert_select() {
        let input = "\
            insert 1 user1 person@example.com
            select
            .exit
        ";
        assert_snapshot!(run_test(input).unwrap());
    }

    #[test]
    fn table_full_error() {
        let mut s = String::new();
        for i in 0..=1400 {
            writeln!(s, "insert {i} user{i} person{i}@example.com").unwrap();
        }
        s.push_str(".exit\n");
        assert_snapshot!(run_test(s).unwrap());
    }

    #[test]
    fn max_user_name_and_email() {
        let user = "a".repeat(32);
        let email = "a".repeat(255);
        let input = format!("\
            insert 1 {user} {email}
            select
            .exit
        ");
        assert_snapshot!(run_test(input).unwrap());
    }

    #[test]
    fn name_is_too_long() {
        let user = "a".repeat(33);
        let input = format!("\
            insert 1 {user} foo@example.com
            select
            .exit
        ");
        assert_snapshot!(run_test(input).unwrap());
    }

    #[test]
    fn email_is_too_long() {
        let email = "a".repeat(256);
        let input = format!("\
            insert 1 foo {email}
            select
            .exit
        ");
        assert_snapshot!(run_test(input).unwrap());
    }

    #[test]
    fn id_must_not_be_negative() {
        let input = "\
            insert -1 foo foo@example.com
            select
            .exit
        ";
        assert_snapshot!(run_test(input).unwrap());
    }
}
