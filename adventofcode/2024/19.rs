use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let line = lines.next().unwrap();
    let towels: Vec<_> = line.split(", ").collect();
    assert!(lines.next().unwrap().is_empty());

    fn is_possible(design: &str, towels: &[&str], memo: &mut HashMap<String, bool>) -> bool {
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
        memo.insert(design.to_string(), ret);
        ret
    }

    let mut memo = HashMap::new();
    println!("{}", lines.filter(|d| is_possible(d, &towels, &mut memo)).count());
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let line = lines.next().unwrap();
    let towels: Vec<_> = line.split(", ").collect();
    assert!(lines.next().unwrap().is_empty());

    fn combinations(design: &str, towels: &[&str], memo: &mut HashMap<String, u64>) -> u64 {
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
        memo.insert(design.to_string(), ret);
        ret
    }

    let mut memo = HashMap::new();
    println!("{}", lines.map(|d| combinations(&d, &towels, &mut memo)).sum::<u64>());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
