use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dot {
    Light,
    Dark,
}

impl From<char> for Dot {
    fn from(c: char) -> Self {
        match c {
            '#' => Dot::Light,
            '.' => Dot::Dark,
            c => panic!("invalid char {:?}", c),
        }
    }
}

impl From<Dot> for char {
    fn from(c: Dot) -> char {
        match c {
            Dot::Light => '#',
            Dot::Dark => '.',
        }
    }
}

impl Dot {
    fn bit(self) -> usize {
        match self {
            Dot::Light => 1,
            Dot::Dark => 0,
        }
    }
}

struct Image {
    table: Box<[Dot]>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    surround: Dot,
    dots: HashMap<(i32, i32), Dot>,
}

impl Image {
    fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let table: Vec<_> = lines.next().unwrap().chars().map(Dot::from).collect();
        assert_eq!(table.len(), 512);

        lines.next().unwrap();

        let mut max_x: i32 = i32::MIN;
        let mut min_x: i32 = i32::MAX;
        let mut max_y: i32 = i32::MIN;
        let mut min_y: i32 = i32::MAX;
        let mut dots = HashMap::new();
        for (y, line) in lines.enumerate() {
            let y = y as i32;
            for (x, c) in line.chars().enumerate() {
                let x = x as i32;
                max_x = cmp::max(max_x, x);
                max_y = cmp::max(max_y, y);
                min_x = cmp::min(min_x, x);
                min_y = cmp::min(min_y, y);
                dots.insert((x, y), Dot::from(c));
            }
        }

        Self {
            table: table.into_boxed_slice(),
            min_x,
            min_y,
            max_x,
            max_y,
            dots,
            surround: Dot::Dark,
        }
    }

    fn enhance_at(&self, x: i32, y: i32) -> Dot {
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
        .map(|p| self.dots.get(&p).copied().unwrap_or(self.surround).bit())
        {
            idx = (idx << 1) + bit
        }
        self.table[idx]
    }

    fn enhance(&mut self) {
        let mut next = HashMap::new();
        for (x, y) in self.dots.keys() {
            let (x, y) = (*x, *y);
            next.insert((x, y), self.enhance_at(x, y));
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

        self.dots = next;

        self.surround = match self.surround {
            Dot::Dark => self.table[0],
            Dot::Light => self.table[0x1ff],
        };
    }

    fn count_lits(&self) -> usize {
        assert_eq!(self.surround, Dot::Dark);
        self.dots.iter().filter(|(_, c)| **c == Dot::Light).count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                print!("{}", char::from(*self.dots.get(&(x, y)).unwrap()));
            }
            println!();
        }
    }
}

fn main() {
    let mut image = Image::parse(io::stdin().lock().lines().map(Result::unwrap));
    for _ in 0..2 {
        image.enhance();
    }
    println!("Part 1: {:?}", image.count_lits());
    for _ in 0..48 {
        image.enhance();
    }
    println!("Part 2: {:?}", image.count_lits());
}
