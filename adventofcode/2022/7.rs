use std::cmp;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};
use std::iter::Peekable;

type Entries = HashMap<String, Entry>;

#[derive(Debug)]
enum Entry {
    File(usize),
    Dir(Entries),
}

#[derive(Debug)]
enum Dest {
    Parent,
    Child(String),
    Root,
}

#[derive(Debug)]
enum Cmd {
    Cd(Dest),
    Ls(Entries),
}

struct CmdParser<I: Iterator<Item = String>> {
    lines: Peekable<I>,
}

impl<I: Iterator<Item = String>> Iterator for CmdParser<I> {
    type Item = Cmd;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        if let Some(args) = line.strip_prefix("$ cd ") {
            let dest = match args {
                ".." => Dest::Parent,
                "/" => Dest::Root,
                c => Dest::Child(c.to_string()),
            };
            Some(Cmd::Cd(dest))
        } else if line == "$ ls" {
            let mut entries = HashMap::new();

            while self
                .lines
                .peek()
                .map(|s| !s.starts_with('$'))
                .unwrap_or(false)
            {
                let line = self.lines.next().unwrap();
                if let Some(arg) = line.strip_prefix("dir ") {
                    entries.insert(arg.to_string(), Entry::Dir(HashMap::new()));
                } else {
                    let mut s = line.split(' ');
                    let size = s.next().unwrap().parse().unwrap();
                    let name = s.next().unwrap();
                    entries.insert(name.to_string(), Entry::File(size));
                }
            }

            Some(Cmd::Ls(entries))
        } else {
            unreachable!("unknown command: {:?}", line)
        }
    }
}

impl<I: Iterator<Item = String>> CmdParser<I> {
    fn new(lines: I) -> Self {
        Self {
            lines: lines.peekable(),
        }
    }

    fn eval_impl(&mut self, cur: &mut Entries) -> bool {
        while let Some(cmd) = self.next() {
            match cmd {
                Cmd::Cd(Dest::Parent) => return false,
                Cmd::Cd(Dest::Child(name)) => {
                    let Entry::Dir(children) = cur.entry(name).or_insert(Entry::Dir(HashMap::new())) else {
                        unreachable!("cannot cd to file");
                    };
                    if self.eval_impl(children) {
                        return true; // Back to root
                    }
                }
                Cmd::Cd(Dest::Root) => return true,
                Cmd::Ls(entries) => {
                    for (name, entry) in entries.into_iter() {
                        match entry {
                            Entry::File(size) => {
                                cur.insert(name, Entry::File(size));
                            }
                            Entry::Dir(children) => {
                                cur.extend(children);
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn eval(mut self) -> Entries {
        let mut root = HashMap::new();
        while self.eval_impl(&mut root) {}
        root
    }
}

fn amount(node: &Entries) -> usize {
    let mut total = 0;
    for entry in node.values() {
        match entry {
            Entry::File(size) => total += size,
            Entry::Dir(children) => total += amount(children),
        }
    }
    total
}

fn part1(lines: impl Iterator<Item = String>) {
    let root = CmdParser::new(lines).eval();

    fn solve(node: &Entries) -> (usize, usize) {
        let mut total = 0;
        let mut result = 0;
        for entry in node.values() {
            match entry {
                Entry::File(size) => total += size,
                Entry::Dir(children) => {
                    let (r, t) = solve(children);
                    total += t;
                    result += r;
                    if t <= 100000 {
                        result += t;
                    }
                }
            }
        }
        (result, total)
    }

    println!("{:?}", solve(&root).0);
}

fn part2(lines: impl Iterator<Item = String>) {
    let root = CmdParser::new(lines).eval();
    let unused = 70_000_000 - amount(&root);
    let necessary = 30_000_000 - unused;

    fn solve(node: &Entries, necessary: usize) -> (usize, usize) {
        let mut total = 0;
        let mut smallest = usize::MAX;
        for entry in node.values() {
            match entry {
                Entry::File(size) => total += size, // Question requires smallest 'directory'
                Entry::Dir(children) => {
                    let (r, t) = solve(children, necessary);
                    total += t;

                    smallest = cmp::min(smallest, r);
                    if t >= necessary {
                        smallest = cmp::min(smallest, t);
                    }
                }
            }
        }
        (smallest, total)
    }

    println!("{:?}", solve(&root, necessary).0);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
