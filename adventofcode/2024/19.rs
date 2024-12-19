use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let line = lines.next().unwrap();
    let towels: Vec<_> = line.split(", ").collect();
    assert!(lines.next().unwrap().is_empty());
    let designs: Vec<_> = lines.collect();

    fn is_possible<'a>(
        design: &'a str,
        towels: &[&str],
        memo: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if design.is_empty() {
            return true;
        }
        if let Some(&ret) = memo.get(design) {
            return ret;
        }
        let ret = towels
            .iter()
            .flat_map(|t| design.strip_prefix(t))
            .any(|d| is_possible(d, towels, memo));
        memo.insert(design, ret);
        ret
    }

    let mut memo = HashMap::new();
    println!("{}", designs.iter().filter(|d| is_possible(d, &towels, &mut memo)).count());
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let line = lines.next().unwrap();
    let towels: Vec<_> = line.split(", ").collect();
    assert!(lines.next().unwrap().is_empty());
    let designs: Vec<_> = lines.collect();

    fn combinations<'a>(design: &'a str, towels: &[&str], memo: &mut HashMap<&'a str, u64>) -> u64 {
        if design.is_empty() {
            return 1;
        }
        if let Some(&ret) = memo.get(design) {
            return ret;
        }
        let ret = towels
            .iter()
            .flat_map(|t| design.strip_prefix(t))
            .map(|d| combinations(d, towels, memo))
            .sum();
        memo.insert(design, ret);
        ret
    }

    let mut memo = HashMap::new();
    println!("{}", designs.iter().map(|d| combinations(d, &towels, &mut memo)).sum::<u64>());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
