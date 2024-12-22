use std::array;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

fn next_secret(s: u64) -> u64 {
    let s = ((s * 64) ^ s) % 16777216;
    let s = ((s / 32) ^ s) % 16777216;
    ((s * 2048) ^ s) % 16777216
}

fn part1(lines: impl Iterator<Item = String>) {
    let sum: u64 = lines
        .map(|l| {
            let mut s = l.parse().unwrap();
            for _ in 0..2000 {
                s = next_secret(s);
            }
            s
        })
        .sum();
    println!("{sum}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut prices = HashMap::<[i8; 4], i32>::new();
    let mut max = 0;
    for mut secret in lines.map(|l| l.parse().unwrap()) {
        let mut cur = [0, 0, 0, 0, (secret % 10) as _];
        let mut seen = HashSet::new();
        for i in 0..2000 {
            secret = next_secret(secret);
            cur.rotate_left(1);
            cur[4] = (secret % 10) as _;

            if i < 3 {
                continue;
            }

            let seq = array::from_fn(|i| cur[i + 1] - cur[i]);
            if !seen.insert(seq) {
                continue;
            }

            let sum = prices.entry(seq).or_default();
            *sum += cur[4] as i32;
            max = max.max(*sum);
        }
    }
    println!("{max}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
