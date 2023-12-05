use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut current: Vec<usize> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next().unwrap();

    let mut done = vec![false; current.len()];
    loop {
        match lines.next() {
            Some(l) if l.ends_with(':') => {}
            Some(l) if l.is_empty() => done.fill(false),
            Some(l) => {
                let mut n = l.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let dst_start = n.next().unwrap();
                let src_start = n.next().unwrap();
                let len = n.next().unwrap();
                let src = src_start..src_start + len;
                for (done, cur) in done.iter_mut().zip(current.iter_mut()) {
                    if !*done && src.contains(cur) {
                        *cur = *cur - src_start + dst_start;
                        *done = true;
                    }
                }
            }
            None => {
                println!("{}", current.into_iter().min().unwrap());
                return;
            }
        }
    }
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let first_line = lines.next().unwrap();
    let mut init = first_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut current = vec![];
    while let (Some(start), Some(len)) = (init.next(), init.next()) {
        current.push(start..start + len);
    }
    lines.next().unwrap();

    let mut done = vec![];
    loop {
        match lines.next() {
            Some(l) if l.ends_with(':') => {}
            Some(l) if l.is_empty() => {
                current.extend(done.iter().cloned());
                done.clear();
            }
            Some(l) => {
                let mut nums = l.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let dst_start = nums.next().unwrap();
                let src_start = nums.next().unwrap();
                let len = nums.next().unwrap();
                let src = src_start..src_start + len;
                let map = |n| n - src_start + dst_start;
                current = current
                    .into_iter()
                    .filter(|range| !range.is_empty())
                    .flat_map(
                        |r| match (src.contains(&r.start), src.contains(&(r.end - 1))) {
                            (true, true) => {
                                done.push(map(r.start)..map(r.end));
                                vec![]
                            }
                            (false, false) if r.start < src.start && src.end < r.end => {
                                done.push(map(src.start)..map(src.end));
                                vec![r.start..src.start, src.end..r.end]
                            }
                            (false, false) => vec![r],
                            (true, false) => {
                                done.push(map(r.start)..map(src.end));
                                vec![src.end..r.end]
                            }
                            (false, true) => {
                                done.push(map(src.start)..map(r.end));
                                vec![r.start..src.start]
                            }
                        },
                    )
                    .collect();
            }
            None => {
                let lowest = current
                    .into_iter()
                    .chain(done)
                    .map(|r| r.start)
                    .min()
                    .unwrap();
                println!("{lowest}");
                return;
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
