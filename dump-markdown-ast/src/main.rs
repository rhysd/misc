use pulldown_cmark::{
    html, Alignment, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Options, Parser, Tag,
};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::{BufWriter, Read, Write};

fn err(msg: impl AsRef<str>) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, msg.as_ref()))
}

fn parser(s: &str) -> Parser<'_, '_> {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_TABLES
            | Options::ENABLE_TASKLISTS,
    );
    Parser::new_ext(s, options)
}

fn html<W: Write>(s: &str, w: W) -> io::Result<()> {
    html::write_html(w, parser(s))
}

fn json<W: Write>(s: &str, w: W) -> io::Result<()> {
    let mut json = Json::new(w);
    json.begin()?;
    for event in parser(s) {
        json.push(event)?;
    }
    json.end()
}

fn events<W: Write>(s: &str, mut w: W) -> io::Result<()> {
    for (event, range) in parser(s).into_offset_iter() {
        writeln!(w, "{:?} ... [{:?}]", event, range)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    let program = args.next().unwrap();
    let Some(kind) = args.next() else {
        return err(format!("Usage: {} [html|json] FILE", program));
    };
    let source = if let Some(path) = args.next() {
        fs::read_to_string(path)?
    } else {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s)?;
        s
    };
    let stdout = BufWriter::new(io::stdout().lock());

    match kind.as_str() {
        "html" => html(&source, stdout),
        "json" => json(&source, stdout),
        "events" => events(&source, stdout),
        _ => err(format!("Usage: {} [html|json] FILE", program)),
    }
}

enum T {
    Head,
    Row,
}

struct Json<'a, W: Write> {
    w: W,
    table: T,
    is_start: bool,
    ids: HashMap<CowStr<'a>, usize>,
}

impl<'a, W: Write> Json<'a, W> {
    fn new(w: W) -> Self {
        Self {
            w,
            table: T::Head,
            is_start: true,
            ids: HashMap::new(),
        }
    }

