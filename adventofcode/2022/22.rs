use std::env;
use std::io::{self, BufRead};

type Pos = (i32, i32);
type Edge = (Pos, Pos);

#[rustfmt::skip]
const PART2_TRANSITIONS: &[(Edge, Edge, Dir, Dir)] = &[
    (((50, 0),  (99, 0)),  ((0, 150), (0, 199)), Dir::U, Dir::R),
    (((0, 150), (0, 199)), ((50, 0),  (99, 0)),  Dir::L, Dir::D),

    (((100, 0), (149, 0)),  ((0, 199), (49, 199)), Dir::U, Dir::U),
    (((0, 199), (49, 199)), ((100, 0), (149, 0)),  Dir::D, Dir::D),

    (((149, 0),  (149, 49)), ((99, 149), (99, 100)), Dir::R, Dir::L),
    (((99, 149), (99, 100)), ((149, 0),  (149, 49)), Dir::R, Dir::L),

    (((100, 49), (149, 49)), ((99, 50),  (99, 99)),  Dir::D, Dir::L),
    (((99, 50),  (99, 99)),  ((100, 49), (149, 49)), Dir::R, Dir::U),

    (((50, 149), (99, 149)), ((49, 150), (49, 199)), Dir::D, Dir::L),
    (((49, 150), (49, 199)), ((50, 149), (99, 149)), Dir::R, Dir::U),

    (((50, 0),  (50, 49)), ((0, 149), (0, 100)), Dir::L, Dir::R),
    (((0, 149), (0, 100)), ((50, 0),  (50, 49)), Dir::L, Dir::R),

    (((50, 50), (50, 99)),  ((0, 100), (49, 100)), Dir::L, Dir::D),
    (((0, 100), (49, 100)), ((50, 50), (50, 99)),  Dir::U, Dir::R),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Dir {
    U = 3,
    R = 0,
    D = 1,
    L = 2,
}

impl Dir {
    fn turn_right(self) -> Self {
        use Dir::*;
        match self {
            U => R,
            R => D,
            D => L,
            L => U,
        }
    }
    fn turn_left(self) -> Self {
        use Dir::*;
        match self {
            U => L,
            R => U,
            D => R,
            L => D,
        }
    }
    fn to_char(self) -> char {
        match self {
            Dir::U => '^',
            Dir::R => '>',
            Dir::D => 'v',
            Dir::L => '<',
        }
    }
    fn unit(self) -> Pos {
        match self {
            Dir::U => (0, -1),
            Dir::R => (1, 0),
            Dir::D => (0, 1),
            Dir::L => (-1, 0),
        }
    }
    fn add_to(self, (x, y): Pos) -> Pos {
        let (dx, dy) = self.unit();
        (x + dx, y + dy)
    }
}

enum Inst {
    Move(usize),
    Right,
    Left,
}

struct Input(Vec<u8>);

impl Input {
    fn new(mut b: Vec<u8>) -> Self {
        b.reverse();
        Self(b)
    }
}

impl Iterator for Input {
    type Item = Inst;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(b) = self.0.pop() else {
            return None;
        };
        let i = match b {
            b'R' => Inst::Right,
            b'L' => Inst::Left,
            b if b.is_ascii_digit() => {
                let mut i = (b - b'0') as usize;
                while let Some(b) = self.0.last().copied() {
                    // dbg!(std::str::from_utf8(&self.0).unwrap());
                    if !b.is_ascii_digit() {
                        break;
                    }
                    i = i * 10 + (b - b'0') as usize;
                    self.0.pop();
                }
                Inst::Move(i)
            }
            b => panic!("invalid byte {} ({})", b, b as char),
        };
        Some(i)
    }
}

struct Board {
    map: Vec<Vec<u8>>,
    dir: Dir,
    pos: Pos,
    edge_len: i32,
}

