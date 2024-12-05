use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

struct Rules(HashMap<i32, Vec<i32>>);

impl Rules {
    fn parse(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut m = HashMap::<i32, Vec<i32>>::new();
        for line in lines.take_while(|l| !l.is_empty()) {
            let mut s = line.split('|');
            let mut parse = || s.next().unwrap().parse().unwrap();
            m.entry(parse()).or_default().push(parse());
        }
        Self(m)
    }

    fn cmp(&self, l: &i32, r: &i32) -> Ordering {
        if matches!(self.0.get(l), Some(v) if v.contains(r)) {
            Ordering::Less
        } else if matches!(self.0.get(r), Some(v) if v.contains(l)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn is_sorted(&self, pages: &[i32]) -> bool {
        pages.windows(2).all(|w| self.cmp(&w[0], &w[1]) != Ordering::Greater)
    }

    fn sort(&self, pages: &mut [i32]) {
        pages.sort_by(|l, r| self.cmp(l, r));
    }
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let rules = Rules::parse(&mut lines);
    let total: i32 = lines
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .filter(|pages| rules.is_sorted(pages))
        .map(|pages| pages[pages.len() / 2])
        .sum();
    println!("{total}");
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let rules = Rules::parse(&mut lines);
    let total: i32 = lines
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .filter(|pages| !rules.is_sorted(pages))
        .map(|mut pages| {
            rules.sort(&mut pages);
            pages[pages.len() / 2]
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
