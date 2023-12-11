use std::cmp;
use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

type Emptiness = Vec<Vec<bool>>;
type Pos = (usize, usize);
type Expanded = (HashSet<usize>, HashSet<usize>); // (rows, cols)

fn parse(lines: impl Iterator<Item = String>) -> (Emptiness, Vec<Pos>) {
    let mut galaxies = vec![];
    let empty = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '#' {
                        galaxies.push((x, y));
                    }
                    c == '.'
                })
                .collect()
        })
        .collect();
    (empty, galaxies)
}

fn expand(empty: &Emptiness) -> Expanded {
    let rows = (0..empty.len())
        .filter(|&y| empty[y].iter().all(|&s| s))
        .collect();
    let cols = (0..empty[0].len())
        .filter(|&x| (0..empty.len()).all(|y| empty[y][x]))
        .collect();
    (rows, cols)
}

fn cost((x1, y1): Pos, (x2, y2): Pos, expanded: &Expanded, rate: usize) -> usize {
    let (rows, cols) = expanded;
    let (x1, x2) = (cmp::min(x1, x2), cmp::max(x1, x2));
    let (y1, y2) = (cmp::min(y1, y2), cmp::max(y1, y2));
    let dist = |is_expanded| if is_expanded { rate } else { 1 };
    let x: usize = (x1..x2).map(|x| dist(cols.contains(&x))).sum();
    let y: usize = (y1..y2).map(|y| dist(rows.contains(&y))).sum();
    x + y
}

fn solve(lines: impl Iterator<Item = String>, rate: usize) -> usize {
    let (empty, galaxies) = parse(lines);
    let expanded = expand(&empty);
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += cost(galaxies[i], galaxies[j], &expanded, rate);
        }
    }
    sum
}

fn part1(lines: impl Iterator<Item = String>) {
    println!("{}", solve(lines, 2));
}

fn part2(lines: impl Iterator<Item = String>) {
    println!("{}", solve(lines, 1000000));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
