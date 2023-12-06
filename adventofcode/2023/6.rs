use std::env;
use std::io::{self, BufRead};

fn solve(time: &str, dist: &str) -> usize {
    let time = time
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap());
    let record = dist
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap());
    time.zip(record)
        .map(|(time, record)| {
            (1..time)
                .map(|velocity| velocity * (time - velocity))
                .filter(|dist| *dist > record)
                .count()
        })
        .product()
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let time = lines.next().unwrap();
    let dist = lines.next().unwrap();
    println!("{}", solve(&time, &dist));
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let time = lines.next().unwrap().replace(' ', "");
    let dist = lines.next().unwrap().replace(' ', "");
    println!("{}", solve(&time, &dist));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
