use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let mut s = line.split_whitespace();
        let mut parse = || s.next().unwrap().parse::<i32>().unwrap();
        left.push(parse());
        right.push(parse());
    }
    left.sort();
    right.sort();
    let total: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    println!("{total}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut left = vec![];
    let mut counts = HashMap::new();
    for line in lines {
        let mut s = line.split_whitespace();
        let mut parse = || s.next().unwrap().parse::<u32>().unwrap();
        left.push(parse());
        *counts.entry(parse()).or_default() += 1u32;
    }
    let total: u32 = left
        .into_iter()
        .map(|x| x * counts.get(&x).copied().unwrap_or(0))
        .sum();
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
