use fxhash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::io::{self, BufRead};

type Idx = usize;
struct Valve {
    rate: usize,
    tunnel: Vec<Idx>,
}
type Valves = Vec<Valve>;

fn parse_input(lines: impl Iterator<Item = String>) -> (Idx, Valves) {
    fn parse_line(line: &str) -> (&str, (usize, Vec<&str>)) {
        let s = line.strip_prefix("Valve ").unwrap();
        let mut s = s.split(" has flow rate=");
        let name = s.next().unwrap();
        let s = s.next().unwrap();
        let mut s = if s.contains("; tunnels lead to valves ") {
            s.split("; tunnels lead to valves ")
        } else {
            s.split("; tunnel leads to valve ")
        };
        let rate = s.next().unwrap().parse().unwrap();
        let s = s.next().unwrap();
        let tunnel = s.split(", ").collect();
        (name, (rate, tunnel))
    }

    let lines: Vec<_> = lines.collect();
    let mut valves = Vec::new();
    let mut indices = HashMap::new();
    for (idx, line) in lines.iter().enumerate() {
        let (k, v) = parse_line(line);
        indices.insert(k, idx);
        valves.push(v);
    }

    let valves = valves
        .into_iter()
        .map(|(rate, tunnel)| Valve {
            rate,
            tunnel: tunnel.into_iter().map(|k| indices[k]).collect(),
        })
        .collect();
    (indices["AA"], valves)
}

fn shortest(start: Idx, goal: Idx, valves: &Valves) -> Option<usize> {
    struct Vert {
        idx: Idx,
        cost: usize,
    }

    impl PartialEq for Vert {
        fn eq(&self, rhs: &Vert) -> bool {
            self.cost == rhs.cost
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
            rhs.cost.cmp(&self.cost)
        }
    }

    let mut queue = BinaryHeap::new();
    queue.push(Vert {
        idx: start,
        cost: 0,
    });
    let mut visit = FxHashSet::default();
    visit.insert(start);

    while let Some(Vert { idx, cost }) = queue.pop() {
        if idx == goal {
            return Some(cost);
        }
        for &idx in valves[idx].tunnel.iter() {
            if visit.contains(&idx) {
                continue;
            }
            visit.insert(idx);
            queue.push(Vert {
                idx,
                cost: cost + 1,
            });
        }
    }

    None
}

type Distances = Vec<Vec<usize>>;
fn distances(valves: &Valves) -> Distances {
    (0..valves.len())
        .map(|i1| {
            (0..valves.len())
                .map(|i2| shortest(i1, i2, valves).unwrap())
                .collect()
        })
        .collect()
}

fn solve1(
    start: Idx,
    valves: &Valves,
    remain: usize,
    closed: &mut Vec<Idx>,
    dist: &Distances,
) -> usize {
    if closed.is_empty() || remain <= 1 {
        return 0;
    }

    let d = dist[start].as_slice();
    closed
        .to_vec()
        .into_iter()
        .filter_map(|idx| {
            let cost = d[idx];
            (remain > cost + 1).then_some((idx, cost))
        })
        .map(|(idx, cost)| {
            let v = &valves[idx];
            let remain = remain - cost - 1;
            let mut pressure = remain * v.rate;
            if remain > 2 {
                closed.swap_remove(closed.iter().position(|&i| idx == i).unwrap());
                pressure += solve1(idx, valves, remain, closed, dist);
                closed.push(idx);
            }
            pressure
        })
        .max()
        .unwrap_or(0)
}

fn part1(lines: impl Iterator<Item = String>) {
    let (start, valves) = parse_input(lines);
    let dist = distances(&valves);
    let mut closed = valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (v.rate > 0).then_some(i))
        .collect();
    println!("{}", solve1(start, &valves, 30, &mut closed, &dist));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    idx: Idx,
    remain: usize,
}
impl Player {
    fn new(idx: Idx, remain: usize) -> Self {
        Self { idx, remain }
    }
}

fn solve2(
    me: Player,
    you: Player,
    valves: &Valves,
    closed: &mut Vec<Idx>, // Note: O(n) lookup with Vec is faster than O(1) lookup with HashSet in this case
    dist: &Distances,
    memo: &mut FxHashMap<(Player, Player, Vec<Idx>), usize>,
) -> usize {
    if closed.is_empty() {
        return 0;
    }

    closed.sort_unstable();
    let mut state = (me.clone(), you.clone(), closed.clone());
    if let Some(&p) = memo.get(&state) {
        return p;
    }
    std::mem::swap(&mut state.0, &mut state.1); // 2 players are symmertry
    if let Some(&p) = memo.get(&state) {
        return p;
    }
    let cloned = state.2;

    let prepare_dist = |idx: Idx| {
        let dist = dist[idx].as_slice();
        let min = closed.iter().copied().map(|i| dist[i]).min().unwrap_or(0);
        (dist, min)
    };

    let (dist_me, min_dist_me) = prepare_dist(me.idx);
    let (dist_you, min_dist_you) = prepare_dist(you.idx);

    let mut max = 0;
    for idx_me in cloned.iter().copied() {
        for idx_you in cloned.iter().copied() {
            if idx_me == idx_you {
                continue;
            }
            let mut pressure = 0;

            macro_rules! next {
                ($player:ident, $idx:ident, $dist:ident, $min_dist:ident) => {
                    'blk: {
                        let cost = $dist[$idx];
                        if $player.remain <= cost + 1 {
                            break 'blk None;
                        }
                        let remain = $player.remain - cost - 1;
                        pressure += remain * valves[$idx].rate;
                        if remain <= 2 + $min_dist {
                            break 'blk None;
                        }
                        closed.swap_remove(closed.iter().position(|&i| $idx == i).unwrap());
                        Some(Player::new($idx, remain))
                    }
                };
            }

            let me = next!(me, idx_me, dist_me, min_dist_me);
            let you = next!(you, idx_you, dist_you, min_dist_you);

            match (me, you) {
                (Some(me), Some(you)) => {
                    pressure += solve2(me.clone(), you.clone(), valves, closed, dist, memo);
                    closed.push(me.idx);
                    closed.push(you.idx);
                }
                (Some(pl), _) | (_, Some(pl)) => {
                    pressure += solve1(pl.idx, valves, pl.remain, closed, dist);
                    closed.push(pl.idx);
                }
                _ => continue,
            }

            max = max.max(pressure)
        }
    }
    memo.insert((me, you, cloned), max);
    max
}

fn part2(lines: impl Iterator<Item = String>) {
    let (start, valves) = parse_input(lines);
    let dist = distances(&valves);
    let mut closed = valves
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (v.rate > 0).then_some(i))
        .collect();
    let me = Player::new(start, 26);
    let you = Player::new(start, 26);
    let pressure = solve2(
        me,
        you,
        &valves,
        &mut closed,
        &dist,
        &mut FxHashMap::default(),
    );
    println!("{}", pressure);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
