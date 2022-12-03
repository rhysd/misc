use std::env;
use std::io::{self, BufRead};

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 1 + 26,
        _ => unreachable!("{:?}", c),
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let total: usize = lines
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let c = left.chars().find(|&c| right.contains(c)).unwrap();
            priority(c)
        })
        .sum();
    println!("{}", total);
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut next_group = move || Some((lines.next()?, lines.next()?, lines.next()?));
    let mut total = 0;
    while let Some((e1, e2, e3)) = next_group() {
        let c = e1.chars().find(|&c| e2.contains(c) && e3.contains(c));
        total += priority(c.unwrap());
    }
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
