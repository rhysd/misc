use std::env;
use std::io::{self, BufRead};
use std::iter;
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Eq)]
struct Region(Vec<Vec<bool>>); // bitvec is a better structure

impl Deref for Region {
    type Target = Vec<Vec<bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Region {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Region {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let v = lines
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        Self(v)
    }

    fn empty(w: usize, h: usize) -> Self {
        Self(vec![vec![false; w]; h])
    }

    fn area(&self) -> usize {
        self.len() * self[0].len()
    }

    fn rotate(&mut self) {
        debug_assert!(self.iter().all(|row| row.len() == self.len()));
        let n = self.len();
        for first in 0..n / 2 {
            let last = n - 1 - first;
            for i in first..last {
                let offset = last - (i - first);
                let top = self[first][i];
                self[first][i] = self[offset][first];
                self[offset][first] = self[last][offset];
                self[last][offset] = self[i][last];
                self[i][last] = top;
            }
        }
    }

    fn flip_horizontal(&mut self) {
        for row in self.iter_mut() {
            row.reverse();
        }
    }

    fn flip_vertical(&mut self) {
        let n = self.len();
        for i in 0..self[0].len() {
            for j in 0..n / 2 {
                let k = n - j - 1;
                let tmp = self[j][i];
                self[j][i] = self[k][i];
                self[k][i] = tmp;
            }
        }
    }

    fn fits_at(&self, x: usize, y: usize, region: &Region) -> bool {
        for (dy, xs) in region.iter().enumerate() {
            for (dx, b) in xs.iter().copied().enumerate() {
                if b && let Some(xs) = self.get(y + dy)
                    && let Some(&occupied) = xs.get(x + dx)
                    && occupied
                {
                    return false;
                }
            }
        }
        true
    }

    fn toggle_at(&mut self, x: usize, y: usize, region: &Region) {
        for (dy, xs) in region.iter().enumerate() {
            for (dx, b) in xs.iter().copied().enumerate() {
                if b {
                    self[y + dy][x + dx] = !self[y + dy][x + dx];
                }
            }
        }
    }
}

struct Present {
    shapes: Vec<Region>,
    area: usize,
    size: usize,
}

impl Present {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut s = Region::parse(lines);
        let area = s.len() * s.len();
        let size = s.iter().map(|l| l.iter().filter(|&&b| b).count()).sum();

        let mut shapes = vec![];
        let mut push = |s: &Region| {
            if !shapes.contains(s) {
                shapes.push(s.clone());
            }
        };
        for _ in 0..4 {
            push(&s);
            s.rotate();
        }
        s.flip_horizontal();
        for _ in 0..4 {
            push(&s);
            s.rotate();
        }
        s.flip_vertical();
        for _ in 0..4 {
            push(&s);
            s.rotate();
        }

        Self { shapes, area, size }
    }
}

fn fits_dfs(presents: &[&Present], region: &mut Region, empties: usize) -> bool {
    let Some((&head, tail)) = presents.split_first() else {
        return true;
    };
    if empties < head.size {
        return false;
    }

    for shape in &head.shapes {
        for y in 0..region.len() - shape.len() + 1 {
            for x in 0..region[0].len() - shape.len() + 1 {
                if region.fits_at(x, y, shape) {
                    region.toggle_at(x, y, shape);
                    if fits_dfs(tail, region, empties - head.size) {
                        return true;
                    }
                    region.toggle_at(x, y, shape); // Restore the state
                }
            }
        }
    }

    false
}

fn fits(line: &str, presents: &[Present]) -> bool {
    let mut s = line.split(": ");
    let (w, h) = s.next().unwrap().split_once('x').unwrap();
    let (w, h) = (w.parse().unwrap(), h.parse().unwrap());
    let mut region = Region::empty(w, h);
    let counts = s.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
    let presents = counts.zip(presents.iter()).fold(vec![], |mut acc, (count, present)| {
        acc.extend(iter::repeat_n(present, count));
        acc
    });

    let area = region.area();
    if presents.iter().map(|p| p.area).sum::<usize>() <= area {
        return true;
    }
    if presents.iter().map(|p| p.size).sum::<usize>() > area {
        return false;
    }
    fits_dfs(&presents, &mut region, area)
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut presents = vec![];
    let first_line = loop {
        let line = lines.next().unwrap();
        if !line.ends_with(':') {
            break line;
        }
        presents.push(Present::parse(lines.by_ref()));
    };

    let count =
        fits(&first_line, &presents) as usize + lines.filter(|l| fits(l, &presents)).count();

    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
