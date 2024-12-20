use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::io::{self, BufRead};
use std::ops::Deref;
use std::rc::Rc;

type Pos = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Track,
    Wall,
}

#[derive(Default)]
struct PathNode(Pos, Option<Rc<PathNode>>);
type Path = Rc<PathNode>;

impl PathNode {
    fn to_vec(&self) -> Vec<Pos> {
        let mut v = vec![self.0];
        let mut cur = self.1.as_ref();
        while let Some(PathNode(pos, next)) = cur.map(|o| o.deref()) {
            v.push(*pos);
            cur = next.as_ref();
        }
        v.reverse();
        v
    }
}

struct Maze {
    cells: Vec<Vec<Cell>>,
    start: Pos,
    end: Pos,
    width: usize,
    height: usize,
    cache: HashMap<Pos, usize>,
}

impl Maze {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let cells: Vec<Vec<_>> = lines
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            Cell::Track
                        }
                        'E' => {
                            end = (x, y);
                            Cell::Track
                        }
                        '.' => Cell::Track,
                        '#' => Cell::Wall,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let width = cells[0].len();
        let height = cells.len();
        Self { cells, start, end, width, height, cache: HashMap::new() }
    }

    fn memoize(&mut self, mut steps: usize, mut cur: &Path) {
        loop {
            let PathNode(pos, next) = &**cur;
            self.cache.insert(*pos, steps);
            if let Some(next) = next {
                cur = next;
                steps += 1;
            } else {
                return;
            }
        }
    }

    fn path(&mut self, start: Pos) -> Option<(usize, Option<Path>)> {
        if let Some(&ret) = self.cache.get(&start) {
            return Some((ret, None));
        }

        struct State {
            pos: Pos,
            steps: usize,
            path: Path,
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.steps.cmp(&self.steps)
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        impl Eq for State {}

        let mut queue = BinaryHeap::from([State {
            pos: start,
            steps: 0,
            path: Rc::new(PathNode(start, None)),
        }]);
        let mut dist = HashMap::from([(start, 0)]);

        while let Some(State { pos, steps, path }) = queue.pop() {
            if let Some(&steps) = self.cache.get(&pos) {
                self.memoize(steps, &path);
                return Some((steps, None));
            }

            if pos == self.end {
                self.memoize(0, &path);
                return Some((steps, Some(path)));
            }
            if let Some(s) = dist.get(&pos) {
                if *s < steps {
                    continue;
                }
            }
            let (x, y) = pos;
            let steps = steps + 1;
            for pos in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
                let (x, y) = pos;
                if self.cells[y][x] == Cell::Track
                    && dist.get(&pos).map(|&s| steps < s).unwrap_or(true)
                {
                    let path = Rc::new(PathNode(pos, Some(path.clone())));
                    dist.insert(pos, steps);
                    queue.push(State { pos, steps, path });
                }
            }
        }

        None
    }

    fn solve(&mut self, max: usize, threshold: usize) -> usize {
        let (base, path) = self.path(self.start).unwrap();
        let path = path.unwrap().to_vec();

        let mut count = 0;
        for (before, (sx, sy)) in path.into_iter().enumerate() {
            for x in sx.saturating_sub(max)..(sx + max + 1).min(self.width) {
                for y in sy.saturating_sub(max)..(sy + max + 1).min(self.height) {
                    let cheat = x.abs_diff(sx) + y.abs_diff(sy);
                    if 2 <= cheat && cheat <= max && self.cells[y][x] == Cell::Track {
                        if let Some((after, _)) = self.path((x, y)) {
                            let steps = before + cheat + after;
                            if base.saturating_sub(steps) >= threshold {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut maze = Maze::parse(lines);
    println!("{}", maze.solve(2, 100));
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut maze = Maze::parse(lines);
    println!("{}", maze.solve(20, 100));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
