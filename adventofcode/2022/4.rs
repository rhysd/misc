use std::env;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

type Range = RangeInclusive<u8>;

fn covers(r1: &Range, r2: &Range) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn overwraps(r1: &Range, r2: &Range) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start()) && r2.contains(r1.end())
}

fn parse(line: String) -> (Range, Range) {
    let mut s = line.split(',');
    let mut range = move || -> Range {
        let mut s = s.next().unwrap().split('-');
        s.next().unwrap().parse().unwrap()..=s.next().unwrap().parse().unwrap()
    };
    (range(), range())
}

fn part1(lines: impl Iterator<Item = String>) {
    let total: usize = lines
        .map(parse)
        .filter(|(l, r)| covers(l, r) || covers(r, l))
        .count();
    println!("{}", total);
}

fn part2(lines: impl Iterator<Item = String>) {
    let total: usize = lines.map(parse).filter(|(l, r)| overwraps(l, r)).count();
    println!("{}", total);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
