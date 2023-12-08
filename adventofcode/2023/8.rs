use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

type Nodes = HashMap<String, (String, String)>;

#[derive(Clone, Copy)]
enum Dir {
    R,
    L,
}

impl Dir {
    fn new(c: char) -> Self {
        match c {
            'R' => Self::R,
            'L' => Self::L,
            _ => unreachable!(),
        }
    }
}

fn parse(mut lines: impl Iterator<Item = String>) -> (Vec<Dir>, Nodes) {
    let dirs = lines.next().unwrap().chars().map(Dir::new).collect();
    lines.next().unwrap();
    let nodes = lines
        .map(|line| {
            let mut s = line.split(" = ");
            let node = s.next().unwrap().to_string();
            let pair = s.next().unwrap();
            let mut pair = pair
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ");
            let left = pair.next().unwrap().to_string();
            let right = pair.next().unwrap().to_string();
            (node, (left, right))
        })
        .collect();
    (dirs, nodes)
}

fn steps<'a>(
    mut current: &'a str,
    dirs: &[Dir],
    nodes: &'a Nodes,
    is_goal: fn(&str) -> bool,
) -> usize {
    let mut steps = 0;
    loop {
        for dir in dirs {
            let (left, right) = nodes.get(current).unwrap();
            current = match dir {
                Dir::R => right,
                Dir::L => left,
            };
            steps += 1;
            if is_goal(current) {
                return steps;
            }
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    let d = gcd(a, b);
    a / d * b
}

fn part1(lines: impl Iterator<Item = String>) {
    let (dirs, nodes) = parse(lines);
    println!("{}", steps("AAA", &dirs, &nodes, |n| n == "ZZZ"));
}

fn part2(lines: impl Iterator<Item = String>) {
    let (dirs, nodes) = parse(lines);
    let steps = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|n| steps(n, &dirs, &nodes, |n| n.ends_with('Z')))
        .reduce(lcm)
        .unwrap();
    println!("{steps}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
