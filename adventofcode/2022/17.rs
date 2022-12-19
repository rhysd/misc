use std::env;
use std::io::{self, BufRead};

type Shape = &'static [(usize, usize)];

const SHAPES: &[Shape] = &[
    // ####
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    // .#.
    // ###
    // .#.
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    // ..#
    // ..#
    // ###
    &[(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
    // #
    // #
    // #
    // #
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    // ##
    // ##
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
];

#[derive(Clone, Copy)]
enum Dir {
    L,
    R,
    D,
}

#[derive(Default)]
struct Board {
    rocks: Vec<(usize, usize)>,
    inuse: Vec<u8>,
}

impl Board {
    const WIDTH: usize = 7;

    fn top_blanks(&self) -> usize {
        self.inuse
            .iter()
            .copied()
            .rev()
            .take_while(|&l| l == 0)
            .count()
    }

    fn height(&self) -> usize {
        self.inuse.len() - self.top_blanks()
    }

    fn inuse(&self, x: usize, y: usize) -> bool {
        self.inuse[y] >> (Self::WIDTH - x - 1) & 1 != 0
    }

    fn start(&mut self, shape: Shape) {
        let blanks = self.top_blanks();
        let init_height = shape.iter().map(|(_, y)| y).max().unwrap() + 1 + 3;
        if blanks < init_height {
            for _ in 0..init_height - blanks {
                self.inuse.push(0);
            }
        } else if blanks > init_height {
            for _ in 0..blanks - init_height {
                self.inuse.pop();
            }
        }

        let len = self.inuse.len();
        self.rocks
            .extend(shape.iter().map(|(x, y)| (x + 2, len - y - 1)));
    }

    fn collision(&self, x: usize, y: usize, dir: Dir) -> bool {
        match dir {
            Dir::L if x == 0 => true,
            Dir::R if x == Self::WIDTH - 1 => true,
            Dir::D if y == 0 => true,
            Dir::L => self.inuse(x - 1, y),
            Dir::R => self.inuse(x + 1, y),
            Dir::D => self.inuse(x, y - 1),
        }
    }

    fn jet(&mut self, dir: Dir) -> bool {
        if self
            .rocks
            .iter()
            .copied()
            .any(|(x, y)| self.collision(x, y, dir))
        {
            return false;
        }
        for (x, y) in self.rocks.iter_mut() {
            match dir {
                Dir::L => *x -= 1,
                Dir::R => *x += 1,
                Dir::D => *y -= 1,
            }
        }
        true
    }

    fn settle(&mut self) {
        for (x, y) in self.rocks.iter().copied() {
            self.inuse[y] |= 1 << (Self::WIDTH - x - 1);
        }
        self.rocks.clear();
    }

    fn simulate(&mut self, shape: Shape, mut input: impl Iterator<Item = u8>) {
        self.start(shape);
        loop {
            match input.next().unwrap() {
                b'<' => self.jet(Dir::L),
                b'>' => self.jet(Dir::R),
                _ => unreachable!(),
            };
            if !self.jet(Dir::D) {
                self.settle();
                return;
            }
        }
    }

    fn detect_cycle(&self) -> Option<(usize, usize, usize)> {
        let blanks = self.top_blanks();
        let len = self.inuse.len() - blanks;
        if len < 2 {
            return None;
        }

        let mut window = 5; // {min height of any shape} + 1
        while window * 2 < len {
            let left_start = len - window;
            let left = &self.inuse[left_start..len];
            let right_start = len - window * 2;
            let right = &self.inuse[right_start..left_start];
            if left == right {
                return Some((right_start, left_start, len));
            }
            window += 1;
        }

        None
    }

    #[allow(unused)]
    fn print(&self) {
        for (y, line) in self.inuse.iter().enumerate().rev() {
            for x in 0..Self::WIDTH {
                let c = if self.rocks.contains(&(x, y)) {
                    '@'
                } else if self.inuse(x, y) {
                    '#'
                } else {
                    '.'
                };
                eprint!("{}", c);
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut b = Board::default();
    let mut shapes = SHAPES.iter().copied().cycle();
    let line = lines.next().unwrap();
    let mut input = line.as_bytes().iter().copied().cycle();
    for _ in 0..2022 {
        b.simulate(shapes.next().unwrap(), &mut input);
    }
    println!("{}", b.height());
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut b = Board::default();
    let mut shapes = SHAPES.iter().copied().cycle();
    let line = lines.next().unwrap();
    let mut input = line.as_bytes().iter().copied().cycle();

    let mut age = 0;
    let mut history = vec![];
    let (start_height, end_height, current_height) = loop {
        b.simulate(shapes.next().unwrap(), &mut input);
        age += 1;
        history.push((age, b.height()));
        if let Some(detected) = b.detect_cycle() {
            break detected;
        }
    };
    let height_delta = end_height - start_height;
    let find_age = move |age| {
        history
            .iter()
            .find_map(|(c, h)| (*h == age).then_some(*c))
            .unwrap()
    };
    let start_age = find_age(start_height);
    let end_age = find_age(end_height);
    let age_delta = end_age - start_age;

    let remain = 1000000000000usize - start_age;

    for _ in 0..remain % age_delta {
        b.simulate(shapes.next().unwrap(), &mut input);
    }
    let height_before_cycles = start_height - 1; // start_height is the height at the first cycle start
    let height_after_cycles = b.height() - current_height;
    let height = height_before_cycles + remain / age_delta * height_delta + height_after_cycles;

    println!("{}", height);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
