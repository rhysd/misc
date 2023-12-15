use std::env;
use std::io::{self, BufRead};

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |cur, c| (cur + c) * 17 % 256)
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let sum: usize = lines.next().unwrap().split(',').map(hash).sum();
    println!("{sum}");
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut boxes: [_; 256] = std::array::from_fn(|_| vec![]);

    let line = lines.next().unwrap();
    for operation in line.split(',') {
        if let Some(label) = operation.strip_suffix('-') {
            boxes[hash(label)].retain(|(l, _)| *l != label);
        } else {
            let mut s = operation.split('=');
            let label = s.next().unwrap();
            let focal_len: usize = s.next().unwrap().parse().unwrap();
            let b = &mut boxes[hash(label)];
            if let Some(l) = b.iter_mut().find(|(l, _)| *l == label) {
                *l = (label, focal_len);
            } else {
                b.push((label, focal_len));
            }
        }
    }

    let power: usize = boxes
        .into_iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(j, (_, focal_len))| (i + 1) * (j + 1) * focal_len)
                .sum::<usize>()
        })
        .sum();
    println!("{power}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
