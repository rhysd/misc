use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};
use std::mem;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Digit(u8),
    Symbol(char),
    None,
}

type Schematic = Vec<Vec<Cell>>;
type Pos = (usize, usize);

fn parse(lines: impl Iterator<Item = String>) -> Schematic {
    lines
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Cell::None
                    } else if let Some(d) = c.to_digit(10) {
                        Cell::Digit(d as u8)
                    } else {
                        Cell::Symbol(c)
                    }
                })
                .collect()
        })
        .collect()
}

fn adjucents(x: usize, y: usize, s: &Schematic) -> impl Iterator<Item = Pos> {
    let xs = 0..s[0].len();
    let ys = 0..s.len();
    [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x, y.wrapping_sub(1)),
        (x + 1, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x.wrapping_sub(1), y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter(move |(x, y)| xs.contains(x) && ys.contains(y))
}

fn part1(lines: impl Iterator<Item = String>) {
    let schematic = parse(lines);

    let is_part_at =
        |x, y| adjucents(x, y, &schematic).any(|(x, y)| matches!(schematic[y][x], Cell::Symbol(_)));

    let mut sum = 0;
    for (y, cells) in schematic.iter().enumerate() {
        let mut num = 0;
        let mut is_part = false;
        for (x, cell) in cells.iter().enumerate() {
            if let Cell::Digit(d) = *cell {
                num = num * 10 + d as usize;
                is_part = is_part || is_part_at(x, y);
            } else {
                if num > 0 && is_part {
                    sum += num;
                }
                num = 0;
                is_part = false;
            }
        }
        if num > 0 && is_part {
            sum += num;
        }
    }
    println!("{sum}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let schematic = parse(lines);

    let mut gears = HashMap::<Pos, Vec<_>>::new();
    let mut extend_positions = |num, positions: HashSet<Pos>| {
        for pos in positions.into_iter() {
            gears.entry(pos).or_default().push(num);
        }
    };
    let adjucent_gear_positions = |x, y| {
        adjucents(x, y, &schematic).filter(|(x, y)| matches!(schematic[*y][*x], Cell::Symbol('*')))
    };

    for (y, cells) in schematic.iter().enumerate() {
        let mut num = 0;
        let mut gear_positions = HashSet::new();
        for (x, cell) in cells.iter().enumerate() {
            if let Cell::Digit(d) = *cell {
                num = num * 10 + d as usize;
                gear_positions.extend(adjucent_gear_positions(x, y));
            } else if num > 0 {
                extend_positions(num, mem::take(&mut gear_positions));
                num = 0;
            }
        }
        if num > 0 {
            extend_positions(num, gear_positions);
        }
    }

    let powers: usize = gears
        .into_values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum();
    println!("{powers}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
