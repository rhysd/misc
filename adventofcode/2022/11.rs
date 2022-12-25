use num::Integer;
use std::env;
use std::io::{self, BufRead};
use std::mem;
use std::str::FromStr;

type Arg = Option<i64>; // None means 'old' variable

#[derive(Debug)]
enum Op {
    Add(Arg, Arg),
    Mul(Arg, Arg),
}

impl Op {
    fn parse(expr: &str) -> Self {
        let mut s = expr.split(' ');
        let l = s.next().unwrap().parse().ok();
        let op = s.next().unwrap();
        let r = s.next().unwrap().parse().ok();
        match op {
            "+" => Self::Add(l, r),
            "*" => Self::Mul(l, r),
            _ => panic!("unexpected op {:?}", op),
        }
    }

    fn eval(&self, old: i64) -> i64 {
        match &self {
            Self::Add(l, r) => l.unwrap_or(old) + r.unwrap_or(old),
            Self::Mul(l, r) => l.unwrap_or(old) * r.unwrap_or(old),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    div_cond: i64,
    if_true: usize,
    if_false: usize,
}

fn parse_monkey(mut it: impl Iterator<Item = String>) -> Option<Monkey> {
    loop {
        if it.next()?.starts_with("Monkey ") {
            break;
        }
    }

    let mut next = move || it.next().unwrap();
    let items = next()
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|i| i.parse().unwrap())
        .collect();
    let op = Op::parse(next().strip_prefix("  Operation: new = ").unwrap());

    fn parse<V: FromStr>(l: String, prefix: &str) -> V {
        l.strip_prefix(prefix).unwrap().parse().ok().unwrap()
    }

    Some(Monkey {
        items,
        op,
        div_cond: parse(next(), "  Test: divisible by "),
        if_true: parse(next(), "    If true: throw to monkey "),
        if_false: parse(next(), "    If false: throw to monkey "),
    })
}

fn parse_monkeys(mut it: impl Iterator<Item = String>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    while let Some(monkey) = parse_monkey(&mut it) {
        monkeys.push(monkey);
    }
    monkeys
}

fn solve<F: Fn(i64) -> i64>(round: usize, mut monkeys: Vec<Monkey>, reduce: F) -> usize {
    let mut count = vec![0; monkeys.len()];
    for _ in 0..round {
        for i in 0..monkeys.len() {
            count[i] += monkeys[i].items.len();
            for item in mem::take(&mut monkeys[i].items) {
                let m = &monkeys[i];
                let new = reduce(m.op.eval(item));
                let thrown = if new % m.div_cond == 0 {
                    m.if_true
                } else {
                    m.if_false
                };
                monkeys[thrown].items.push(new);
            }
        }
    }

    count.sort();
    count[count.len() - 2] * count[count.len() - 1]
}

fn part1(lines: impl Iterator<Item = String>) {
    let monkeys = parse_monkeys(lines);
    println!("{}", solve(20, monkeys, |i| i / 3));
}

fn part2(lines: impl Iterator<Item = String>) {
    let monkeys = parse_monkeys(lines);
    let divider: i64 = monkeys.iter().fold(1, |acc, m| acc.lcm(&m.div_cond));
    println!("{}", solve(10000, monkeys, move |i| i % divider));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
