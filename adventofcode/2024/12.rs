use std::collections::{HashSet, VecDeque};
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
        if !visited.insert((x, y)) {
            return 0; // Avoid memory allocation of `VecDeque`
        }

        let c = arrange[y][x];
        let mut queue = VecDeque::from([(x, y)]);
        let mut area: usize = 0;
        let mut corners: usize = 0;
        while let Some((x, y)) = queue.pop_front() {
            area += 1;

            let neighbors = [
                (y > 0).then(|| (x, y - 1)),
                (x + 1 < xlen).then(|| (x + 1, y)),
                (y + 1 < ylen).then(|| (x, y + 1)),
                (x > 0).then(|| (x - 1, y)),
            ];
            for (i, cur) in neighbors.into_iter().enumerate() {
                // Looking at the top-right corner, check if it is one of the following patterns where A is the current
                // cell and B is other cell and E is empty (outside the field).
                //
                //   AB   B    E    B    E
                //   AA   AB   AB   AE   AE
                //
                // Do this for all four corners.
                let is_corner = match (cur, neighbors[(i + 1) % 4]) {
                    (Some((x1, y1)), Some((x2, y2))) => {
                        let (x3, y3) = if (x, y) != (x1, y2) { (x1, y2) } else { (x2, y1) }; // Diagonal position
                        let (c1, c2, c3) = (arrange[y1][x1], arrange[y2][x2], arrange[y3][x3]);
                        c != c1 && c != c2 || c == c1 && c == c2 && c != c3
                    }
                    (Some((x, y)), None) | (None, Some((x, y))) => arrange[y][x] != c,
                    (None, None) => true,
                };
                if is_corner {
                    corners += 1;
                }

                if let Some((x, y)) = cur {
                    if arrange[y][x] == c && visited.insert((x, y)) {
                        queue.push_back((x, y));
                    }
                }
            }
        }

        // The number of corners is equal to the number of sides.
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
