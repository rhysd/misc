use std::env;
use std::io::{self, BufRead};

type Pattern = Vec<Vec<char>>;

struct Patterns<I: Iterator<Item = String>>(I);
impl<I: Iterator<Item = String>> Iterator for Patterns<I> {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pat = vec![];
        loop {
            match self.0.next() {
                Some(line) if line.is_empty() => return Some(pat),
                Some(line) => pat.push(line.chars().collect()),
                None if pat.is_empty() => return None,
                None => return Some(pat),
            }
        }
    }
}

fn find_reflec(pat: &Pattern) -> Option<usize> {
    let xlen = pat[0].len();
    for x in 0..xlen - 1 {
        let (mut l, mut r) = (x, x + 1);
        while pat.iter().all(|xs| xs[l] == xs[r]) {
            if l == 0 || r == xlen - 1 {
                return Some(x);
            }
            l -= 1;
            r += 1;
        }
    }
    None
}

fn transpose(pat: &Pattern) -> Pattern {
    let (xlen, ylen) = (pat[0].len(), pat.len());
    (0..xlen)
        .map(|x| (0..ylen).map(|y| pat[y][x]).collect())
        .collect()
}

fn part1(lines: impl Iterator<Item = String>) {
    let total: usize = Patterns(lines)
        .map(|pat| {
            find_reflec(&pat)
                .map(|x| x + 1)
                .or_else(|| find_reflec(&transpose(&pat)).map(|y| 100 * (y + 1)))
                .unwrap_or(0)
        })
        .sum();
    println!("{total}");
}

fn find_reflec_with_smudge(pat: &Pattern) -> Option<usize> {
    let xlen = pat[0].len();
    for x in 0..xlen - 1 {
        let (mut l, mut r) = (x, x + 1);
        let mut fixed = false;
        loop {
            match pat.iter().filter(|xs| xs[l] != xs[r]).count() {
                0 => {}
                1 if !fixed => fixed = true,
                _ => break,
            }

            if l == 0 || r == xlen - 1 {
                if fixed {
                    return Some(x);
                } else {
                    break;
                }
            }
            l -= 1;
            r += 1;
        }
    }
    None
}

fn part2(lines: impl Iterator<Item = String>) {
    let total: usize = Patterns(lines)
        .map(|pat| {
            find_reflec_with_smudge(&pat)
                .map(|x| x + 1)
                .or_else(|| find_reflec_with_smudge(&transpose(&pat)).map(|y| 100 * (y + 1)))
                .unwrap_or(0)
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
