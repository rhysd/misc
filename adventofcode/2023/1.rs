use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String> + std::fmt::Debug) {
    let total: u32 = lines
        .map(|l| {
            let mut it = l.chars().filter_map(|c| c.to_digit(10));
            let hi = it.next().unwrap();
            let lo = it.last().unwrap_or(hi);
            hi * 10 + lo
        })
        .sum();
    println!("{total}");
}

struct Calibs<'a>(&'a str);

impl Iterator for Calibs<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let l = self.0;
            let c = l.chars().next()?;
            self.0 = &self.0[1..];
            if let Some(u) = c
                .to_digit(10)
                .or_else(|| l.starts_with("one").then_some(1))
                .or_else(|| l.starts_with("two").then_some(2))
                .or_else(|| l.starts_with("three").then_some(3))
                .or_else(|| l.starts_with("four").then_some(4))
                .or_else(|| l.starts_with("five").then_some(5))
                .or_else(|| l.starts_with("six").then_some(6))
                .or_else(|| l.starts_with("seven").then_some(7))
                .or_else(|| l.starts_with("eight").then_some(8))
                .or_else(|| l.starts_with("nine").then_some(9))
            {
                return Some(u);
            };
        }
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    let total: u32 = lines
        .map(|l| {
            let mut it = Calibs(l.as_str());
            let hi = it.next().unwrap();
            let lo = it.last().unwrap_or(hi);
            hi * 10 + lo
        })
        .sum();
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
