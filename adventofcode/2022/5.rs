use std::env;
use std::io::{self, BufRead};

fn crates(lines: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut lines: Vec<_> = lines.take_while(|l| !l.is_empty()).collect();
    let len = lines.last().unwrap().split_whitespace().count();
    lines.pop();
    let mut crates = vec![vec![]; len];
    for line in lines.into_iter().rev() {
        for (i, c) in crates.iter_mut().enumerate() {
            let b = line.as_bytes()[i * 4 + 1];
            if let b'A'..=b'Z' = b {
                c.push(b as char);
            }
        }
    }
    crates
}

fn ops(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (usize, usize, usize)> {
    lines.map(|line| {
        let line = line.strip_prefix("move ").unwrap();
        let mut s = line.split(" from ");
        let len = s.next().unwrap().parse().unwrap();
        let mut s = s.next().unwrap().split(" to ");
        let from = s.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = s.next().unwrap().parse::<usize>().unwrap() - 1;
        (len, from, to)
    })
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut crates = crates(&mut lines);

    for (len, from, to) in ops(lines) {
        for _ in 0..len {
            let b = crates[from].pop().unwrap();
            crates[to].push(b);
        }
    }

    for c in crates {
        print!("{}", c.last().unwrap());
    }
    println!();
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut crates = crates(&mut lines);
    let mut buf = vec![];

    for (len, from, to) in ops(lines) {
        for _ in 0..len {
            buf.push(crates[from].pop().unwrap());
        }
        for _ in 0..len {
            crates[to].push(buf.pop().unwrap());
        }
    }

    for c in crates {
        print!("{}", c.last().unwrap());
    }
    println!();
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
