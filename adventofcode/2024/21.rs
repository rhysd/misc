use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{self, BufRead};
use std::mem;
use std::rc::Rc;

const DIR_BUTTONS: [u8; 5] = [b'^', b'>', b'v', b'<', b'A'];

type Buttons = HashMap<u8, [Option<u8>; 4]>;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
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

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
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

struct PathNode(u8, Option<Rc<PathNode>>);

#[derive(Default, Clone)]
struct Path(Option<Rc<PathNode>>);

impl Path {
    fn prepend(&mut self, b: u8) {
        let head = mem::take(&mut self.0);
        self.0 = Some(Rc::new(PathNode(b, head)));
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut ret = vec![];
        let mut cur = &self.0;
        while let Some(PathNode(pos, next)) = cur.as_deref() {
            ret.push(*pos);
            cur = next;
        }
        ret.reverse();
        ret
    }
}

fn shortest_paths(pad: &Buttons, start: u8, goal: u8) -> Vec<Vec<u8>> {
    struct State {
        pos: u8,
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

    let mut queue = VecDeque::from([State { pos: start, cost: 0, path: Path::default() }]);
    let mut dist = HashMap::new();
    let mut ret = vec![];
    while let Some(State { pos, path, cost }) = queue.pop_front() {
        if pos == goal {
            let mut path = path.to_vec();
            path.push(b'A');
            ret.push(path);
            continue;
        }

        if let Some(&c) = dist.get(&pos) {
            if cost > c {
                continue;
            }
        }

        let cost = cost + 1;
        for (i, pos) in pad[&pos].iter().enumerate() {
            let Some(pos) = *pos else {
                continue;
            };

            if let Some(&c) = dist.get(&pos) {
                if cost > c {
                    continue;
                }
            }

            let mut path = path.clone();
            path.prepend(DIR_BUTTONS[i]);

            dist.insert(pos, cost);
            queue.push_back(State { pos, cost, path });
        }
    }

    ret
}

fn precompute_dir_seqs() -> HashMap<(u8, u8), Vec<Vec<u8>>> {
    let mut ret = HashMap::new();
    let pad = directional_pad();
    for from in DIR_BUTTONS {
        for to in DIR_BUTTONS {
            ret.insert((from, to), shortest_paths(&pad, from, to));
        }
    }
    ret
}

fn solve_directional<'a>(
    input: &'a [u8],
    remaining: u8,
    all_dir_seqs: &'a HashMap<(u8, u8), Vec<Vec<u8>>>,
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
        ret += all_dir_seqs[&(from, to)]
            .iter()
            .map(|seq| solve_directional(seq, remaining - 1, all_dir_seqs, cache))
            .min()
            .unwrap();
        from = to;
    }
    cache.insert((input, remaining), ret);
    ret
}

fn solve(num_dir_robots: u8, lines: impl Iterator<Item = String>) -> u64 {
    // Precompute all initial sequences to reuse cache accross all codes with making the borrow checker happy.
    let numeric = numeric_pad();
    let inits: Vec<_> = lines
        .map(|l| {
            let mut from = b'A';
            let mut init = vec![];
            for &to in l.as_bytes() {
                init.push(shortest_paths(&numeric, from, to));
                from = to;
            }
            let num: u64 = l.strip_suffix('A').unwrap_or(&l).parse().unwrap();
            (num, init)
        })
        .collect();

    let all_dir_seqs = precompute_dir_seqs();
    let mut cache = HashMap::new();
    inits
        .iter()
        .map(|(num, init)| {
            let min_len: u64 = init
                .iter()
                .map(|seqs| {
                    seqs.iter()
                        .map(|seq| {
                            solve_directional(seq, num_dir_robots, &all_dir_seqs, &mut cache)
                        })
                        .min()
                        .unwrap()
                })
                .sum();
            *num * min_len
        })
        .sum()
}

fn part1(lines: impl Iterator<Item = String>) {
    println!("{}", solve(2, lines));
}

fn part2(lines: impl Iterator<Item = String>) {
    println!("{}", solve(25, lines));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
