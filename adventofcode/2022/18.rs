use fxhash::FxHashSet;
use std::collections::VecDeque;
use std::env;
use std::io::{self, BufRead};

type Cube = (i32, i32, i32);

fn parse_input(lines: impl Iterator<Item = String>) -> FxHashSet<Cube> {
    lines
        .map(|line| {
            let mut s = line.split(',').map(|s| s.parse().unwrap());
            (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
        })
        .collect()
}

fn neighbors((x, y, z): Cube) -> [Cube; 6] {
    [
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
        (x, y + 1, z),
        (x + 1, y, z),
    ]
}

fn part1(lines: impl Iterator<Item = String>) {
    let cubes = parse_input(lines);
    let surfaces: usize = cubes
        .iter()
        .copied()
        .map(|c| neighbors(c).iter().filter(|n| !cubes.contains(n)).count())
        .sum();
    println!("{}", surfaces);
}

fn part2(lines: impl Iterator<Item = String>) {
    let cubes = parse_input(lines);

    // Calculate (x, y, z) ranges to fill
    let (mut xmin, mut xmax) = (i32::MAX, i32::MIN);
    let (mut ymin, mut ymax) = (i32::MAX, i32::MIN);
    let (mut zmin, mut zmax) = (i32::MAX, i32::MIN);
    for (x, y, z) in cubes.iter() {
        (xmin, xmax) = (xmin.min(x - 1), xmax.max(x + 1));
        (ymin, ymax) = (ymin.min(y - 1), ymax.max(y + 1));
        (zmin, zmax) = (zmin.min(z - 1), zmax.max(z + 1));
    }
    let inside = |(x, y, z)| {
        (xmin..=xmax).contains(&x) && (ymin..=ymax).contains(&y) && (zmin..=zmax).contains(&z)
    };

    // Fill outside lava
    let mut filled = FxHashSet::default();
    let mut queue = VecDeque::from([(xmax, ymax, zmax)]);
    while let Some(cube) = queue.pop_front() {
        for n in neighbors(cube) {
            if !inside(n) || cubes.contains(&n) || filled.contains(&n) {
                continue;
            }
            filled.insert(n);
            queue.push_back(n);
        }
    }

    let surfaces: usize = cubes
        .into_iter()
        .map(|c| neighbors(c).iter().filter(|n| filled.contains(n)).count())
        .sum();
    println!("{}", surfaces);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
