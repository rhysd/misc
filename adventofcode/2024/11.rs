use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn count(rem: u8, stone: u64, memo: &mut HashMap<(u8, u64), u64>) -> u64 {
    if let Some(&c) = memo.get(&(rem, stone)) {
        return c;
    }
    if rem == 0 {
        return 1;
    }
    let ret = if stone == 0 {
        count(rem - 1, 1, memo)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 1 {
            count(rem - 1, stone * 2024, memo)
        } else {
            let div = 10u64.pow(digits / 2);
            count(rem - 1, stone / div, memo) + count(rem - 1, stone % div, memo)
        }
    };
    memo.insert((rem, stone), ret);
    ret
}

fn solve(times: u8, mut lines: impl Iterator<Item = String>) -> u64 {
    let mut memo = HashMap::new();
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| count(times, s.parse().unwrap(), &mut memo))
        .sum()
}

fn part1(lines: impl Iterator<Item = String>) {
    println!("{}", solve(25, lines));
}

fn part2(lines: impl Iterator<Item = String>) {
    println!("{}", solve(75, lines));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
