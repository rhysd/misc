use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);
type Nodes = HashMap<Pos, Vec<Pos>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    U,
    R,
    D,
    L,
}
impl Dir {
    fn inv(self) -> Self {
        use Dir::*;
        match self {
            U => D,
            D => U,
            L => R,
            R => L,
        }
    }
}

fn dirs(c: char) -> &'static [Dir] {
    use Dir::*;
    match c {
        '|' => &[U, D],
        '-' => &[L, R],
        'L' => &[U, R],
        'J' => &[U, L],
        '7' => &[D, L],
        'F' => &[D, R],
        '.' => &[],
        'S' => &[U, R, D, L],
        _ => unreachable!(),
    }
}

fn parse(lines: impl Iterator<Item = String>) -> (Pos, Nodes) {
    let ys: Vec<Vec<_>> = lines.map(|l| l.chars().map(dirs).collect()).collect();

    let mut start = (0, 0);
    let mut nodes = Nodes::new();
    for y in 0..ys.len() {
        let xs = &ys[y];
        for x in 0..xs.len() {
            let dirs = xs[x];
            if dirs.len() == 4 {
                start = (x, y);
            }

            let neighbors = [
                y.checked_sub(1).map(|y| (x, y, Dir::U)),
                x.checked_sub(1).map(|x| (x, y, Dir::L)),
                (xs.len() > x + 1).then_some((x + 1, y, Dir::R)),
                (ys.len() > y + 1).then_some((x, y + 1, Dir::D)),
            ];
            for (nx, ny, dir) in neighbors.into_iter().flatten() {
                if dirs.contains(&dir) && ys[ny][nx].contains(&dir.inv()) {
                    nodes.entry((x, y)).or_default().push((nx, ny));
                }
            }
        }
    }

    (start, nodes)
}

fn costs(start: Pos, nodes: &Nodes) -> HashMap<Pos, usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut costs = HashMap::<Pos, usize>::new();

    while let Some((pos, cost)) = queue.pop_front() {
        for pos in nodes[&pos].iter().copied() {
            if costs.contains_key(&pos) {
                continue;
            }
            let cost = cost + 1;
            costs.insert(pos, cost);
            queue.push_back((pos, cost));
        }
    }

    costs
}

fn part1(lines: impl Iterator<Item = String>) {
    let (start, nodes) = parse(lines);
    let costs = costs(start, &nodes);
    let max: usize = costs.into_values().max().unwrap();
    println!("{max}");
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Bit {
    In,
    Out,
    Pipe,
}

fn part2(lines: impl Iterator<Item = String>) {
    let (start, mut nodes) = parse(lines);
    let costs = costs(start, &nodes);
    nodes.retain(|k, _| costs.contains_key(k)); // Retain nodes only in the loop
    let nodes = nodes;

    let (max_x, max_y) = nodes.iter().fold((0, 0), |(max_x, max_y), ((x, y), _)| {
        (cmp::max(max_x, *x), cmp::max(max_y, *y))
    });
    let x_len = (max_x + 1) * 2 + 1;
    let y_len = (max_y + 1) * 2 + 1;
    let mut bits = vec![vec![Bit::In; x_len]; y_len]; // Assume all vacant bits are inside at start

    for ((x, y), neighbors) in nodes.iter().map(|(&k, v)| (k, v.iter().copied())) {
        let (x, y) = (x * 2 + 1, y * 2 + 1);
        bits[y][x] = Bit::Pipe;
        for (nx, ny) in neighbors {
            let (nx, ny) = (nx * 2 + 1, ny * 2 + 1);
            let (x, y) = if x != nx {
                (cmp::min(x, nx) + 1, y)
            } else if y != ny {
                (x, cmp::min(y, ny) + 1)
            } else {
                unreachable!();
            };
            bits[y][x] = Bit::Pipe;
        }
    }

    let mut queue = VecDeque::new();
    // All edge bits are trivially outside. Start from them
    for x in 0..x_len {
        queue.push_back((x, 0));
        queue.push_back((x, y_len - 1));
    }
    for y in 1..y_len - 1 {
        queue.push_back((0, y));
        queue.push_back((x_len - 1, y));
    }

    // Fill all the outside bits
    while let Some((x, y)) = queue.pop_front() {
        if bits[y][x] != Bit::In {
            continue;
        }
        bits[y][x] = Bit::Out;
        let px = x.checked_sub(1);
        let nx = (x + 1 < x_len).then_some(x + 1);
        let py = y.checked_sub(1);
        let ny = (y + 1 < y_len).then_some(y + 1);
        let x = Some(x);
        let y = Some(y);
        #[rustfmt::skip]
        let adjacents = [
            (px, py), (x, py), (nx, py),
            (px,  y),          (nx,  y),
            (px, ny), (x, ny), (nx, ny),
        ];
        for adj in adjacents {
            if let (Some(x), Some(y)) = adj {
                queue.push_back((x, y));
            }
        }
    }

    let mut total = 0usize;
    for y in 0..max_y {
        for x in 0..max_x {
            let (x, y) = (x * 2 + 1, y * 2 + 1);
            if bits[y][x] == Bit::In {
                total += 1;
            }
        }
    }
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
