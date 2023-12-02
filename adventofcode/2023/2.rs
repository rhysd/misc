use std::env;
use std::io::{self, BufRead};

fn games(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (usize, usize, usize)> {
    lines.map(|line| {
        let line = &line[line.find(':').unwrap() + 1..];
        let (mut r, mut g, mut b) = (0, 0, 0);
        for cubes in line.split(';') {
            for cube in cubes.trim().split(", ") {
                let mut s = cube.split(' ');
                let c = s.next().unwrap().parse().unwrap();
                match s.next().unwrap() {
                    "red" if c > r => r = c,
                    "green" if c > g => g = c,
                    "blue" if c > b => b = c,
                    _ => {}
                }
            }
        }
        (r, g, b)
    })
}

fn part1(lines: impl Iterator<Item = String>) {
    let sum: usize = games(lines)
        .enumerate()
        .filter_map(|(i, (r, g, b))| (r <= 12 && g <= 13 && b <= 14).then_some(i + 1))
        .sum();
    println!("{}", sum);
}

fn part2(lines: impl Iterator<Item = String>) {
    let sum: usize = games(lines).map(|(r, g, b)| r * g * b).sum();
    println!("{}", sum);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
