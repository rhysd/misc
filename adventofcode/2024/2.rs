use std::env;
use std::io::{self, BufRead};

fn parse(report: &str) -> impl Iterator<Item = i32> + '_ {
    report.split_whitespace().map(|n| n.parse().unwrap())
}

fn is_safe_report(levels: impl Iterator<Item = i32>) -> bool {
    fn is_safe(l: i32, r: i32) -> bool {
        l < r && r - l <= 3
    }

    let mut levels = levels.peekable();

    let (l, &r) = (levels.next().unwrap(), levels.peek().unwrap());
    let predicate = if is_safe(l, r) {
        is_safe
    } else if is_safe(r, l) {
        |l, r| is_safe(r, l) // flip
    } else {
        return false;
    };

    loop {
        let (Some(l), Some(&r)) = (levels.next(), levels.peek()) else {
            return true;
        };
        if !predicate(l, r) {
            return false;
        }
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    println!("{}", lines.filter(|l| is_safe_report(parse(l))).count());
}

fn part2(lines: impl Iterator<Item = String>) {
    let count = lines
        .filter(|report| {
            let levels: Vec<i32> = parse(report).collect();
            (0..levels.len()).any(|i| {
                let (l, r) = (&levels[..i], &levels[i + 1..]);
                is_safe_report(l.iter().copied().chain(r.iter().copied()))
            })
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
