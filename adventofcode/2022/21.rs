use fxhash::FxHashMap;
use std::env;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Op {
    Sub,
    Add,
    Mul,
    Div,
}
impl Op {
    fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            s => panic!("not an operator {:?}", s),
        }
    }
}

type Resolved<'a> = FxHashMap<&'a str, i64>;
type Unresolved<'a> = FxHashMap<&'a str, (&'a str, Op, &'a str)>;

fn parse(lines: &[String]) -> (Resolved<'_>, Unresolved<'_>) {
    let mut res = Resolved::default();
    let mut unres = Unresolved::default();
    for line in lines {
        let mut s = line.split(": ");
        let x = s.next().unwrap();
        let expr = s.next().unwrap();
        if let Ok(c) = expr.parse() {
            res.insert(x, c);
        } else {
            let mut s = expr.split_ascii_whitespace();
            let l = s.next().unwrap();
            let op = Op::new(s.next().unwrap());
            let r = s.next().unwrap();
            unres.insert(x, (l, op, r));
        }
    }
    (res, unres)
}

fn solve<'a>(res: &mut Resolved<'a>, unres: &mut Unresolved<'a>) {
    while !unres.is_empty() {
        let Some(n) = unres
            .iter()
            .find_map(|(n, (l, op, r))| {
                let x = res.get(n).copied();
                let y = res.get(l).copied();
                let z = res.get(r).copied();
                let (k, v) = match (x, y, z) {
                    (None, Some(y), Some(z)) => {
                        let v = match op {
                            Op::Add => y + z,
                            Op::Sub => y - z,
                            Op::Mul => y * z,
                            Op::Div => y / z,
                        };
                        (*n, v)
                    }
                    (Some(x), None, Some(z)) => {
                        let v = match op {
                            Op::Add => x - z, // x = y + z
                            Op::Sub => x + z, // x = y - z
                            Op::Mul => x / z, // x = y * z
                            Op::Div => x * z, // x = y / z
                        };
                        (*l, v)
                    }
                    (Some(x), Some(y), None) => {
                        let v = match op {
                            Op::Add => x - y, // x = y + z
                            Op::Sub => y - x, // x = y - z
                            Op::Mul => x / y, // x = y * z
                            Op::Div => y / x, // x = y / z
                        };
                        (*r, v)
                    }
                    _ => return None,
                };
                res.insert(k, v);
                Some(*n)
            }) else {
                return;
            };
        unres.remove(n);
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines = lines.collect::<Vec<_>>();
    let (mut res, mut unres) = parse(&lines);
    solve(&mut res, &mut unres);
    println!("{}", res.get("root").unwrap());
}

fn part2(lines: impl Iterator<Item = String>) {
    let lines = lines.collect::<Vec<_>>();
    let (mut res, mut unres) = parse(&lines);
    res.remove("humn"); // Revert wrong expression
    let (root_left, _, root_right) = unres.remove("root").unwrap();
    // First, solve lhs or rhs of "root" considering lhs is equal to rhs.
    solve(&mut res, &mut unres);
    if let Some(&v) = res.get(root_left) {
        res.insert(root_right, v);
    } else if let Some(&v) = res.get(root_right) {
        res.insert(root_left, v);
    }
    // Second, try to solve "humn"
    solve(&mut res, &mut unres);
    println!("{}", res.get("humn").unwrap());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
