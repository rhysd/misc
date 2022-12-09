use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

fn parse(line: &str) -> ((i32, i32), u32) {
    let mut s = line.split(' ');
    let dir = match s.next().unwrap() {
        "U" => (0, 1),
        "R" => (1, 0),
        "D" => (0, -1),
        "L" => (-1, 0),
        _ => unreachable!(),
    };
    let delta = s.next().unwrap().parse().unwrap();
    (dir, delta)
}

fn solve<const N: usize>(lines: impl Iterator<Item = String>) {
    let mut history = HashSet::new();
    let mut ropes = [(0, 0); N];
    for line in lines {
        let (dir, delta) = parse(&line);
        for _ in 0..delta {
            ropes[0].0 += dir.0;
            ropes[0].1 += dir.1;
            for i in 1..N {
                let (h, t) = (ropes[i - 1], &mut ropes[i]);
                let (dx, dy) = (h.0 - t.0, h.1 - t.1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    t.0 += dx.signum();
                    t.1 += dy.signum();
                }
            }
            history.insert(ropes[N - 1]);
        }
    }
    println!("{:?}", history.len());
}

fn part1(lines: impl Iterator<Item = String>) {
    solve::<2>(lines);
}

fn part2(lines: impl Iterator<Item = String>) {
    solve::<10>(lines);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
