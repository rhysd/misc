use std::cmp;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

enum Term<'a> {
    Accepted,
    Rejected,
    Next(&'a str),
}

impl<'a> Term<'a> {
    fn parse(s: &'a str) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            s => Self::Next(s),
        }
    }
}

enum Insn<'a> {
    Gt(char, usize, Term<'a>),
    Lt(char, usize, Term<'a>),
    End(Term<'a>),
}

impl<'a> Insn<'a> {
    fn parse(s: &'a str) -> Self {
        if let Some((expr, term)) = s.split_once(':') {
            let term = Term::parse(term);
            if let Some((lhs, rhs)) = expr.split_once('>') {
                let var = lhs.chars().next().unwrap();
                Self::Gt(var, rhs.parse().unwrap(), term)
            } else if let Some((lhs, rhs)) = expr.split_once('<') {
                let var = lhs.chars().next().unwrap();
                Self::Lt(var, rhs.parse().unwrap(), term)
            } else {
                unreachable!()
            }
        } else {
            Insn::End(Term::parse(s))
        }
    }
}

type Workflows<'a> = HashMap<&'a str, Vec<Insn<'a>>>;

fn parse_part(s: &str) -> HashMap<char, usize> {
    let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
    s.split(',')
        .map(|assign| {
            let (var, val) = assign.split_once('=').unwrap();
            let var = var.chars().next().unwrap();
            let val = val.parse().unwrap();
            (var, val)
        })
        .collect()
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let mut lines = lines.iter();
    let mut workflows = Workflows::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (name, insns) = line.split_once('{').unwrap();
        let insns = insns.strip_suffix('}').unwrap();
        let insns: Vec<_> = insns.split(',').map(Insn::parse).collect();
        workflows.insert(name, insns);
    }

    let total: usize = lines
        .map(|line| parse_part(line))
        .filter_map(|vars| {
            let mut current = "in";
            loop {
                for insn in &workflows[current] {
                    let term = match insn {
                        Insn::Gt(n, v, t) if vars[n] > *v => t,
                        Insn::Lt(n, v, t) if vars[n] < *v => t,
                        Insn::End(t) => t,
                        _ => continue,
                    };
                    match term {
                        Term::Accepted => return Some(vars.values().sum::<usize>()),
                        Term::Rejected => return None,
                        Term::Next(name) => {
                            current = name;
                            break;
                        }
                    }
                }
            }
        })
        .sum();

    println!("{total}");
}

enum Cond {
    Gt(char, usize),
    Lt(char, usize),
    True,
}

struct If {
    cond: Cond,
    then: EvalTree,
}

enum EvalTree {
    Node(Vec<If>),
    Accepted,
    Rejected,
}

impl EvalTree {
    fn child(term: &Term<'_>, workflows: &Workflows<'_>) -> Self {
        match term {
            Term::Accepted => Self::Accepted,
            Term::Rejected => Self::Rejected,
            Term::Next(w) => Self::build(w, workflows),
        }
    }

    fn build(name: &'_ str, workflows: &Workflows<'_>) -> Self {
        let children = workflows[name]
            .iter()
            .map(|insn| match insn {
                Insn::Gt(n, v, t) => If {
                    cond: Cond::Gt(*n, *v),
                    then: Self::child(t, workflows),
                },
                Insn::Lt(n, v, t) => If {
                    cond: Cond::Lt(*n, *v),
                    then: Self::child(t, workflows),
                },
                Insn::End(t) => If {
                    cond: Cond::True,
                    then: Self::child(t, workflows),
                },
            })
            .collect();
        Self::Node(children)
    }

    fn solve(&self, mut res: Restrictions) -> usize {
        match self {
            Self::Accepted => res.x.combi() * res.m.combi() * res.a.combi() * res.s.combi(),
            Self::Rejected => 0,
            Self::Node(children) => {
                let mut combi = 0;
                for If { cond, then } in children {
                    match cond {
                        Cond::Gt(n, v) => {
                            let mut if_true = res.clone();
                            let var = if_true.var_mut(*n);
                            var.gte = cmp::max(var.gte, v + 1);
                            if var.is_possible() {
                                combi += then.solve(if_true)
                            }
                            res.var_mut(*n).lte = *v;
                        }
                        Cond::Lt(n, v) => {
                            let mut if_true = res.clone();
                            let var = if_true.var_mut(*n);
                            var.lte = cmp::min(var.lte, v - 1);
                            if var.is_possible() {
                                combi += then.solve(if_true)
                            }
                            res.var_mut(*n).gte = *v;
                        }
                        Cond::True => {
                            combi += then.solve(res.clone());
                        }
                    }
                }
                combi
            }
        }
    }
}

#[derive(Clone)]
struct Restriction {
    gte: usize,
    lte: usize,
}

impl Default for Restriction {
    fn default() -> Self {
        Self { gte: 1, lte: 4000 }
    }
}

impl Restriction {
    fn is_possible(&self) -> bool {
        self.gte <= self.lte
    }

    fn combi(&self) -> usize {
        self.lte - self.gte + 1
    }
}

#[derive(Default, Clone)]
struct Restrictions {
    x: Restriction,
    m: Restriction,
    a: Restriction,
    s: Restriction,
}

impl Restrictions {
    fn var_mut(&mut self, name: char) -> &mut Restriction {
        match name {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => unreachable!(),
        }
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let workflows = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, insns) = line.split_once('{').unwrap();
            let insns = insns.strip_suffix('}').unwrap();
            let insns: Vec<_> = insns.split(',').map(Insn::parse).collect();
            (name, insns)
        })
        .collect();
    let tree = EvalTree::build("in", &workflows);
    println!("{}", tree.solve(Restrictions::default()));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
