use std::env;
use std::io::{self, BufRead};

type Shape = &'static [&'static [u8]];

#[rustfmt::skip]
const SHAPES: &[Shape] = &[
    &[
        b"####",
    ],
    &[
        b".#.",
        b"###",
        b".#.",
    ],
    &[
        b"..#",
        b"..#",
        b"###",
    ],
    &[
        b"#",
        b"#",
        b"#",
        b"#",
    ],
    &[
        b"##",
        b"##",
    ],
];

struct Board {
    rocks: Vec<[bool; 7]>,
    inuse: Vec<[bool; 7]>,
    base: usize,
}
impl Default for Board {
    fn default() -> Self {
        Self {
            rocks: vec![[false; 7]; 4],
            inuse: vec![[false; 7]; 4],
            base: 0,
        }
    }
}

impl Board {
    fn top_blanks(&self) -> usize {
        self.inuse
            .iter()
            .rev()
            .take_while(|xs| !xs.contains(&true))
            .count()
    }

    fn is_all_clear(&self) -> bool {
        self.inuse.iter().all(|l| l.iter().all(|b| !*b))
    }

    fn start(&mut self, shape: Shape) {
        let blanks = self.top_blanks();
        if blanks < 3 + shape.len() {
            for _ in 0..3 + shape.len() - blanks {
                self.inuse.push([false; 7]);
                self.rocks.push([false; 7]);
            }
        } else if blanks > 3 + shape.len() {
            for _ in 0..blanks - (3 + shape.len()) {
                self.inuse.pop();
                self.rocks.pop();
            }
        }

        for (y, xs) in shape.iter().enumerate() {
            for (x, &b) in xs.iter().enumerate() {
                if b != b'#' {
                    continue;
                }
                let y = self.rocks.len() - y - 1;
                self.rocks[y][x + 2] = true;
            }
        }
    }

    fn collision(&self, rocks: &[[bool; 7]]) -> bool {
        for y in 0..rocks.len() {
            for x in 0..rocks[y].len() {
                if rocks[y][x] && self.inuse[y][x] {
                    return true;
                }
            }
        }
        false
    }

    fn jet(&mut self, left: bool) {
        for line in self.rocks.iter() {
            if left && line.starts_with(&[true]) {
                return;
            }
            if !left && line.ends_with(&[true]) {
                return;
            }
        }

        let mut next = self.rocks.clone();
        for line in next.iter_mut() {
            if left {
                line.rotate_left(1);
            } else {
                line.rotate_right(1);
            }
        }

        if self.collision(&next) {
            return;
        }

        self.rocks = next;
    }

    fn fall(&mut self) -> bool {
        if self.rocks[0].contains(&true) {
            return false; // Reached floor
        }
        let mut next = self.rocks[1..].to_vec();
        next.push([false; 7]);
        if self.collision(&next) {
            return false;
        }
        self.rocks = next;
        true
    }

    fn settle(&mut self) {
        for y in 0..self.rocks.len() {
            for x in 0..self.rocks[y].len() {
                if self.rocks[y][x] {
                    self.inuse[y][x] = true;
                }
            }
        }
        for y in (0..self.inuse.len()).rev() {
            if self.inuse[y].iter().all(|b| *b) {
                self.rocks.drain(..=y);
                self.inuse.drain(..=y);
                self.base += y + 1;
                break;
            }
        }
        for line in self.rocks.iter_mut() {
            *line = [false; 7];
        }
    }

    fn simulate(&mut self, shape: Shape, mut input: impl Iterator<Item = u8>) {
        self.start(shape);
        loop {
            // self.print();
            match input.next().unwrap() {
                b'<' => self.jet(true),
                b'>' => self.jet(false),
                _ => unreachable!(),
            }
            if !self.fall() {
                self.settle();
                return;
            }
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for y in (0..self.rocks.len()).rev() {
            for x in 0..self.rocks[y].len() {
                let b = if self.rocks[y][x] {
                    '@'
                } else if self.inuse[y][x] {
                    '#'
                } else {
                    '.'
                };
                eprint!("{}", b);
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut b = Board::default();
    let mut shapes = SHAPES.iter().copied().cycle();
    let mut limit = 2022;
    let line = lines.next().unwrap();
    let mut input = line.as_bytes().iter().copied().cycle();
    while limit > 0 {
        b.simulate(shapes.next().unwrap(), &mut input);
        limit -= 1;
    }
    println!("{}", b.rocks.len() - b.top_blanks() + b.base);
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut b = Board::default();
    let mut shapes = SHAPES.iter().copied().cycle();
    let mut count = 0;
    let line = lines.next().unwrap();
    let mut input = line.as_bytes().iter().copied().cycle();

    // Detect cycle
    let mut prev_clear = 0;
    let mut prev_prev_clear = 0;
    let mut prev_base = 0;
    let mut prev_prev_base = 0;
    loop {
        b.simulate(shapes.next().unwrap(), &mut input);
        count += 1;
        if b.is_all_clear() {
            if count - prev_clear == prev_clear - prev_prev_clear
                && b.base - prev_base == prev_base - prev_prev_base
            {
                break;
            }
            prev_prev_clear = prev_clear;
            prev_clear = count;
            prev_prev_base = prev_base;
            prev_base = b.base;
        }
    }

    let count_delta = count - prev_clear;
    let base_delta = b.base - prev_base;
    let remain = 1000000000000usize - count;
    b.base += remain / count_delta * base_delta;

    let mut limit = remain % count_delta;
    while limit > 0 {
        b.simulate(shapes.next().unwrap(), &mut input);
        limit -= 1;
    }
    println!("{}", b.rocks.len() - b.top_blanks() + b.base);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