    fn begin(&mut self) -> io::Result<()> {
        self.w(r#"["#)
    }

    fn end(&mut self) -> io::Result<()> {
        self.w(r#"]"#)
    }

    fn comma(&mut self) -> io::Result<()> {
        if !self.is_start {
            self.w(",")?;
        } else {
            self.is_start = false;
        }
        Ok(())
    }

    fn w(&mut self, s: &str) -> io::Result<()> {
        self.w.write_all(s.as_bytes())
    }

    fn s(&mut self, s: impl AsRef<str>) -> io::Result<()> {
        self.w("\"")?;
        for c in s.as_ref().chars() {
            match c {
                '\\' => self.w("\\\\")?,
                '\u{0008}' => self.w("\\b")?,
                '\u{000c}' => self.w("\\f")?,
                '\n' => self.w("\\n")?,
                '\r' => self.w("\\r")?,
                '\t' => self.w("\\t")?,
                '"' => self.w("\\\"")?,
                c if c.is_control() => write!(self.w, "\\u{:04x}", c as u32)?,
                c => write!(self.w, "{}", c)?,
            }
        }
        self.w("\"")
    }

    fn a(&mut self, a: Alignment) -> io::Result<()> {
        self.w(match a {
            Alignment::None => r#""none""#,
            Alignment::Left => r#""left""#,
            Alignment::Center => r#""center""#,
            Alignment::Right => r#""right""#,
        })
    }

    fn id(&mut self, name: CowStr<'a>) -> usize {
        let new = self.ids.len() + 1;
        *self.ids.entry(name).or_insert(new)
    }

    pub fn push(&mut self, event: Event<'a>) -> io::Result<()> {
        use Event::*;

        match event {
            Start(tag) => {
                self.comma()?;
                self.start_tag(tag)
            }
            End(tag) => self.end_tag(tag),
            Text(text) => {
                self.comma()?;
                self.w(r#"{"tag":"text","body":"#)?;
                self.s(&text)?;
                self.w("}")
            }
            Code(text) => {
                self.comma()?;
                self.w(r#"{"tag":"code","body":"#)?;
                self.s(&text)?;
                self.w("}")
            }
            Html(html) => {
                self.comma()?;
                self.w(r#"{"tag":"html","body":"#)?;
                self.s(&html)?;
                self.w("}")
            }
            SoftBreak => {
                self.comma()?;
                self.w(r#"{"tag":"text","body":"\n"}"#)
            }
            HardBreak => {
                self.comma()?;
                self.w(r#"{"tag":"br"}"#)
            }
            Rule => {
                self.comma()?;
                self.w(r#"{"tag":"hr"}"#)
            }
            FootnoteReference(name) => {
                self.comma()?;
                self.w(r#"{"tag":"footnote-ref","name":"#)?;
                self.s(&name)?;
                let id = self.id(name);
                write!(self.w, r#","id":{}}}"#, id)
            }
            TaskListMarker(checked) => {
                self.comma()?;
                write!(self.w, r#"{{"tag":"checkbox","checked":{}}}"#, checked)
            }
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        use Tag::*;
        match tag {
            Paragraph => self.w(r#"{"tag":"p","children":["#)?,
            Heading(level, id, _) => {
                let level: u8 = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                write!(self.w, r#"{{"tag":"h","level":{}"#, level)?;
                if let Some(id) = id {
                    self.w(r#","id":"#)?;
                    self.s(id)?;
                }
                self.w(r#","children":["#)?;
            }
            Table(alignments) => {
                self.w(r#"{"tag":"table","align":["#)?;
                let mut alignments = alignments.into_iter();
                if let Some(a) = alignments.next() {
                    self.a(a)?;
                }
                for a in alignments {
                    self.w(",")?;
                    self.a(a)?;
                }
                self.w(r#"],"children":["#)?;
            }
            TableHead => {
                self.table = T::Head;
                self.w(r#"{"tag":"thread","children":[{"tag":"tr","children":["#)?;
            }
            TableRow => {
                self.table = T::Row;
                self.w(r#"{"tag":"tr","children":["#)?;
            }
            TableCell => self.w(match self.table {
                T::Head => r#"{"tag":"th","children":["#,
                T::Row => r#"{"tag":"td","children":["#,
            })?,
            BlockQuote => self.w(r#"{"tag":"blockquote","children":["#)?,
            CodeBlock(info) => {
                self.w(r#"{"tag":"pre","children":[{"tag":"code""#)?;
                if let CodeBlockKind::Fenced(info) = info {
                    if let Some(lang) = info.split(' ').next() {
                        if !lang.is_empty() {
                            self.w(r#","lang":"#)?;
                            self.s(lang)?;
                        }
                    }
                }
                self.w(r#","children":["#)?;
            }
            List(Some(1)) => self.w(r#"{"tag":"ol","children":["#)?,
            List(Some(start)) => write!(self.w, r#"{{"tag":"ol","start":{},"children":["#, start)?,
            List(None) => self.w(r#"{"tag":"ul","children":["#)?,
            Item => self.w(r#"{"tag":"li","children":["#)?,
            Emphasis => self.w(r#"{"tag":"em","children":["#)?,
            Strong => self.w(r#"{"tag":"strong","children":["#)?,
            Strikethrough => self.w(r#"{"tag":"del","children":["#)?,
            Link(link_type, dest, title) => {
                self.w(r#"{"tag":"link","href":"#)?;
                match link_type {
                    LinkType::Email => {
                        let mut href = "mailto:".to_string();
                        href.push_str(&dest);
                        self.s(&href)?;
                    }
                    _ => self.s(&dest)?,
                }
                self.w(r#","title":"#)?;
                self.s(title)?;
                self.w(r#","children":["#)?;
            }
            Image(_link_type, dest, title) => {
                self.w(r#"{"tag":"image","title":"#)?;
                self.s(title)?;
                self.w(r#","src":"#)?;
                self.s(dest)?;
                self.w(r#","children":["#)?;
            }
            FootnoteDefinition(name) => {
                self.w(r#"{"tag":"footnote-def","name":"#)?;
                self.s(&name)?;
                let id = self.id(name);
                write!(self.w, r#","id":{},"children":["#, id)?;
            }
        }
        self.is_start = true;
        Ok(())
    }

    fn end_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        use Tag::*;
        match tag {
            Paragraph
            | Heading(_, _, _)
            | TableRow
            | TableCell
            | BlockQuote
            | List(_)
            | Item
            | Emphasis
            | Strong
            | Strikethrough
            | Link(_, _, _)
            | Image(_, _, _)
            | FootnoteDefinition(_) => self.w("]}"),
            Table(_) | CodeBlock(_) => self.w("]}]}"),
            TableHead => {
                self.w("]}]}")?;
                self.comma()?;
                self.is_start = true;
                self.w(r#"{"tag":"tbody","children":["#)
            }
        }
    }
}