impl Board {
    fn new(map: Vec<Vec<u8>>, edge_len: i32) -> Self {
        let pos = map
            .iter()
            .enumerate()
            .find_map(|(y, xs)| {
                xs.iter()
                    .enumerate()
                    .find_map(|(x, &b)| (b == b'.').then_some((x as i32, y as i32)))
            })
            .unwrap();
        Self {
            map,
            dir: Dir::R,
            pos,
            edge_len,
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for (y, xs) in self.map.iter().enumerate() {
            for (x, b) in xs.iter().enumerate() {
                let p = (x as i32, y as i32);
                let c = if p == self.pos {
                    self.dir.to_char()
                } else {
                    *b as char
                };
                eprint!("{}", c);
            }
            eprintln!();
        }
        eprintln!();
    }

    fn step_2d(&mut self) -> bool {
        let (mut x, mut y) = self.pos;
        let h = self.map.len() as i32;
        let w = self.map[0].len() as i32;
        loop {
            (x, y) = match self.dir {
                Dir::U => (x, y - 1),
                Dir::R => (x + 1, y),
                Dir::D => (x, y + 1),
                Dir::L => (x - 1, y),
            };
            if x < 0 {
                x = w - 1;
            } else if x >= w {
                x = 0;
            }
            if y < 0 {
                y = h - 1;
            } else if y >= h {
                y = 0;
            }
            match self
                .map
                .get(y as usize)
                .and_then(|xs| xs.get(x as usize))
                .copied()
                .unwrap_or(b' ')
            {
                b' ' => continue,
                b'#' => return false,
                b'.' => {
                    self.pos = (x, y);
                    return true;
                }
                b => panic!("invalid byte {} ({})", b, b as char),
            }
        }
    }

    fn inst_2d(&mut self, i: Inst) {
        match i {
            Inst::Right => self.dir = self.dir.turn_right(),
            Inst::Left => self.dir = self.dir.turn_left(),
            Inst::Move(i) => {
                for _ in 0..i {
                    if !self.step_2d() {
                        break;
                    }
                }
            }
        }
    }

    fn password(&self) -> usize {
        let x = self.pos.0 as usize + 1;
        let y = self.pos.1 as usize + 1;
        let d = self.dir as usize;
        y * 1000 + 4 * x + d
    }

    fn inst_3d(&mut self, i: Inst) {
        match i {
            Inst::Right => self.dir = self.dir.turn_right(),
            Inst::Left => self.dir = self.dir.turn_left(),
            Inst::Move(i) => {
                for _ in 0..i {
                    if !self.step_3d() {
                        break;
                    }
                }
            }
        }
    }

    fn at(&self, (x, y): Pos) -> u8 {
        if y < 0 || y >= self.map.len() as i32 {
            return b' ';
        }
        if x < 0 || x >= self.map[y as usize].len() as i32 {
            return b' ';
        }
        self.map[y as usize][x as usize]
    }

    fn edge(&self) -> Edge {
        match self.dir {
            Dir::L | Dir::R => {
                let (x, y) = self.pos;
                let y = y / self.edge_len * self.edge_len;
                ((x, y), (x, y + self.edge_len - 1))
            }
            Dir::U | Dir::D => {
                let (x, y) = self.pos;
                let x = x / self.edge_len * self.edge_len;
                ((x, y), (x + self.edge_len - 1, y))
            }
        }
    }

    fn wrapped(&self) -> (Pos, Dir) {
        let edge = self.edge();
        for (from, to, from_dir, to_dir) in PART2_TRANSITIONS.iter().copied() {
            if self.dir != from_dir {
                continue;
            }

            let (ex, ey) = if from == edge {
                edge.0
            } else if from == (edge.1, edge.0) {
                edge.1
            } else {
                continue;
            };

            let (x, y) = self.pos;
            let delta = if x == ex {
                (y - ey).abs()
            } else if y == ey {
                (x - ex).abs()
            } else {
                unreachable!()
            };

            let (tx, ty) = to.0;
            let pos = if tx == to.1 .0 {
                let delta = if ty < to.1 .1 { delta } else { -delta };
                (tx, ty + delta)
            } else if ty == to.1 .1 {
                let delta = if tx < to.1 .0 { delta } else { -delta };
                (tx + delta, ty)
            } else {
                panic!("invalid edge {:?}", to);
            };

            return (pos, to_dir);
        }
        panic!("{:?} is not an edge: {:?} {:?}", edge, self.pos, self.dir);
    }

    fn step_3d(&mut self) -> bool {
        let (x, y) = self.dir.add_to(self.pos);
        let b = self.at((x, y));

        match b {
            b' ' => { /* go through */ }
            b'#' => return false,
            b'.' => {
                self.pos = (x, y);
                return true;
            }
            b => panic!("invalid byte {} ({})", b, b as char),
        }

        let (pos, dir) = self.wrapped();
        match self.at(pos) {
            b'#' => false,
            b'.' => {
                self.pos = pos;
                self.dir = dir;
                true
            }
            b => panic!("invalid byte {} ({}) at {:?}", b, b as char, pos),
        }
    }
}

fn parse(mut lines: impl Iterator<Item = String>) -> (Board, Input) {
    let mut b = vec![];
    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }
        b.push(l.into());
    }
    let is_test = b.len() < 50;
    let edge_len = if is_test { 4 } else { 50 };
    (
        Board::new(b, edge_len),
        Input::new(lines.next().unwrap().into()),
    )
}

fn part1(lines: impl Iterator<Item = String>) {
    let (mut board, input) = parse(lines);
    for inst in input {
        board.inst_2d(inst);
    }
    println!("{}", board.password());
}

fn part2(lines: impl Iterator<Item = String>) {
    let (mut board, input) = parse(lines);
    for inst in input {
        board.inst_3d(inst);
    }
    println!("{}", board.password());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
