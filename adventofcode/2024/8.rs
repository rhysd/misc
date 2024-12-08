use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

type Pos = (i32, i32);

struct Field {
    max_x: i32,
    max_y: i32,
    antennas: HashMap<char, Vec<Pos>>,
}

impl Field {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let (mut max_x, mut max_y) = (0, 0);
        let mut antennas = HashMap::new();

        for (y, line) in lines.enumerate() {
            let y = y as _;
            for (x, c) in line.chars().enumerate() {
                let x = x as _;
                if c != '.' {
                    antennas.entry(c).or_insert(vec![]).push((x, y));
                }
                max_x = x;
            }
            max_y = y;
        }

        Self { max_x, max_y, antennas }
    }

    fn contains(&self, (x, y): Pos) -> bool {
        0 <= x && x <= self.max_x && 0 <= y && y <= self.max_y
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let field = Field::parse(lines);

    let mut antinodes = HashSet::new();
    for antennas in field.antennas.values() {
        for &a1 in antennas {
            let (x1, y1) = a1;
            for &a2 in antennas {
                if a1 == a2 {
                    continue;
                }
                let (x2, y2) = a2;
                let pos = (x1 + (x1 - x2), y1 + (y1 - y2));
                if field.contains(pos) {
                    antinodes.insert(pos);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn part2(lines: impl Iterator<Item = String>) {
    let field = Field::parse(lines);

    let mut antinodes = HashSet::new();
    for antennas in field.antennas.values() {
        for &a1 in antennas {
            let (x1, y1) = a1;
            for &a2 in antennas {
                if a1 == a2 {
                    continue;
                }
                let (x2, y2) = a2;
                let (dx, dy) = (x1 - x2, y1 - y2);
                let mut pos = (x1, y1);
                while field.contains(pos) {
                    antinodes.insert(pos);
                    pos.0 += dx;
                    pos.1 += dy;
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
