use fxhash::FxHashMap;
use rayon::prelude::*;
use std::env;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

type Pos = (i64, i64);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
    distance: i64,
}
impl Sensor {
    fn parse(line: &str) -> Self {
        fn parse_pos(s: &str) -> Pos {
            let s = s.strip_prefix("x=").unwrap();
            let mut s = s.split(", y=").map(|s| s.parse().unwrap());
            (s.next().unwrap(), s.next().unwrap())
        }

        let line = line.strip_prefix("Sensor at ").unwrap();
        let mut s = line.split(": closest beacon is at ");
        let pos = parse_pos(s.next().unwrap());
        let beacon = parse_pos(s.next().unwrap());
        let distance = (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs();
        Self {
            pos,
            beacon,
            distance,
        }
    }

    fn scanned_x(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let p = self.pos;
        let half = self.distance - (p.1 - y).abs();
        (half > 0).then_some(p.0 - half..=p.0 + half)
    }
}

fn is_test(s: &[Sensor]) -> bool {
    s[0].pos.0 < 1000000
}

fn part1(lines: impl Iterator<Item = String>) {
    #[derive(Debug)]
    enum Pos {
        Scanned,
        Beacon,
    }

    let mut pos = FxHashMap::default();
    let sensors: Vec<_> = lines.map(|l| Sensor::parse(&l)).collect();
    let y = if is_test(&sensors) { 10 } else { 2000000 };
    for sensor in sensors {
        if let Some(xs) = sensor.scanned_x(y) {
            for x in xs {
                pos.entry(x).or_insert(Pos::Scanned);
            }
        }
        if sensor.beacon.1 == y {
            pos.insert(sensor.beacon.0, Pos::Beacon);
        }
    }
    let count = pos
        .into_iter()
        .filter(|(_, p)| matches!(p, Pos::Scanned))
        .count();
    println!("{}", count);
}

fn part2(lines: impl Iterator<Item = String>) {
    let sensors: Vec<_> = lines.map(|l| Sensor::parse(&l)).collect();
    let max = if is_test(&sensors) { 20 } else { 4000000 };
    let found = (0..=max).into_par_iter().find_map_any(|y| {
        let covered: Vec<_> = sensors.iter().flat_map(|s| s.scanned_x(y)).collect();
        let mut x = 0;
        while x <= max {
            if let Some(range) = covered.iter().find(|r| r.contains(&x)) {
                x = range.end() + 1;
                continue;
            }
            return Some((x, y));
        }
        None
    });
    let (x, y) = found.unwrap();
    println!("{}", x * 4000000 + y);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
