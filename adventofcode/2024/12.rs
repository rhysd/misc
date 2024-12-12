use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let arrange: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let (xlen, ylen) = (arrange[0].len(), arrange.len());
    let mut visited = HashSet::new();
    let mut price_at = move |x: usize, y: usize| -> u32 {
        let c = arrange[y][x];
        let mut area = 0;
        let mut sides = 0;
        let mut queue = VecDeque::from([(x, y)]);
        while let Some((x, y)) = queue.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            area += 1;
            for pos in [
                (y > 0).then(|| (x, y - 1)),
                (x + 1 < xlen).then(|| (x + 1, y)),
                (y + 1 < ylen).then(|| (x, y + 1)),
                (x > 0).then(|| (x - 1, y)),
            ] {
                match pos {
                    Some((x, y)) if arrange[y][x] == c => queue.push_back((x, y)),
                    _ => sides += 1,
                }
            }
        }
        area * sides
    };

    let mut price = 0;
    for y in 0..ylen {
        for x in 0..xlen {
            price += price_at(x, y);
        }
    }

    println!("{price}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let arrange: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let (xlen, ylen) = (arrange[0].len(), arrange.len());

    let mut visited = HashSet::new();
    let mut price_at = move |x: usize, y: usize| {
        let c = arrange[y][x];
        let mut queue = VecDeque::from([(x, y)]);
        let mut points = HashMap::new(); // Map from point to its edges
        let mut area: usize = 0;
        while let Some((x, y)) = queue.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            area += 1;
            for (pos, (from, to)) in [
                ((y > 0).then(|| (x, y - 1)), ((x, y), (x + 1, y))),
                ((x + 1 < xlen).then(|| (x + 1, y)), ((x + 1, y), (x + 1, y + 1))),
                ((y + 1 < ylen).then(|| (x, y + 1)), ((x, y + 1), (x + 1, y + 1))),
                ((x > 0).then(|| (x - 1, y)), ((x, y), (x, y + 1))),
            ] {
                match pos {
                    Some((x, y)) if arrange[y][x] == c => {
                        queue.push_back((x, y));
                    }
                    _ => {
                        points.entry(from).or_insert(vec![]).push(to);
                        points.entry(to).or_insert(vec![]).push(from);
                    }
                }
            }
        }

        // The number of corners is equal to the number of sides.
        let corners: usize = points
            .into_values()
            .map(|edges| match edges.len() {
                4 => 2, // 4 edges at a single point means two corners are tangent
                2 => {
                    let ((x1, y1), (x2, y2)) = (edges[0], edges[1]);
                    if x1 == x2 || y1 == y2 {
                        0 // Straight
                    } else {
                        1 // Orthogonal
                    }
                }
                _ => unreachable!(),
            })
            .sum();

        corners * area
    };

    let mut price = 0;
    for y in 0..ylen {
        for x in 0..xlen {
            price += price_at(x, y);
        }
    }

    println!("{price}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
