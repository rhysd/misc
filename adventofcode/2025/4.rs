use std::env;
use std::io::{self, BufRead};

struct Cells(Vec<Vec<u8>>);

impl Cells {
    fn new(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(|l| l.into_bytes()).collect())
    }

    fn adjucents(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + use<> {
        let (px, nx) = (x.checked_sub(1), (x + 1 < self.0[y].len()).then_some(x + 1));
        let (py, ny) = (y.checked_sub(1), (y + 1 < self.0.len()).then_some(y + 1));
        let (x, y) = (Some(x), Some(y));
        [(px, py), (x, py), (nx, py), (px, y), (nx, y), (px, ny), (x, ny), (nx, ny)]
            .into_iter()
            .filter_map(|(x, y)| Some((x?, y?)))
    }

    fn can_remove(&self, x: usize, y: usize) -> bool {
        self.0[y][x] != b'.'
            && self.adjucents(x, y).filter(|&(x, y)| self.0[y][x] == b'@').count() < 4
    }

    fn part1(&self) -> u32 {
        let mut count = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.can_remove(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn part2(&mut self) -> u32 {
        let mut removed = 0;

        let mut stack = vec![];
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.can_remove(x, y) {
                    self.0[y][x] = b'.';
                    removed += 1;
                    stack.push((x, y));
                }
            }
        }

        while let Some((x, y)) = stack.pop() {
            for (x, y) in self.adjucents(x, y) {
                if self.can_remove(x, y) {
                    self.0[y][x] = b'.';
                    removed += 1;
                    stack.push((x, y));
                }
            }
        }
        removed
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => println!("{}", Cells::new(lines).part1()),
        Some("2") => println!("{}", Cells::new(lines).part2()),
        x => panic!("invalid argument: {x:?}"),
    }
}
