use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

fn count_matches(line: impl AsRef<str>) -> usize {
    let line = line.as_ref().split(": ").nth(1).unwrap();
    let mut parts = line.split(" | ");
    let wins: HashSet<_> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|n| wins.contains(n))
        .count()
}

fn part1(lines: impl Iterator<Item = String>) {
    let piled: usize = lines
        .map(count_matches)
        .filter(|c| *c > 0)
        .map(|c| 2usize.pow((c - 1) as _))
        .sum();
    println!("{piled}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let matches: Vec<_> = lines.map(count_matches).collect();
    let len = matches.len();
    let mut counts = vec![1; len];
    for i in 0..len {
        let (m, c) = (matches[i], counts[i]);
        for j in i + 1..=min(i + m, len - 1) {
            counts[j] += c;
        }
    }
    println!("{}", counts.into_iter().sum::<usize>());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
