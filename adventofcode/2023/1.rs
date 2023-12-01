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
            if let Some(u) = self.0.chars().next()?.to_digit(10).or_else(|| {
                [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .position(|s| self.0.starts_with(s))
                .map(|i| i as u32 + 1)
            }) {
                self.0 = &self.0[1..];
                return Some(u);
            };
            self.0 = &self.0[1..];
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
