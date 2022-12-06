use std::env;
use std::io::{self, BufRead};

fn solve(line: &str, size: usize) -> usize {
    let (i, _) = line
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, s)| {
            for i in 0..size {
                for j in i + 1..size {
                    if s[i] == s[j] {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap();
    i + size
}

fn part1(lines: impl Iterator<Item = String>) {
    for line in lines {
        println!("{}", solve(&line, 4));
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    for line in lines {
        println!("{}", solve(&line, 14));
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
