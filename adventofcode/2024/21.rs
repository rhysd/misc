use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::io::{self, BufRead};
use std::mem;
use std::rc::Rc;

type Buttons = HashMap<u8, [Option<u8>; 4]>;

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Dir {
    U = b'^',
    R = b'>',
    D = b'v',
    L = b'<',
}

impl Dir {
    fn new(idx: usize) -> Self {
        match idx {
            0 => Self::U,
            1 => Self::R,
            2 => Self::D,
            3 => Self::L,
            _ => unreachable!(),
        }
    }

    fn distance_a(self) -> u64 {
        match self {
            Self::U => 1,
            Self::R => 1,
            Self::D => 2,
            Self::L => 3,
        }
    }

    fn distance(self, to: Self) -> u64 {
        use Dir::*;
        let move_cost = match (self, to) {
            (U, R) | (R, U) | (U, L) | (L, U) | (R, L) | (L, R) => 2,
            (U, D) | (D, U) | (R, D) | (D, R) | (D, L) | (L, D) => 1,
            (U, U) | (R, R) | (D, D) | (L, L) => 0,
        };
        move_cost + to.distance_a()
    }
}

#[rustfmt::skip]
fn numeric_pad() -> Buttons {
    HashMap::from([
        //     Up           Right       Down        Left
        (b'7', [None,       Some(b'8'), Some(b'4'), None]),
        (b'8', [None,       Some(b'9'), Some(b'5'), Some(b'7')]),
        (b'9', [None,       None,       Some(b'6'), Some(b'8')]),
        (b'4', [Some(b'7'), Some(b'5'), Some(b'1'), None]),
        (b'5', [Some(b'8'), Some(b'6'), Some(b'2'), Some(b'4')]),
        (b'6', [Some(b'9'), None,       Some(b'3'), Some(b'5')]),
        (b'1', [Some(b'4'), Some(b'2'), None,       None]),
        (b'2', [Some(b'5'), Some(b'3'), Some(b'0'), Some(b'1')]),
        (b'3', [Some(b'6'), None,       Some(b'A'), Some(b'2')]),
        (b'0', [Some(b'2'), Some(b'A'), None,       None]),
        (b'A', [Some(b'3'), None,       None,       Some(b'0')]),
    ])
}

#[rustfmt::skip]
fn directional_pad() -> Buttons {
    HashMap::from([
        //     Up           Right       Down        Left
        (b'^', [None,       Some(b'A'), Some(b'v'), None]),
        (b'A', [None,       None,       Some(b'>'), Some(b'^')]),
        (b'<', [None,       Some(b'v'), None,       None]),
        (b'v', [Some(b'^'), Some(b'>'), None,       Some(b'<')]),
        (b'>', [Some(b'A'), None,       None,       Some(b'v')]),
    ])
}

#[derive(Default)]
struct PathNode(u8, Option<Rc<PathNode>>);

#[derive(Default, Clone)]
struct Path(Option<Rc<PathNode>>);

impl Path {
    fn prepend(&mut self, b: u8) {
        let head = mem::take(&mut self.0);
        self.0 = Some(Rc::new(PathNode(b, head)));
    }

    fn extend(&self, dest: &mut Vec<u8>) {
        let start = dest.len();
        let mut cur = &self.0;
        while let Some(PathNode(pos, next)) = cur.as_deref() {
            dest.push(*pos);
            cur = next;
        }
        dest[start..].reverse();
    }
}

fn shortest_path(pad: &Buttons, start: u8, goal: u8) -> Path {
    if start == goal {
        let mut path = Path::default();
        path.prepend(b'A');
        return path;
    }

    struct State {
        pos: u8,
        dir: Dir,
        path: Path,
        cost: u64,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
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

    let mut queue = BinaryHeap::new();
    for (i, b) in pad[&start].iter().enumerate() {
        if b.is_some() {
            let dir = Dir::new(i);
            queue.push(State { pos: start, cost: 0, path: Path::default(), dir });
        }
    }

    let mut dist = HashMap::new();
    while let Some(State { pos, path, dir, cost }) = queue.pop() {
        if pos == goal {
            return path;
        }

        if let Some(&c) = dist.get(&pos) {
            if cost > c {
                continue;
            }
        }

        for (i, b) in pad[&pos].iter().enumerate() {
            let Some(b) = *b else {
                continue;
            };

            let next_dir = Dir::new(i);
            let mut cost = cost + dir.distance(next_dir);
            if b == goal {
                cost += next_dir.distance_a();
            }
            if let Some(&c) = dist.get(&b) {
                if c < cost {
                    continue;
                }
            }

            let mut path = path.clone();
            path.prepend(next_dir as u8);
            if b == goal {
                path.prepend(b'A');
            }

            dist.insert(b, cost);
            queue.push(State { pos: b, dir: next_dir, cost, path });
        }
    }

    unreachable!("could not reach")
}

fn shortest_seq(pad: &Buttons, input: &[u8]) -> Vec<u8> {
    let mut seq = vec![];
    let mut from = b'A';
    for &to in input {
        shortest_path(pad, from, to).extend(&mut seq);
        from = to;
    }
    seq
}

fn part1(lines: impl Iterator<Item = String>) {
    let numeric = numeric_pad();
    let directional = directional_pad();
    let total: u64 = lines
        .map(|l| {
            let input = l.as_bytes();
            let num = l.strip_suffix('A').unwrap_or(&l).parse::<u64>().unwrap();

            let mut seq = shortest_seq(&numeric, input);
            for _ in 0..2 {
                seq = shortest_seq(&directional, &seq);
            }

            seq.len() as u64 * num
        })
        .sum();
    println!("{total}");
}

fn dir_seqs() -> HashMap<(u8, u8), Vec<u8>> {
    const BUTTONS: [u8; 5] = [Dir::U as u8, Dir::R as u8, Dir::D as u8, Dir::L as u8, b'A'];
    let mut ret = HashMap::new();
    let pad = directional_pad();
    for from in BUTTONS {
        for to in BUTTONS {
            let mut seq = vec![];
            shortest_path(&pad, from, to).extend(&mut seq);
            ret.insert((from, to), seq);
        }
    }
    ret
}

fn solve_directional<'a>(
    input: &'a [u8],
    remaining: u8,
    dir_seqs: &'a HashMap<(u8, u8), Vec<u8>>,
    cache: &mut HashMap<(&'a [u8], u8), u64>,
) -> u64 {
    if remaining == 0 {
        return input.len() as u64;
    }
    if let Some(&len) = cache.get(&(input, remaining)) {
        return len;
    }
    let mut ret = 0;
    let mut from = b'A';
    for &to in input {
        ret += solve_directional(&dir_seqs[&(from, to)], remaining - 1, dir_seqs, cache);
        from = to;
    }
    cache.insert((input, remaining), ret);
    ret
}

fn part2(lines: impl Iterator<Item = String>) {
    // Precompute all initial sequences to reuse cache accross all computations making the borrow checker happy.
    let numeric = numeric_pad();
    let inits: Vec<_> = lines
        .map(|l| {
            let num = l.strip_suffix('A').unwrap_or(&l).parse::<u64>().unwrap();
            (num, shortest_seq(&numeric, l.as_bytes()))
        })
        .collect();

    let dir_seqs = dir_seqs();
    let mut cache = HashMap::new();
    let total: u64 = inits
        .iter()
        .map(|(num, init)| {
            let len = solve_directional(init, 25, &dir_seqs, &mut cache);
            len * *num
        })
        .sum();
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
