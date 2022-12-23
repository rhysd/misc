use fxhash::{FxHashMap, FxHashSet};
use std::env;
use std::io::{self, BufRead};
use std::mem;

type Point = (i32, i32);

struct Grove {
    elves: FxHashSet<Point>,
    propose: [fn(&[bool; 8]) -> Option<Point>; 4],
}

impl Grove {
    fn new(lines: impl Iterator<Item = String>) -> Self {
        let mut elves = FxHashSet::default();
        for (y, line) in lines.enumerate() {
            for (x, b) in line.into_bytes().into_iter().enumerate() {
                if b != b'#' {
                    continue;
                }
                elves.insert((x as i32, y as i32));
            }
        }
        Self {
            elves,
            propose: [
                |adj| (!adj[0] && !adj[1] && !adj[2]).then_some((0, -1)), // N
                |adj| (!adj[4] && !adj[5] && !adj[6]).then_some((0, 1)),  // S
                |adj| (!adj[0] && !adj[6] && !adj[7]).then_some((-1, 0)), // W
                |adj| (!adj[2] && !adj[3] && !adj[4]).then_some((1, 0)),  // E
            ],
        }
    }

    fn round(&mut self) -> bool {
        let elves = mem::take(&mut self.elves);
        let mut proposals = FxHashMap::default();
        for (x, y) in elves.iter().copied() {
            let adj = [
                (x - 1, y - 1), // NW
                (x, y - 1),     // N
                (x + 1, y - 1), // NE
                (x + 1, y),     // E
                (x + 1, y + 1), // SE
                (x, y + 1),     // S
                (x - 1, y + 1), // SW
                (x - 1, y),     // W
            ]
            .map(|p| elves.contains(&p));

            if adj.iter().all(|b| !b) {
                self.elves.insert((x, y)); // Do not move
                continue;
            }

            let Some((dx, dy)) = self.propose.into_iter().find_map(|f| f(&adj)) else {
                self.elves.insert((x, y)); // Do not move
                continue;
            };
            let want = (x + dx, y + dy);

            match proposals.get(&want) {
                None => {
                    proposals.insert(want, Some((x, y)));
                }
                Some(Some(elf)) => {
                    self.elves.insert((x, y));
                    self.elves.insert(*elf);
                    proposals.insert(want, None);
                }
                Some(None) => {
                    self.elves.insert((x, y));
                }
            }
        }
        for (to, _) in proposals.into_iter().filter(|(_, e)| e.is_some()) {
            self.elves.insert(to);
        }
        self.propose.rotate_left(1);
        elves != self.elves
    }

    fn minmax(&self) -> (i32, i32, i32, i32) {
        let (mut xmax, mut xmin) = (i32::MIN, i32::MAX);
        let (mut ymax, mut ymin) = (i32::MIN, i32::MAX);
        for (x, y) in self.elves.iter().copied() {
            xmax = xmax.max(x);
            xmin = xmin.min(x);
            ymax = ymax.max(y);
            ymin = ymin.min(y);
        }
        (xmin, xmax, ymin, ymax)
    }

    fn count_empty(&self) -> usize {
        let (xmin, xmax, ymin, ymax) = self.minmax();
        (ymax - ymin + 1) as usize * (xmax - xmin + 1) as usize - self.elves.len()
    }

    #[allow(unused)]
    fn print(&self) {
        let (xmin, xmax, ymin, ymax) = self.minmax();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let c = if self.elves.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut grove = Grove::new(lines);
    for _ in 0..10 {
        grove.round();
    }
    println!("{}", grove.count_empty());
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut grove = Grove::new(lines);
    for i in 1.. {
        if !grove.round() {
            println!("{}", i);
            return;
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
