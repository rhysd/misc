use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead};

type Field = Vec<Vec<u8>>;

fn parse(lines: impl Iterator<Item = String>) -> Field {
    lines.map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}

fn count_trails(field: &Field, x: usize, y: usize) -> HashMap<(usize, usize), u32> {
    let ylen = field.len();
    let xlen = field[0].len();

    let mut queue = VecDeque::from([(0, x, y)]);
    let mut found = HashMap::new();
    while let Some((n, x, y)) = queue.pop_front() {
        if n == 9 {
            *found.entry((x, y)).or_default() += 1;
            continue;
        }
        for (x, y) in [
            (y > 0).then(|| (x, y - 1)),
            (x + 1 < xlen).then(|| (x + 1, y)),
            (y + 1 < ylen).then(|| (x, y + 1)),
            (x > 0).then(|| (x - 1, y)),
        ]
        .into_iter()
        .flatten()
        .filter(|&(x, y)| field[y][x] == n + 1)
        {
            queue.push_back((n + 1, x, y));
        }
    }

    found
}

fn part1(lines: impl Iterator<Item = String>) {
    let field = parse(lines);
    let total: u32 = field
        .iter()
        .enumerate()
        .map(|(y, xs)| {
            xs.iter()
                .enumerate()
                .filter(|(_, &n)| n == 0)
                .map(|(x, _)| count_trails(&field, x, y).len() as u32)
                .sum::<u32>()
        })
        .sum();
    println!("{total}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let field = parse(lines);
    let total: u32 = field
        .iter()
        .enumerate()
        .map(|(y, xs)| {
            xs.iter()
                .enumerate()
                .filter(|(_, &n)| n == 0)
                .map(|(x, _)| count_trails(&field, x, y).values().sum::<u32>())
                .sum::<u32>()
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
