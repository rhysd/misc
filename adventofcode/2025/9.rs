use std::env;
use std::io::{self, BufRead};

type Pos = (u64, u64);

#[derive(Debug)]
struct Rect(u64, u64, u64, u64);

impl Rect {
    fn new((x1, y1): Pos, (x2, y2): Pos) -> Self {
        Self(x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
    }

    fn area(&self) -> u64 {
        let Self(x1, y1, x2, y2) = *self;
        (x2 - x1 + 1) * (y2 - y1 + 1)
    }

    fn corners(&self) -> [Pos; 4] {
        let Self(x1, y1, x2, y2) = *self;
        [(x1, y1), (x1, y2), (x2, y2), (x2, y1)]
    }

    fn inner_rect(&self) -> Self {
        let Self(x1, y1, x2, y2) = *self;
        Self(x1 + 1, y1 + 1, x2 - 1, y2 - 1)
    }
}

fn parse(lines: impl Iterator<Item = String>) -> Vec<Pos> {
    lines
        .map(|l| {
            let mut s = l.split(',').map(|s| s.parse().unwrap());
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect()
}

fn part1(lines: impl Iterator<Item = String>) {
    let tiles = parse(lines);
    let mut max = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            max = max.max(Rect::new(tiles[i], tiles[j]).area());
        }
    }
    println!("{max}");
}

#[derive(Debug)]
enum Edge {
    Virt(u64, (u64, u64)),
    Horz((u64, u64), u64),
}

impl Edge {
    fn new((x1, y1): Pos, (x2, y2): Pos) -> Self {
        if x1 == x2 {
            Self::Virt(x1, (y1.min(y2), y1.max(y2)))
        } else if y1 == y2 {
            Self::Horz((x1.min(x2), x1.max(x2)), y1)
        } else {
            unreachable!()
        }
    }

    fn contains(&self, p: Pos) -> bool {
        match *self {
            Self::Virt(x, (ys, ye)) => x == p.0 && ys <= p.1 && p.1 <= ye,
            Self::Horz((xs, xe), y) => y == p.1 && xs <= p.0 && p.0 <= xe,
        }
    }

    fn intersects(&self, rect: &Rect) -> bool {
        let Rect(x1, y1, x2, y2) = *rect;
        match *self {
            Self::Virt(x, (ys, ye)) => x1 <= x && x <= x2 && !(ye < y1 || y2 < ys),
            Self::Horz((xs, xe), y) => y1 <= y && y <= y2 && !(xe < x1 || x2 < xs),
        }
    }
}

fn is_inside(p: Pos, edges: &[Edge]) -> bool {
    if edges.iter().any(|e| e.contains(p)) {
        return true;
    }
    let (px, py) = p;
    let mut count = 0;
    for edge in edges {
        if let Edge::Virt(x, (ys, ye)) = *edge
            && px < x
            && ys <= p.1
            && py < ye
        {
            count += 1;
        }
    }
    count % 2 == 1
}

fn intersects_inner_rect(rect: &Rect, edges: &[Edge]) -> bool {
    let inner = rect.inner_rect();
    edges.iter().any(|e| e.intersects(&inner))
}

fn part2(lines: impl Iterator<Item = String>) {
    let tiles = parse(lines);

    let edges: Vec<_> = (0..tiles.len())
        .map(|i| {
            let j = (i + 1) % tiles.len();
            Edge::new(tiles[i], tiles[j])
        })
        .collect();

    let mut max = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let rect = Rect::new(tiles[i], tiles[j]);
            let area = rect.area();
            // XXX: This check is not sufficient. For example
            //
            //      ..........................
            //      ..##..................##..
            //      ..XX..................XX..
            //      ..XX..................XX..
            //      ..XX..................XX..
            //      ..X#XXXXXXXXXXXXXXXXXX#X..
            //      ..#XXXXXXXXXXXXXXXXXXXX#..
            //      ..........................
            //
            if max < area
                && rect.corners().into_iter().all(|p| is_inside(p, &edges))
                && !intersects_inner_rect(&rect, &edges)
            {
                max = area;
            }
        }
    }
    println!("{max}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
