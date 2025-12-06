use std::env;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}
impl Op {
    fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => unreachable!(),
        }
    }
    fn fold(self, it: impl Iterator<Item = u64>) -> u64 {
        match self {
            Self::Add => it.sum(),
            Self::Mul => it.product(),
        }
    }
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut inputs: Vec<Vec<u64>> =
        lines.next().unwrap().split_whitespace().map(|s| vec![s.parse().unwrap()]).collect();
    let ops: Vec<_> = loop {
        let line = lines.next().unwrap();
        if line.starts_with(['*', '+']) {
            break line.split_whitespace().map(Op::new).collect();
        }
        for (i, v) in line.split_whitespace().map(|s| s.parse().unwrap()).zip(inputs.iter_mut()) {
            v.push(i);
        }
    };

    let total: u64 =
        inputs.into_iter().zip(ops).map(|(input, op)| op.fold(input.into_iter())).sum();
    println!("{total}");
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut inputs: Vec<_> = lines.next().unwrap().chars().map(|c| c.to_string()).collect();
    let ops: Vec<_> = loop {
        let line = lines.next().unwrap();
        if line.starts_with(['*', '+']) {
            break line.split_whitespace().map(Op::new).collect();
        }
        for (c, s) in line.chars().zip(inputs.iter_mut()) {
            s.push(c);
        }
    };

    let total: u64 = inputs
        .split(|s| s.chars().all(|c| c == ' '))
        .zip(ops)
        .map(|(i, op)| op.fold(i.iter().map(|s| s.trim().parse().unwrap())))
        .sum();
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
