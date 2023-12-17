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

    let init = [
        Vert {
            cost: costs[0][1..=4].iter().sum(),
            dir: Dir::R,
            count: 4,
            pos: (4, 0),
        },
        Vert {
            cost: (1..=4).map(|y| costs[y][0]).sum(),
            dir: Dir::D,
            count: 4,
            pos: (0, 4),
        },
    ];
    let mut seen: HashSet<_> = init
        .iter()
        .map(|v| (v.pos.0, v.pos.1, v.dir, v.count))
        .collect();
    let mut queue = BinaryHeap::from(init);

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
        let (ldir, rdir) = match dir {
            Dir::U | Dir::D => (Dir::L, Dir::R),
            Dir::L | Dir::R => (Dir::U, Dir::D),
        };
        for vert in [(true, dir), (false, ldir), (false, rdir)]
            .into_iter()
            .filter_map(|(consecutive, dir)| {
                let delta = if consecutive { 1 } else { 4 };
                let pos = match dir {
                    Dir::U => y.checked_sub(delta).map(|y| (x, y)),
                    Dir::R => (x + delta <= x_max).then_some((x + delta, y)),
                    Dir::D => (y + delta <= y_max).then_some((x, y + delta)),
                    Dir::L => x.checked_sub(delta).map(|x| (x, y)),
                }?;

                let (nx, ny) = pos;
                if consecutive {
                    return (count < 10).then(|| Vert {
                        cost: cost + costs[ny][nx],
                        dir,
                        count: count + 1,
                        pos,
                    });
                }

                let cost_delta: usize = match dir {
                    Dir::U => (ny..y).map(|y| costs[y][nx]).sum(),
                    Dir::R => costs[y][x + 1..=nx].iter().sum(),
                    Dir::D => (y + 1..=ny).map(|y| costs[y][nx]).sum(),
                    Dir::L => costs[y][nx..x].iter().sum(),
                };
                Some(Vert {
                    cost: cost + cost_delta,
                    dir,
                    count: 4,
                    pos,
                })
            })
        {
            let state = (vert.pos.0, vert.pos.1, vert.dir, vert.count);
            if seen.contains(&state) {
                continue;
            }
            seen.insert(state);
            queue.push(vert);
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
