use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

struct Vert {
    cost: usize,
    dir: Dir,
    count: u8,
    pos: Pos,
}
impl PartialEq for Vert {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for Vert {}
impl Ord for Vert {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Vert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(lines: impl Iterator<Item = String>) -> Vec<Vec<usize>> {
    lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part1(lines: impl Iterator<Item = String>) {
    let costs = parse(lines);
    let (x_max, y_max) = (costs[0].len() - 1, costs.len() - 1);
    let goal = (x_max, y_max);

    let mut queue = BinaryHeap::from([Vert {
        cost: 0,
        dir: Dir::R,
        count: 0,
        pos: (0, 0),
    }]);
    let mut seen = HashSet::new();

    while let Some(Vert {
        cost,
        dir,
        count,
        pos,
    }) = queue.pop()
    {
        if pos == goal {
            println!("{cost}");
            return;
        }

        let (x, y) = pos;
        let adjucents = [
            y.checked_sub(1).map(|y| (x, y, Dir::U)),
            (x < x_max).then_some((x + 1, y, Dir::R)),
            (y < y_max).then_some((x, y + 1, Dir::D)),
            x.checked_sub(1).map(|x| (x, y, Dir::L)),
        ];
        for (x, y, d) in adjucents
            .into_iter()
            .flatten()
            .filter(|&(_, _, d)| d != dir.inv() && (count < 3 || d != dir))
        {
            let count = if dir == d { count + 1 } else { 1 };
            let state = (x, y, d, count);
            if seen.contains(&state) {
                continue;
            }
            seen.insert(state);
            queue.push(Vert {
                cost: cost + costs[y][x],
                dir: d,
                count,
                pos: (x, y),
            });
        }
    }

    unreachable!("answer not found")
}

fn part2(lines: impl Iterator<Item = String>) {
    let costs = parse(lines);
    let (x_max, y_max) = (costs[0].len() - 1, costs.len() - 1);
    let goal = (x_max, y_max);

    let mut queue = BinaryHeap::from([Dir::R, Dir::D].map(|dir| Vert {
        cost: 0,
        dir,
        count: 0,
        pos: (0, 0),
    }));
    let mut seen = HashSet::new();

    while let Some(Vert {
        cost,
        dir,
        count,
        pos,
    }) = queue.pop()
    {
        if pos == goal && count >= 4 {
            println!("{cost}");
            return;
        }

        let (x, y) = pos;
        let adjucents = [
            y.checked_sub(1).map(|y| (x, y, Dir::U)),
            (x < x_max).then_some((x + 1, y, Dir::R)),
            (y < y_max).then_some((x, y + 1, Dir::D)),
            x.checked_sub(1).map(|x| (x, y, Dir::L)),
        ];
        for (x, y, d) in adjucents.into_iter().flatten().filter(|&(_, _, d)| {
            !(d == dir.inv() || count < 4 && d != dir || count >= 10 && d == dir)
        }) {
            let count = if dir == d { count + 1 } else { 1 };
            let state = (x, y, d, count);
            if seen.contains(&state) {
                continue;
            }
            seen.insert(state);
            queue.push(Vert {
                cost: cost + costs[y][x],
                dir: d,
                count,
                pos: (x, y),
            });
        }
    }

    unreachable!("answer not found")
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
