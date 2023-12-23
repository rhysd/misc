use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    R,
    D,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Path,
    Forest,
    Slope(Dir),
}

impl Cell {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Dir::U),
            '>' => Self::Slope(Dir::R),
            'v' => Self::Slope(Dir::D),
            '<' => Self::Slope(Dir::L),
            _ => unreachable!(),
        }
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let maze: Vec<Vec<_>> = lines.map(|l| l.chars().map(Cell::new).collect()).collect();
    let (x_len, y_len) = (maze[0].len(), maze.len());
    let start_x = maze[0].iter().position(|&c| c == Cell::Path).unwrap();
    let start = (start_x, 0);
    let goal_x = maze[y_len - 1]
        .iter()
        .position(|&c| c == Cell::Path)
        .unwrap();
    let goal = (goal_x, y_len - 1);

    let mut stack = vec![(0, start, HashSet::new())];

    let mut max = 0;
    while let Some((cost, pos, mut path)) = stack.pop() {
        if pos == goal {
            max = max.max(cost);
            continue;
        }

        let (x, y) = pos;
        let cost = cost + 1;
        match maze[y][x] {
            Cell::Path => {
                let adj = [
                    y.checked_sub(1).map(|y| (x, y)),
                    (x + 1 < x_len).then_some((x + 1, y)),
                    (y + 1 < y_len).then_some((x, y + 1)),
                    x.checked_sub(1).map(|x| (x, y)),
                ]
                .into_iter()
                .flatten()
                .filter(|&(x, y)| maze[y][x] != Cell::Forest);
                for pos in adj {
                    if path.contains(&pos) {
                        continue;
                    }
                    let mut path = path.clone();
                    path.insert(pos);
                    stack.push((cost, pos, path));
                }
            }
            Cell::Slope(dir) => {
                let pos = match dir {
                    Dir::U => (x, y - 1),
                    Dir::R => (x + 1, y),
                    Dir::D => (x, y + 1),
                    Dir::L => (x - 1, y),
                };
                if path.contains(&pos) {
                    continue;
                }
                path.insert(pos);
                stack.push((cost, pos, path));
            }
            Cell::Forest => unreachable!(),
        }
    }

    println!("{}", max);
}

fn part2(lines: impl Iterator<Item = String>) {
    let maze: Vec<Vec<_>> = lines.map(|l| l.chars().map(Cell::new).collect()).collect();
    let (x_len, y_len) = (maze[0].len(), maze.len());
    let start_x = maze[0].iter().position(|&c| c == Cell::Path).unwrap();
    let start = (start_x, 0);
    let goal_x = maze[y_len - 1]
        .iter()
        .position(|&c| c == Cell::Path)
        .unwrap();
    let goal = (goal_x, y_len - 1);

    let mut graph = HashMap::from([(start, HashMap::new())]);
    for y in 1..y_len {
        for x in 0..x_len {
            if maze[y][x] == Cell::Forest {
                continue;
            }
            let count = [
                y.checked_sub(1).map(|y| (x, y)),
                (x + 1 < x_len).then_some((x + 1, y)),
                (y + 1 < y_len).then_some((x, y + 1)),
                x.checked_sub(1).map(|x| (x, y)),
            ]
            .into_iter()
            .flatten()
            .filter(|&(x, y)| maze[y][x] != Cell::Forest)
            .count();
            if count >= 3 {
                graph.insert((x, y), HashMap::new());
            }
        }
    }

    struct Shortest {
        cost: usize,
        from: Pos,
        cur: Pos,
    }
    impl PartialEq for Shortest {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }
    impl Eq for Shortest {}
    impl Ord for Shortest {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    impl PartialOrd for Shortest {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut queue = BinaryHeap::from([Shortest {
        cost: 0,
        from: start,
        cur: start,
    }]);
    let mut seen = HashMap::new();

    while let Some(Shortest { cost, from, cur }) = queue.pop() {
        if cur == goal {
            graph.get_mut(&from).unwrap().insert(goal, cost);
            continue;
        }

        let (x, y) = cur;
        let cost = cost + 1;
        let adj = [
            y.checked_sub(1).map(|y| (x, y)),
            (x + 1 < x_len).then_some((x + 1, y)),
            (y + 1 < y_len).then_some((x, y + 1)),
            x.checked_sub(1).map(|x| (x, y)),
        ]
        .into_iter()
        .flatten()
        .filter(|&(x, y)| maze[y][x] != Cell::Forest);
        for cur in adj {
            if from == cur {
                continue;
            }
            if let Some(&(to, dist)) = seen.get(&cur) {
                if from != to {
                    graph.get_mut(&from).unwrap().insert(to, cost + dist);
                    graph.get_mut(&to).unwrap().insert(from, cost + dist);
                }
                continue;
            }
            let (from, cost) = if graph.contains_key(&cur) {
                graph.get_mut(&from).unwrap().insert(cur, cost);
                graph.get_mut(&cur).unwrap().insert(from, cost);
                (cur, 0)
            } else {
                seen.insert(cur, (from, cost));
                (from, cost)
            };
            queue.push(Shortest { cost, from, cur });
        }
    }
    let graph = graph;

    let mut stack = vec![(0, start, HashSet::new())];
    let mut max = 0;
    while let Some((cost, src, mut path)) = stack.pop() {
        if src == goal {
            max = max.max(cost);
            continue;
        }
        path.insert(src);
        for (&dest, dist) in graph[&src].iter() {
            if path.contains(&dest) {
                continue;
            }
            stack.push((cost + dist, dest, path.clone()));
        }
    }
    println!("{max}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
