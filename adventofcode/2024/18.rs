use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

const W: usize = 71;
const H: usize = 71;

fn steps(bytes: &HashSet<Pos>) -> Option<u32> {
    struct State {
        pos: Pos,
        steps: u32,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.steps.cmp(&self.steps)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl Eq for State {}

    let mut queue = BinaryHeap::from([State { pos: (0, 0), steps: 0 }]);
    let mut dist = HashMap::from([((0, 0), 0)]);

    while let Some(State { pos, steps }) = queue.pop() {
        if pos == (W - 1, H - 1) {
            return Some(steps);
        }
        if let Some(s) = dist.get(&pos) {
            if *s < steps {
                continue;
            }
        }
        let (x, y) = pos;
        let steps = steps + 1;
        for pos in [
            (x > 0).then(|| (x - 1, y)),
            (y > 0).then(|| (x, y - 1)),
            (W > x + 1).then(|| (x + 1, y)),
            (H > y + 1).then(|| (x, y + 1)),
        ]
        .into_iter()
        .flatten()
        {
            if !bytes.contains(&pos) && dist.get(&pos).map(|&s| steps < s).unwrap_or(true) {
                dist.insert(pos, steps);
                queue.push(State { pos, steps });
            }
        }
    }

    None
}

fn part1(lines: impl Iterator<Item = String>) {
    let bytes: HashSet<Pos> = lines
        .take(1024)
        .map(|l| {
            let mut s = l.split(',');
            (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
        })
        .collect();
    println!("{}", steps(&bytes).unwrap());
}

fn part2(lines: impl Iterator<Item = String>) {
    let all_bytes: Vec<Pos> = lines
        .map(|l| {
            let mut s = l.split(',');
            (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
        })
        .collect();

    fn bin_search(all: &[Pos], lo: usize, hi: usize) -> usize {
        if hi - lo <= 1 {
            return lo;
        }
        let mid = (lo + hi) / 2;
        let bytes: HashSet<_> = all.iter().take(mid).copied().collect();
        if steps(&bytes).is_some() {
            bin_search(all, mid, hi)
        } else {
            bin_search(all, lo, mid)
        }
    }

    let (x, y) = all_bytes[bin_search(&all_bytes, 0, all_bytes.len())];
    println!("{x},{y}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
