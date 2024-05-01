use std::array;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};
use std::iter;
use std::mem;
use std::path::Path;
use std::slice;
use std::str;

enum Error {
    Unknown(String),
    Syntax(&'static str),
    TableFull,
    StringTooLong(u32),
    PageOutOfBounds(u32),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(cmd) => write!(f, "Unrecognized command: {cmd:?}"),
            Self::Syntax(usage) => write!(f, "Syntax error: {usage:?}"),
            Self::TableFull => write!(f, "Table is full"),
            Self::StringTooLong(max) => write!(f, "String length exceeds max length {max}"),
            Self::PageOutOfBounds(idx) => write!(f, "Page index {idx} is out of bounds"),
            Self::Io(inner) => write!(f, "I/O error: {inner}"),
        }
    }
}

type Result<T> = std::result::Result<T, Box<Error>>;

impl Error {
    #[inline]
    fn err<T>(self) -> Result<T> {
        Err(Box::new(self))
    }
}

impl From<io::Error> for Box<Error> {
    fn from(err: io::Error) -> Self {
        Box::new(Error::Io(err))
    }
}

// TODO: This must be allocated on heap since table row structure will be dynamic
#[repr(transparent)]
struct InlineString<const N: usize>([u8; N]);

impl<const N: usize> Default for InlineString<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> InlineString<N> {
    fn new(s: &str) -> Result<Self> {
        let s = s.as_bytes();
        if s.len() > N {
            return Error::StringTooLong(N as u32).err();
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

#[repr(C)]
#[repr(packed(1))]
#[derive(Default)]
struct SerializedRow {
    id: u32,                     // integer
    user_name: InlineString<32>, // varchar(32)
    email: InlineString<255>,    // varchar(255)
}

impl SerializedRow {
    const SIZE: usize = mem::size_of::<SerializedRow>();

    fn serialize(row: &Row<'_>) -> Result<Self> {
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
const _: () = assert!(SerializedRow::SIZE == 291);

#[derive(Default)]
struct Page(Vec<SerializedRow>);

impl Page {
    const PAGE_SIZE: usize = 4096;
    const ROWS_PER_PAGE: usize = Self::PAGE_SIZE / SerializedRow::SIZE;

    fn read_from<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut rows: Vec<_> =
            iter::repeat_with(SerializedRow::default).take(Self::ROWS_PER_PAGE).collect();
        let rows_array: &mut [SerializedRow; Self::ROWS_PER_PAGE] =
            rows.as_mut_slice().try_into().unwrap();
        // Safety: This operation is safe because any bit pattern is valid for `SerializedRow` instance.
        let bytes = unsafe {
            mem::transmute::<
                &mut [SerializedRow; Self::ROWS_PER_PAGE],
                &mut [u8; mem::size_of::<[SerializedRow; Self::ROWS_PER_PAGE]>()],
            >(rows_array)
        };
        // `read_exact` is not available since the last page may not be full.
        let _ = reader.read(bytes)?;
        Ok(Self(rows))
    }

    fn write_to<W: Write>(&self, mut writer: W, num_rows: usize) -> io::Result<()> {
        assert!(num_rows <= self.0.len(), "{} is out of bounds {}", num_rows, self.0.len());
        let src_ptr = self.0.as_ptr();
        // Safety: This operation is safe because the length of array pointed by `src_ptr` and the
        // length of `bytes` are the same.
        let bytes = unsafe {
            let bytes_ptr = mem::transmute::<_, *const u8>(src_ptr);
            slice::from_raw_parts(bytes_ptr, num_rows * SerializedRow::SIZE)
        };
        writer.write_all(bytes)
    }

    fn row_at(&mut self, idx: usize) -> &mut SerializedRow {
        // `self.0` must be filled with zero-initialized SerializedRow instances. Otherwise, this
        // page read from a file will contain uninitialized instances. Accessing such instances
        // causes UB.
        if self.0.is_empty() {
            self.0.reserve_exact(Self::ROWS_PER_PAGE);
            self.0.extend(iter::repeat_with(SerializedRow::default).take(Self::ROWS_PER_PAGE));
        }
        &mut self.0[idx]
    }

    fn is_cached(&self) -> bool {
        !self.0.is_empty()
    }
}

struct Pager {
    file: Option<File>,
    file_len: u64,
    pages: [Page; Self::MAX_PAGES],
}

impl Default for Pager {
    fn default() -> Self {
        let pages = array::from_fn(|_| Page::default());
        Self { file: None, file_len: 0, pages }
    }
}

impl Pager {
    const MAX_PAGES: usize = 100;

    fn open(path: &Path) -> io::Result<Self> {
        let file =
            File::options().read(true).write(true).create(true).truncate(false).open(path)?;
        let file_len = file.metadata()?.len();
        let pages = std::array::from_fn(|_| Page::default());
        Ok(Self { file: Some(file), file_len, pages })
    }

    fn page_at(&mut self, page_num: usize) -> Result<&mut Page> {
        let Some(page) = self.pages.get_mut(page_num) else {
            return Error::PageOutOfBounds(page_num as u32).err();
        };
        if !page.is_cached() {
            if let Some(file) = &mut self.file {
                let num_pages = self.file_len as usize / Page::PAGE_SIZE
                    + usize::from(self.file_len as usize % Page::PAGE_SIZE > 0);
                if page_num <= num_pages {
                    let offset = page_num * Page::PAGE_SIZE;
                    file.seek(SeekFrom::Start(offset as u64))?;
                    *page = Page::read_from(file)?;
                }
            }
        }
        Ok(page)
    }

    fn flush(&mut self, page_num: usize, rows: usize) -> io::Result<()> {
        let Some(file) = &mut self.file else {
            return Ok(());
        };

        let Some(page) = self.pages.get_mut(page_num) else {
            return Ok(());
        };

        if !page.is_cached() {
            return Ok(());
        }

        let offset = page_num * Page::PAGE_SIZE;
        file.seek(SeekFrom::Start(offset as u64))?;
        page.write_to(file, rows)?;

        *page = Page::default();
        Ok(())
    }
}

#[derive(Default)]
struct Table {
    num_rows: u32,
    pager: Pager,
}

impl Table {
    const MAX_ROWS: usize = Page::ROWS_PER_PAGE * Pager::MAX_PAGES;

    fn open(path: &Path) -> io::Result<Self> {
        let pager = Pager::open(path)?;
        let num_rows = (pager.file_len / SerializedRow::SIZE as u64) as u32;
        Ok(Self { num_rows, pager })
    }

    fn row_slot(&mut self, row_num: u32) -> Result<&mut SerializedRow> {
        let page_num = row_num as usize / Page::ROWS_PER_PAGE;
        let page = self.pager.page_at(page_num)?;
        let row = page.row_at(row_num as usize % Page::ROWS_PER_PAGE);
        Ok(row)
    }

    fn close(&mut self) -> io::Result<()> {
        let num_full_pages = self.num_rows as usize / Page::ROWS_PER_PAGE;
        for idx in 0..num_full_pages {
            self.pager.flush(idx, Page::ROWS_PER_PAGE)?;
        }

        let num_additional_rows = self.num_rows as usize % Page::ROWS_PER_PAGE;
        if num_additional_rows > 0 {
            self.pager.flush(num_full_pages, num_additional_rows)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Statement<'input> {
    Insert(Row<'input>),
    Select,
}

impl<'input> Statement<'input> {
    fn prepare(input: &'input str) -> Result<Self> {
        let mut tokens = input.split_whitespace();
        let Some(cmd) = tokens.next() else {
            return Error::Unknown(String::new()).err();
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
                parse().ok_or_else(|| Box::new(Error::Syntax("insert {id} {user} {email}")))
            }
            "select" => Ok(Self::Select),
            c => Error::Unknown(c.to_string()).err(),
        }
    }

    fn execute<W: Write>(&self, table: &mut Table, mut w: W) -> Result<()> {
        match self {
            Self::Insert(row) => {
                if table.num_rows as usize >= Table::MAX_ROWS {
                    return Error::TableFull.err();
                }

                let slot = table.row_slot(table.num_rows)?;
                *slot = SerializedRow::serialize(row)?;
                table.num_rows += 1;

                Ok(())
            }
            Self::Select => {
                for i in 0..table.num_rows {
                    let row = table.row_slot(i)?.deserialize();
                    writeln!(w, "{row}").unwrap();
                }
                Ok(())
            }
        }
    }
}

fn repl<R: BufRead, W: Write>(mut stdin: R, mut stdout: W, mut table: Table) -> io::Result<()> {
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
                        table.close()?;
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
    dbg!(Page::ROWS_PER_PAGE);
    let table = if let Some(path) = env::args_os().nth(1) {
        Table::open(Path::new(&path))?
    } else {
        Table::default()
    };
    repl(io::stdin().lock(), io::stdout(), table)
}

#[cfg(test)]
#[rustfmt::skip::macros(format)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::fmt::Write as _;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use tempfile::TempDir;

    #[track_caller]
    fn run_test_with_table(stdin: impl AsRef<str>, table: Table) -> io::Result<String> {
        let stdin = stdin.as_ref();
        let mut stdout = Vec::<u8>::new();
        let mut stdin = BufReader::new(stdin.as_bytes());
        repl(&mut stdin, &mut stdout, table)?;
        Ok(String::from_utf8(stdout).unwrap())
    }

    #[track_caller]
    fn run_test(stdin: impl AsRef<str>) -> io::Result<String> {
        run_test_with_table(stdin.as_ref(), Table::default())
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
    fn rows_more_than_one_page() {
        let mut s = String::new();
        for i in 1..=15 {
            writeln!(s, "insert {i} user{i} person{i}@example.com").unwrap();
        }
        s.push_str("select\n");
        s.push_str(".exit\n");
        assert_snapshot!(run_test(s).unwrap());
    }

    #[test]
    fn table_full_error() {
        let mut s = String::new();
        for i in 1..=1401 {
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

    fn temp_dir() -> &'static TempDir {
        static TEMP_DIR: OnceLock<TempDir> = OnceLock::new();
        TEMP_DIR.get_or_init(|| tempfile::tempdir().unwrap())
    }

    fn temp_file(name: &str) -> PathBuf {
        temp_dir().path().join(name)
    }

    #[track_caller]
    fn run_test_persistent(name: &str, stdin: impl AsRef<str>) -> io::Result<String> {
        let path = temp_file(name);
        let table = Table::open(&path)?;
        run_test_with_table(stdin.as_ref(), table)
    }

    #[test]
    fn persistent_single_row() {
        let mut output = run_test_persistent(
            "single_row.db",
            "insert 1 foo foo@example.com
            .exit",
        )
        .unwrap();
        output += &run_test_persistent(
            "single_row.db",
            "select
            .exit",
        )
        .unwrap();
        assert_snapshot!(output);
    }

    #[test]
    fn persistent_single_page() {
        let mut input = String::new();
        for i in 1..=14 {
            writeln!(input, "insert {i} user{i} user{i}@example.com").unwrap();
        }
        input.push_str(".exit\n");
        let mut output = run_test_persistent("single_page.db", input).unwrap();
        output += &run_test_persistent(
            "single_page.db",
            "select
            .exit",
        )
        .unwrap();
        assert_snapshot!(output);
    }

    #[test]
    fn persistent_single_and_half_page() {
        let mut input = String::new();
        for i in 1..=21 {
            writeln!(input, "insert {i} user{i} user{i}@example.com").unwrap();
        }
        input.push_str(".exit\n");
        let mut output = run_test_persistent("single_half_page.db", input).unwrap();
        output += &run_test_persistent(
            "single_half_page.db",
            "select
            .exit",
        )
        .unwrap();
        assert_snapshot!(output);
    }

    #[test]
    fn persistent_no_row() {
        let mut output = run_test_persistent("nothing.db", ".exit").unwrap();
        output += &run_test_persistent(
            "nothing.db",
            "select
            .exit",
        )
        .unwrap();
        assert_snapshot!(output);
    }
}
