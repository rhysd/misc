use std::env;
use std::io::{self, BufRead};

fn parse(report: &str) -> Vec<i32> {
    report
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn is_safe(levels: &[i32]) -> bool {
    levels.is_sorted_by(|&l, &r| l < r && r - l <= 3)
        || levels.is_sorted_by(|&l, &r| l > r && l - r <= 3)
}

fn part1(lines: impl Iterator<Item = String>) {
    println!("{}", lines.filter(|report| is_safe(&parse(report))).count());
}

fn part2(lines: impl Iterator<Item = String>) {
    let count = lines
        .filter(|report| {
            let mut levels = parse(report);
            for i in 0..levels.len() {
                let removed = levels.remove(i);
                if is_safe(&levels) {
                    return true;
                }
                levels.insert(i, removed);
            }
            false
        })
        .count();
    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
