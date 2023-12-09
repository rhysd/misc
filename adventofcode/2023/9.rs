use std::env;
use std::io::{self, BufRead};

fn history(line: &str) -> Vec<Vec<isize>> {
    let mut current: Vec<_> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut history = vec![];
    while current.iter().any(|&i| i != 0) {
        let mut it = current.iter().peekable();
        let mut next = vec![];
        while let (Some(&i), Some(&&j)) = (it.next(), it.peek()) {
            next.push(j - i);
        }
        history.push(current);
        current = next;
    }
    history
}

fn part1(lines: impl Iterator<Item = String>) {
    let total: isize = lines
        .map(|line| {
            history(&line)
                .iter()
                .rev()
                .flat_map(|h| h.last())
                .fold(0, |delta, &h| h + delta)
        })
        .sum();
    println!("{total}")
}

fn part2(lines: impl Iterator<Item = String>) {
    let total: isize = lines
        .map(|line| {
            history(&line)
                .iter()
                .rev()
                .flat_map(|h| h.first())
                .fold(0, |delta, &h| h - delta)
        })
        .sum();
    println!("{total}")
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
