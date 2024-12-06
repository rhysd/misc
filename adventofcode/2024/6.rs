use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn turn_right(self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    fn forward(self, (x, y): Pos) -> Option<Pos> {
        Some(match self {
            Self::U => (x, y.checked_sub(1)?),
            Self::R => (x + 1, y),
            Self::D => (x, y + 1),
            Self::L => (x.checked_sub(1)?, y),
        })
    }
}

#[derive(Clone, Copy)]
enum Action {
    Turn(Dir),
    Forward(Pos),
}

#[derive(Clone)]
struct Problem {
    cells: Vec<Vec<bool>>,
    dir: Dir,
    pos: Pos,
    path: HashSet<(Dir, (usize, usize))>,
}

impl Problem {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut path = HashSet::new();
        let mut pos = (0, 0);
        let cells = lines
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '^' => {
                            pos = (x, y);
                            path.insert((Dir::U, pos));
                            true
                        }
                        '.' => true,
                        '#' => false,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Self { dir: Dir::U, pos, cells, path }
    }

    fn action(&self) -> Option<Action> {
        let (x, y) = self.dir.forward(self.pos)?;
        let action = if *self.cells.get(y)?.get(x)? {
            Action::Forward((x, y))
        } else {
            Action::Turn(self.dir.turn_right())
        };
        Some(action)
    }

    fn solve(&mut self) -> bool {
        while let Some(action) = self.action() {
            match action {
                Action::Turn(dir) => self.dir = dir,
                Action::Forward(pos) if self.path.insert((self.dir, pos)) => self.pos = pos,
                Action::Forward(_) => return false,
            }
        }
        true
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut problem = Problem::parse(lines);
    assert!(problem.solve());
    let count = problem.path.into_iter().map(|(_, p)| p).collect::<HashSet<_>>().len();
    println!("{count}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let init = Problem::parse(lines);

    let mut solved = init.clone();
    assert!(solved.solve());

    let count = solved
        .path
        .into_iter()
        .map(|(_, p)| p)
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|&(x, y)| {
            let mut problem = init.clone();
            problem.cells[y][x] = false;
            !problem.solve()
        })
        .count();
    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
