use std::env;
use std::io::{self, BufRead};

struct Board(Vec<Vec<char>>);

impl Board {
    fn at(&self, x: i32, y: i32) -> Option<char> {
        self.0.get(usize::try_from(y).ok()?)?.get(usize::try_from(x).ok()?).copied()
    }

    fn size(&self) -> (i32, i32) {
        (self.0[0].len() as _, self.0.len() as _)
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let board = Board(lines.map(|l| l.chars().collect()).collect());
    let (xlen, ylen) = board.size();

    let is_mas = |mut x, mut y, dx, dy| {
        for c in "MAS".chars() {
            x += dx;
            y += dy;
            if board.at(x, y) != Some(c) {
                return false;
            }
        }
        true
    };

    let mut count = 0;
    for y in 0..ylen {
        for x in 0..xlen {
            if board.at(x, y) == Some('X') {
                for dy in -1..=1 {
                    for dx in -1..1 {
                        if (dx, dy) != (0, 0) && is_mas(x, y, dx, dy) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{count}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let board = Board(lines.map(|l| l.chars().collect()).collect());
    let (xlen, ylen) = board.size();

    let cross_at = |x, y| {
        Some([
            board.at(x - 1, y - 1)?,
            board.at(x + 1, y - 1)?,
            board.at(x + 1, y + 1)?,
            board.at(x - 1, y + 1)?,
        ])
    };

    let mut count = 0;
    for y in 0..ylen {
        for x in 0..xlen {
            if board.at(x, y) == Some('A') {
                if let Some(
                    ['M', 'M', 'S', 'S']
                    | ['S', 'M', 'M', 'S']
                    | ['S', 'S', 'M', 'M']
                    | ['M', 'S', 'S', 'M'],
                ) = cross_at(x, y)
                {
                    count += 1;
                }
            }
        }
    }

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
