use std::cmp;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    R,
    D,
    L,
}

enum Passed {
    Continue(Dir),
    Split(Dir, Dir),
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    RMirror, // \
    LMirror, // /
    VSplit,  // |
    HSplit,  // -
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '\\' => Self::RMirror,
            '/' => Self::LMirror,
            '|' => Self::VSplit,
            '-' => Self::HSplit,
            _ => unreachable!(),
        }
    }

    fn pass(self, dir: Dir) -> Passed {
        use Dir::*;
        match self {
            Self::Empty => Passed::Continue(dir),
            Self::RMirror => Passed::Continue(match dir {
                U => L,
                R => D,
                D => R,
                L => U,
            }),
            Self::LMirror => Passed::Continue(match dir {
                U => R,
                R => U,
                D => L,
                L => D,
            }),
            Self::VSplit => match dir {
                U | D => Passed::Continue(dir),
                R | L => Passed::Split(U, D),
            },
            Self::HSplit => match dir {
                U | D => Passed::Split(L, R),
                R | L => Passed::Continue(dir),
            },
        }
    }
}

type Layout = Vec<Vec<Tile>>;

fn solve(start: Pos, dir: Dir, layout: &Layout) -> usize {
    let mut v = vec![(start, dir)];
    let (x_max, y_max) = (layout[0].len() - 1, layout.len() - 1);
    let mut energized = HashMap::new();

    let push = |v: &mut Vec<(Pos, Dir)>, x: usize, y: usize, dir| {
        if let Some((x, y)) = match dir {
            Dir::U => y.checked_sub(1).map(|y| (x, y)),
            Dir::R => (x < x_max).then_some((x + 1, y)),
            Dir::D => (y < y_max).then_some((x, y + 1)),
            Dir::L => x.checked_sub(1).map(|x| (x, y)),
        } {
            v.push(((x, y), dir));
        }
    };

    while let Some(((x, y), dir)) = v.pop() {
        let energized: &mut Vec<_> = energized.entry((x, y)).or_default();
        if energized.contains(&dir) {
            continue;
        }
        energized.push(dir);

        match layout[y][x].pass(dir) {
            Passed::Continue(dir) => push(&mut v, x, y, dir),
            Passed::Split(dir1, dir2) => {
                push(&mut v, x, y, dir1);
                push(&mut v, x, y, dir2);
            }
        }
    }

    energized.len()
}

fn part1(lines: impl Iterator<Item = String>) {
    let layout: Vec<Vec<_>> = lines.map(|l| l.chars().map(Tile::new).collect()).collect();
    println!("{}", solve((0, 0), Dir::R, &layout));
}

fn part2(lines: impl Iterator<Item = String>) {
    let layout: Vec<Vec<_>> = lines.map(|l| l.chars().map(Tile::new).collect()).collect();
    let (x_max, y_max) = (layout[0].len() - 1, layout.len() - 1);

    let mut max = 0usize;
    for x in 0..=x_max {
        max = cmp::max(max, solve((x, 0), Dir::D, &layout));
        max = cmp::max(max, solve((x, y_max), Dir::U, &layout));
    }
    for y in 0..=y_max {
        max = cmp::max(max, solve((0, y), Dir::R, &layout));
        max = cmp::max(max, solve((x_max, y), Dir::L, &layout));
    }
    println!("{max}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
