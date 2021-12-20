use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum C {
    L, // light
    D, // dark
}

impl C {
    fn new(c: char) -> C {
        match c {
            '#' => C::L,
            '.' => C::D,
            c => panic!("invalid char {:?}", c),
        }
    }

    fn bit(self) -> usize {
        match self {
            C::L => 1,
            C::D => 0,
        }
    }
}

struct Image {
    enhancements: Box<[C]>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    surround: C,
    current: HashMap<(i32, i32), C>,
}

impl Image {
    fn new(mut lines: impl Iterator<Item = String>) -> Self {
        let enhancements: Vec<_> = lines.next().unwrap().chars().map(C::new).collect();
        assert_eq!(enhancements.len(), 512);

        lines.next().unwrap();

        let mut max_x: i32 = i32::MIN;
        let mut min_x: i32 = i32::MAX;
        let mut max_y: i32 = i32::MIN;
        let mut min_y: i32 = i32::MAX;
        let mut current = HashMap::new();
        for (y, line) in lines.enumerate() {
            let y = y as i32;
            for (x, c) in line.chars().enumerate() {
                let x = x as i32;
                max_x = cmp::max(max_x, x);
                max_y = cmp::max(max_y, y);
                min_x = cmp::min(min_x, x);
                min_y = cmp::min(min_y, y);
                current.insert((x, y), C::new(c));
            }
        }

        Self {
            enhancements: enhancements.into_boxed_slice(),
            min_x,
            min_y,
            max_x,
            max_y,
            current,
            surround: C::D,
        }
    }

    fn enhance_at(&self, x: i32, y: i32) -> C {
        let mut idx = 0usize;
        for bit in [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .map(|p| self.current.get(&p).copied().unwrap_or(self.surround).bit())
        {
            idx = (idx << 1) + bit
        }
        self.enhancements[idx]
    }

    fn enhance(&mut self) {
        let mut next = HashMap::new();
        for (x, y) in self.current.keys() {
            let (x, y) = (*x, *y);
            let c = self.enhance_at(x, y);
            next.insert((x, y), c);
        }

        self.min_x -= 1;
        self.min_y -= 1;
        self.max_x += 1;
        self.max_y += 1;
        for x in self.min_x..=self.max_x {
            for y in [self.min_y, self.max_y] {
                next.insert((x, y), self.enhance_at(x, y));
            }
        }
        for y in self.min_y + 1..=self.max_y - 1 {
            for x in [self.min_x, self.max_x] {
                next.insert((x, y), self.enhance_at(x, y));
            }
        }

        self.current = next;

        self.surround = match self.surround {
            C::D => self.enhancements[0],
            C::L => self.enhancements[0x1ff],
        };
    }

    fn count_lits(&self) -> usize {
        assert_eq!(self.surround, C::D);
        self.current.iter().filter(|(_, c)| **c == C::L).count()
    }

    fn print(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let c = match *self.current.get(&(x, y)).unwrap() {
                    C::L => '#',
                    C::D => '.',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn main() {
    let mut image = Image::new(io::stdin().lock().lines().map(Result::unwrap));
    for _ in 0..2 {
        image.enhance();
    }
    println!("Part 1: {:?}", image.count_lits());
    for _ in 0..48 {
        image.enhance();
    }
    println!("Part 2: {:?}", image.count_lits());
}
