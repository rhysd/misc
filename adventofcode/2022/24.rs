use arrayvec::ArrayVec;
use fxhash::FxHashSet;
use num::Integer;
use std::collections::VecDeque;
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    R,
    D,
    L,
}
impl Dir {
    fn to_char(self) -> char {
        match self {
            Dir::U => '^',
            Dir::R => '>',
            Dir::D => 'v',
            Dir::L => '<',
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Blizzard(ArrayVec<Dir, 4>),
    Ground,
    Wall,
}

type Field = Vec<Vec<Cell>>;

struct Sim {
    field: Vec<Field>,
    height: usize,
    width: usize,
    cycle: usize,
    start: Pos,
    goal: Pos,
}

impl Sim {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let field: Field = lines
            .map(|line| {
                line.into_bytes()
                    .into_iter()
                    .map(|b| match b {
                        b'.' => Cell::Ground,
                        b'>' => Cell::Blizzard([Dir::R].into_iter().collect()),
                        b'<' => Cell::Blizzard([Dir::L].into_iter().collect()),
                        b'^' => Cell::Blizzard([Dir::U].into_iter().collect()),
                        b'v' => Cell::Blizzard([Dir::D].into_iter().collect()),
                        b'#' => Cell::Wall,
                        _ => panic!("unknown cell {:?}", b as char),
                    })
                    .collect()
            })
            .collect();
        let start = (field[0].iter().position(|c| *c == Cell::Ground).unwrap(), 0);
        let last = field.len() - 1;
        let goal = (
            field[last].iter().position(|c| *c == Cell::Ground).unwrap(),
            last,
        );

        let mut field = vec![field];
        let width = field[0][0].len() - 2;
        let height = field[0].len() - 2;
        let cycle = width.lcm(&height);

        for _ in 1..cycle {
            let current = field.last().unwrap();
            let mut next: Field = current
                .iter()
                .map(|xs| {
                    xs.iter()
                        .map(|cell| match cell {
                            Cell::Blizzard(_) => Cell::Ground,
                            c => c.clone(),
                        })
                        .collect()
                })
                .collect();

            for (y, xs) in current.iter().enumerate() {
                for (x, cell) in xs.iter().enumerate() {
                    let Cell::Blizzard(dirs) = cell else {
                            continue;
                        };
                    for dir in dirs.iter().copied() {
                        use Dir::*;
                        let (x, y) = match dir {
                            U if y <= 1 => (x, height),
                            U => (x, y - 1),
                            R if x >= width => (1, y),
                            R => (x + 1, y),
                            D if y >= height => (x, 1),
                            D => (x, y + 1),
                            L if x <= 1 => (width, y),
                            L => (x - 1, y),
                        };
                        let cell = &mut next[y][x];
                        match cell {
                            Cell::Blizzard(dirs) => dirs.push(dir),
                            Cell::Ground => {
                                *cell = Cell::Blizzard([dir].into_iter().collect());
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }

            field.push(next);
        }

        Self {
            field,
            start,
            goal,
            width,
            height,
            cycle,
        }
    }

    fn solve(&mut self) -> usize {
        let (w, h) = (self.width, self.height);
        let mut queue = VecDeque::new();
        let mut visit = FxHashSet::default();
        let mut result = usize::MAX;
        queue.push_back((self.start, 0));

        while let Some((pos, minute)) = queue.pop_front() {
            let (x, y) = pos;
            let (gx, gy) = self.goal;
            let dist = gy.max(y) - gy.min(y) + gx.max(x) - gx.min(x);
            if minute + dist > result {
                continue;
            }

            let minute = minute + 1;
            if (x, y + 1) == self.goal || y > 0 && (x, y - 1) == self.goal {
                result = result.min(minute);
                continue;
            }

            let field = &self.field[minute % self.cycle];
            let adj = [
                (y < h).then_some((x, y + 1)),
                (x < w).then_some((x + 1, y)),
                (y > 1).then(|| (x, y - 1)),
                (x > 1).then(|| (x - 1, y)),
                Some((x, y)),
            ];
            for (x, y) in adj.into_iter().flatten() {
                if field[y][x] != Cell::Ground {
                    continue;
                }
                let key = (x, y, minute % self.cycle);
                if visit.contains(&key) {
                    continue;
                }
                visit.insert(key);
                queue.push_back(((x, y), minute));
            }
        }

        // Adjust `field` since `result` minutes passed
        self.field.rotate_left(result % self.cycle);

        result
    }

    #[allow(unused)]
    fn print(&mut self, minute: usize, me: Pos) {
        let f = &self.field[minute % self.cycle];
        for (y, xs) in f.iter().enumerate() {
            for (x, c) in xs.iter().enumerate() {
                let c = if (x, y) == me {
                    'E'
                } else {
                    match c {
                        Cell::Blizzard(dirs) if dirs.len() <= 1 => dirs[0].to_char(),
                        Cell::Blizzard(dirs) => char::from_digit(dirs.len() as u32, 10).unwrap(),
                        Cell::Ground => '.',
                        Cell::Wall => '#',
                    }
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut sim = Sim::parse(lines);
    println!("{}", sim.solve());
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut sim = Sim::parse(lines);
    let go = sim.solve();
    (sim.start, sim.goal) = (sim.goal, sim.start);
    let back = sim.solve();
    (sim.start, sim.goal) = (sim.goal, sim.start);
    let go_again = sim.solve();
    println!("{}", go + back + go_again);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
