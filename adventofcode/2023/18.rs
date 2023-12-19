use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let mut trenches = HashSet::new();
    let mut cur = (0, 0);
    for line in lines {
        let mut s = line.split_whitespace();
        let dir = s.next().unwrap();
        let steps: isize = s.next().unwrap().parse().unwrap();
        let (x, y) = cur;
        cur = match dir {
            "U" => {
                trenches.extend((y - steps..y).map(|y| (x, y)));
                (x, y - steps)
            }
            "R" => {
                trenches.extend((x + 1..=x + steps).map(|x| (x, y)));
                (x + steps, y)
            }
            "D" => {
                trenches.extend((y + 1..=y + steps).map(|y| (x, y)));
                (x, y + steps)
            }
            "L" => {
                trenches.extend((x - steps..x).map(|x| (x, y)));
                (x - steps, y)
            }
            _ => unreachable!(),
        };
    }
    let trenches = trenches;

    let (min_x, max_x, min_y, max_y) = trenches.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (min(min_x, x), max(max_x, x), min(min_y, y), max(max_y, y))
        },
    );

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if trenches.contains(&(x, y)) {
                continue;
            }
            let mut seen = HashSet::new();
            let mut queue = vec![(x, y)];
            let mut is_inside = true;
            while let Some((x, y)) = queue.pop() {
                for adj in [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ] {
                    let (x, y) = adj;
                    if x < min_x || max_x < x || y < min_y || max_y < y {
                        is_inside = false;
                        break;
                    }
                    if trenches.contains(&adj) || seen.contains(&adj) {
                        continue;
                    }
                    seen.insert(adj);
                    queue.push(adj);
                }
            }
            if is_inside {
                println!("{}", trenches.len() + seen.len());
                return;
            }
        }
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut cur = (0, 0);
    let mut points = vec![];
    for line in lines {
        let hex = &line.split_whitespace().nth(2).unwrap()[2..8];
        let steps = isize::from_str_radix(&hex[..5], 16).unwrap();
        let (x, y) = cur;
        cur = match &hex[5..] {
             "3" /*U*/ => {
                points.extend((y - steps..y).rev().map(|y| (x, y)));
                (x, y - steps)
            }
            "0" /*R*/ => {
                points.extend((x + 1..=x + steps).map(|x| (x, y)));
                (x + steps, y) // R
            }
            "1" /*D*/ => {
                points.extend((y + 1..=y + steps).map(|y| (x, y)));
                (x, y + steps)
            }
            "2" /*L*/ => {
                points.extend((x - steps..x).rev().map(|x| (x, y)));
                (x - steps, y)
            }
            _ => unreachable!(),
        };
    }

    // Use Shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
    let area = {
        let len = points.len();
        (0..len)
            .map(|i| {
                let (_, y_p) = points[i];
                let (x, _) = points[(i + 1) % len];
                let (_, y_n) = points[(i + 2) % len];
                x * (y_n - y_p)
            })
            .sum::<isize>()
            .unsigned_abs()
            / 2
    };

    // Use Pick's theorem https://en.wikipedia.org/wiki/Pick's_theorem
    // A = i + b/2 - 1 → i = A - b/2 + 1
    let outer_len = points.len();
    let inner_len = area - outer_len / 2 + 1;

    println!("{}", inner_len + outer_len);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
