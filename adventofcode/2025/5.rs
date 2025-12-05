use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();
            s.parse::<u64>().unwrap()..=e.parse().unwrap()
        })
        .collect::<Vec<_>>();
    let count = lines
        .filter(|l| {
            let v = l.parse().unwrap();
            ranges.iter().any(|r| r.contains(&v))
        })
        .count();
    println!("{count}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut ranges = lines
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();
    ranges.sort_unstable();

    let mut count = 0;
    let mut max = 0;
    for (s, e) in ranges {
        count += (e + 1).saturating_sub(s.max(max));
        max = max.max(e + 1);
    }
    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
