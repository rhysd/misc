use std::cmp::Ordering;
use std::env;
use std::io::{self, BufRead};

fn parse<'a>(time: &'a str, dist: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
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
}

fn solve(time: usize, record: usize) -> usize {
    fn search(min: usize, max: usize, time: usize, record: usize) -> usize {
        if max - min <= 1 {
            return max;
        }
        let v = (min + max) / 2;
        match (v * (time - v)).cmp(&record) {
            Ordering::Equal => v + 1,
            Ordering::Less => search(v, max, time, record),
            Ordering::Greater => search(min, v, time, record),
        }
    }

    let max = time / 2;
    let min = search(1, max, time, record);
    (max - min + 1) * 2 - (time - 1) % 2
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let time = lines.next().unwrap();
    let dist = lines.next().unwrap();
    let result: usize = parse(&time, &dist)
        .map(|(time, record)| solve(time, record))
        .product();
    println!("{}", result);
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let time = lines.next().unwrap().replace(' ', "");
    let dist = lines.next().unwrap().replace(' ', "");
    let (time, record) = parse(&time, &dist).next().unwrap();
    println!("{}", solve(time, record));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
