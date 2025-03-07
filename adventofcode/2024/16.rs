use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};
use std::rc::Rc;

type Pos = (usize, usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn advance(self, (x, y): Pos) -> Pos {
        match self {
            Self::U => (x, y - 1),
            Self::R => (x + 1, y),
            Self::D => (x, y + 1),
            Self::L => (x - 1, y),
        }
    }

    fn right(self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    fn left(self) -> Self {
        match self {
            Self::U => Self::L,
            Self::R => Self::U,
            Self::D => Self::R,
            Self::L => Self::D,
        }
    }
}

fn parse_maze(lines: impl Iterator<Item = String>) -> (Pos, Pos, Vec<Vec<bool>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = lines
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        true
                    }
                    'E' => {
                        end = (x, y);
                        true
                    }
                    '.' => true,
                    '#' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (start, end, map)
}

fn part1(lines: impl Iterator<Item = String>) {
    struct State {
        score: u32,
        pos: Pos,
        dir: Dir,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.score.cmp(&self.score)
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

    let (start, end, maze) = parse_maze(lines);

    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::from([State { score: 0, pos: start, dir: Dir::R }]);

    while let Some(State { score, pos, dir }) = queue.pop() {
        if pos == end {
            println!("{score}");
            return;
        }

        let (x, y) = pos;
        if dist.get(&(x, y, dir)).map(|&s| score > s).unwrap_or(false) {
            continue;
        }

        for (dir, (x, y), score) in [
            (dir, dir.advance((x, y)), score + 1),
            (dir.left(), (x, y), score + 1000),
            (dir.right(), (x, y), score + 1000),
        ] {
            if maze[y][x] && dist.get(&(x, y, dir)).map(|&s| s > score).unwrap_or(true) {
                queue.push(State { score, pos: (x, y), dir });
                dist.insert((x, y, dir), score);
            }
        }
    }

    unreachable!("no solution");
}

fn part2(lines: impl Iterator<Item = String>) {
    struct PathNode(Pos, Option<Rc<PathNode>>);

    struct State {
        score: u32,
        pos: Pos,
        dir: Dir,
        path: Rc<PathNode>,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.score.cmp(&self.score)
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

    let (start, end, maze) = parse_maze(lines);

    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::from([State {
        score: 0,
        pos: start,
        dir: Dir::R,
        path: Rc::new(PathNode(start, None)),
    }]);
    let mut best_score = u32::MAX;
    let mut all_paths = HashSet::new();

    while let Some(State { score, pos, dir, path }) = queue.pop() {
        if pos == end {
            if best_score < score {
                break;
            }
            best_score = score;
            let mut cur = &path;
            loop {
                let PathNode(pos, next) = &**cur;
                all_paths.insert(*pos);
                if let Some(next) = next {
                    cur = next;
                } else {
                    break;
                }
            }
            continue;
        }

        let (x, y) = pos;
        if dist.get(&(x, y, dir)).map(|&s| score > s).unwrap_or(false) {
            continue;
        }

        for (dir, (x, y), score) in [
            (dir, dir.advance((x, y)), score + 1),
            (dir.left(), (x, y), score + 1000),
            (dir.right(), (x, y), score + 1000),
        ] {
            // Use `s >= score` instead of `s > score` so that we can find all the shortest paths.
            if maze[y][x] && dist.get(&(x, y, dir)).map(|&s| s >= score).unwrap_or(true) {
                let mut path = path.clone();
                if (x, y) != path.0 {
                    path = Rc::new(PathNode((x, y), Some(path)));
                }
                queue.push(State { score, pos: (x, y), dir, path });
                dist.insert((x, y, dir), score);
            }
        }
    }

    assert_ne!(best_score, u32::MAX);
    println!("{}", all_paths.len());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
