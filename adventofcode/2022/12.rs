use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::io::{self, BufRead};

type Map = Vec<Vec<u8>>;
type Pos = (usize, usize);

fn neighbors(map: &Map, (x, y): Pos) -> [Option<(Pos, u8)>; 4] {
    let ylen = map.len();
    let xlen = map[0].len();
    let ymin = y.checked_sub(1);
    let ymax = (y < ylen - 1).then_some(y + 1);
    let xmin = x.checked_sub(1);
    let xmax = (x < xlen - 1).then_some(x + 1);
    [
        (Some(x), ymin),
        (xmax, Some(y)),
        (Some(x), ymax),
        (xmin, Some(y)),
    ]
    .map(|(x, y)| match (x, y) {
        (Some(x), Some(y)) => Some(((x, y), map[y][x])),
        _ => None,
    })
}

fn parse_map(lines: impl Iterator<Item = String>) -> (Map, Pos, Pos) {
    let mut me = Default::default();
    let mut goal = Default::default();
    let mut ys = vec![];
    for (y, line) in lines.enumerate() {
        let mut xs = vec![];
        for (x, b) in line.as_bytes().iter().copied().enumerate() {
            let b = match b {
                b'S' => {
                    me = (x, y);
                    b'a'
                }
                b'E' => {
                    goal = (x, y);
                    b'z'
                }
                b => b,
            };
            xs.push(b);
        }
        ys.push(xs);
    }
    (ys, me, goal)
}

#[derive(Debug)]
struct Vert {
    pos: Pos,
    height: u8,
    cost: usize,
}
impl Vert {
    fn new(pos: Pos, height: u8, cost: usize) -> Self {
        Self { pos, height, cost }
    }
}

impl PartialEq for Vert {
    fn eq(&self, rhs: &Vert) -> bool {
        self.cost == rhs.cost && self.height == rhs.height
    }
}
impl Eq for Vert {}
impl PartialOrd for Vert {
    fn partial_cmp(&self, rhs: &Vert) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Vert {
    fn cmp(&self, rhs: &Vert) -> Ordering {
        rhs.cost.cmp(&self.cost).then(self.height.cmp(&rhs.height))
    }
}

fn steps(map: Map, mut queue: BinaryHeap<Vert>, goal: Pos) -> Option<usize> {
    let mut visited = HashSet::new();
    while let Some(Vert { pos, height, cost }) = queue.pop() {
        if pos == goal {
            return Some(cost);
        }
        for (pos, h) in neighbors(&map, pos).into_iter().flatten() {
            if h <= height + 1 && !visited.contains(&pos) {
                visited.insert(pos);
                queue.push(Vert::new(pos, h, cost + 1));
            }
        }
    }
    None
}

fn part1(lines: impl Iterator<Item = String>) {
    let (map, me, goal) = parse_map(lines);
    let mut init = BinaryHeap::new();
    init.push(Vert::new(me, b'a', 0));
    println!("{}", steps(map, init, goal).unwrap());
}

fn part2(lines: impl Iterator<Item = String>) {
    let (map, _, goal) = parse_map(lines);
    let mut init = BinaryHeap::new();
    for (y, xs) in map.iter().enumerate() {
        for (x, h) in xs.iter().enumerate() {
            if *h == b'a' {
                init.push(Vert::new((x, y), b'a', 0));
            }
        }
    }
    println!("{}", steps(map, init, goal).unwrap());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
