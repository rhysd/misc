use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

#[derive(Eq, PartialEq)]
struct State {
    cost: u32,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let t: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut risks = Vec::with_capacity(t.len() * 5);
    for y in 0..5 {
        risks.extend(t.iter().map(|t| {
            let mut row = Vec::with_capacity(t.len() * 5);
            for x in 0..5 {
                row.extend(t.iter().map(|r| {
                    let r = *r + y as u32 + x as u32;
                    if r > 9 {
                        r - 9
                    } else {
                        r
                    }
                }));
            }
            row
        }));
    }
    let risks = risks;

    let start = (0, 0);
    let goal = (risks[0].len() - 1, risks.len() - 1);
    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    costs.insert(start, risks[0][0]);
    heap.push(State { cost: 0, pos: start });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == goal {
            println!("{}", cost);
            return;
        }

        if cost > costs[&pos] {
            continue;
        }

        let (x, y) = pos;
        for pos in [
            (x > 0).then(|| (x - 1, y)),
            (x + 1 < risks[y].len()).then(|| (x + 1, y)),
            (y > 0).then(|| (x, y - 1)),
            (y + 1 < risks.len()).then(|| (x, y + 1)),
        ]
        .into_iter()
        .flatten()
        {
            let (x, y) = pos;
            let cost = cost + risks[y][x];
            if let Some(c) = costs.get(&pos) {
                if cost >= *c {
                    continue;
                }
            }
            heap.push(State { cost, pos });
            costs.insert(pos, cost);
        }
    }
}
