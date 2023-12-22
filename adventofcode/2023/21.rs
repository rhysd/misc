use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

type Pos = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

fn parse(lines: impl Iterator<Item = String>) -> (Vec<Vec<Tile>>, Pos) {
    let mut start = (0, 0);
    let garden = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    'S' => {
                        start = (x as _, y as _);
                        Tile::Plot
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (garden, start)
}

fn part1(lines: impl Iterator<Item = String>) {
    let (garden, start) = parse(lines);
    let (x_len, y_len) = (garden[0].len() as _, garden.len() as _);

    let mut elves = HashSet::from([start]);

    for _ in 0..64 {
        elves = elves
            .into_iter()
            .flat_map(|(x, y)| {
                [
                    (y > 0).then_some((x, y - 1)),
                    (x + 1 < x_len).then_some((x + 1, y)),
                    (y + 1 < y_len).then_some((x, y + 1)),
                    (x > 0).then_some((x - 1, y)),
                ]
                .into_iter()
                .flatten()
                .filter(|&(x, y)| garden[y as usize][x as usize] == Tile::Plot)
            })
            .collect();
    }

    println!("{}", elves.len());
}

fn part2(lines: impl Iterator<Item = String>) {
    let (garden, start) = parse(lines);
    let len = garden.len() as _;

    // Check assumed pre-condition specific to the input
    assert_eq!(len, garden[0].len() as _);
    assert!((0..len as usize).all(|i| garden[0][i] == Tile::Plot && garden[i][0] == Tile::Plot));

    // Since (1) the input is cubic and (2) all edge tiles are plot, the reachable points spreads in square diamond shape.
    // And the square diamond spreads one tile per one tick since elve can only move to the adjucent tiles.
    // So the area of reachable point spreads in quadratic order per the length of side of the input square.
    //
    // f(x) = a * x^2 + b * x + c
    //
    // Calculate 3 pairs of consecutive (x, f(x)) and solve
    //
    // f(1) = a + b + c
    // f(2) = 4 * a + 2 * b + c
    // f(3) = 9 * a + 3 * b + c
    //
    // a = (f(1) - 2 * f(2) + f(3)) / 2
    // b = (-5 * f(1) + 8 * f(2) - 3 * f(3)) / 2
    // c = 3 * f(1) - 3 * f(2) + f(3)

    // x = 1 → THE_STEP % len
    // x = 2 → THE_STEP % len + len
    // x = 3 → THE_STEP % len + len + len
    const THE_STEP: isize = 26501365;
    let mut next_tick = THE_STEP % len;
    let mut a = vec![];

    let mut elves = HashSet::from([start]);
    for tick in 1.. {
        elves = elves
            .into_iter()
            .flat_map(|(x, y)| {
                [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                    .into_iter()
                    .filter(|&(x, y)| {
                        let x = x.rem_euclid(len) as usize;
                        let y = y.rem_euclid(len) as usize;
                        garden[y][x] == Tile::Plot
                    })
            })
            .collect();

        if tick == next_tick {
            a.push(elves.len() as isize);
            if a.len() == 3 {
                break;
            }
            next_tick += len;
        }
    }

    // f(x) = ((f(1) - 2 * f(2) + f(3)) / 2) * x^2 + ((-5 * f(1) + 8 * f(2) - 3 * f(3)) / 2) * x + 3 * f(1) - 3 * f(2) + f(3)
    let (f1, f2, f3) = (a[0], a[1], a[2]);
    let f = |x: isize| {
        (f1 - 2 * f2 + f3) / 2 * x * x + (-5 * f1 + 8 * f2 - 3 * f3) / 2 * x + 3 * f1 - 3 * f2 + f3
    };

    println!("{}", f(THE_STEP / len + 1));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
