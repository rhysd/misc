use std::env;
use std::io::{self, BufRead};

enum Dir {
    U,
    R,
    D,
    L,
}

fn solve(moves: impl Iterator<Item = (Dir, isize)>) -> usize {
    let mut boundary = vec![];
    let mut cur = (0, 0);
    for (dir, dist) in moves {
        let (x, y) = cur;
        cur = match dir {
            Dir::U => {
                boundary.extend((y - dist..y).rev().map(|y| (x, y)));
                (x, y - dist)
            }
            Dir::R => {
                boundary.extend((x + 1..=x + dist).map(|x| (x, y)));
                (x + dist, y)
            }
            Dir::D => {
                boundary.extend((y + 1..=y + dist).map(|y| (x, y)));
                (x, y + dist)
            }
            Dir::L => {
                boundary.extend((x - dist..x).rev().map(|x| (x, y)));
                (x - dist, y)
            }
        };
    }

    // Use Shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
    let area = {
        let len = boundary.len();
        (0..len)
            .map(|i| {
                let (_, y_p) = boundary[i];
                let (x, _) = boundary[(i + 1) % len];
                let (_, y_n) = boundary[(i + 2) % len];
                x * (y_n - y_p)
            })
            .sum::<isize>()
            .unsigned_abs()
            / 2
    };

    // Use Pick's theorem https://en.wikipedia.org/wiki/Pick's_theorem
    // A = i + b/2 - 1 → i = A - b/2 + 1
    let boundaries = boundary.len();
    let inners = area - boundaries / 2 + 1;

    inners + boundaries
}

fn part1(lines: impl Iterator<Item = String>) {
    let moves = lines.map(|line| {
        let mut s = line.split_whitespace();
        let dir = match s.next().unwrap() {
            "U" => Dir::U,
            "R" => Dir::R,
            "D" => Dir::D,
            "L" => Dir::L,
            _ => unreachable!(),
        };
        let dist = s.next().unwrap().parse().unwrap();
        (dir, dist)
    });
    println!("{}", solve(moves));
}

fn part2(lines: impl Iterator<Item = String>) {
    let moves = lines.map(|line| {
        let hex = &line.split_whitespace().nth(2).unwrap()[2..8];
        let dist = isize::from_str_radix(&hex[..5], 16).unwrap();
        let dir = match &hex[5..] {
            "3" => Dir::U,
            "0" => Dir::R,
            "1" => Dir::D,
            "2" => Dir::L,
            _ => unreachable!(),
        };
        (dir, dist)
    });
    println!("{}", solve(moves));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
