use fxhash::FxHashSet;
use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

fn parse(lines: impl Iterator<Item = String>) -> FxHashSet<Pos> {
    fn parse_pos(s: &str) -> Pos {
        let mut s = s.split(',').map(|s| s.parse().unwrap());
        (s.next().unwrap(), s.next().unwrap())
    }

    let mut ret = FxHashSet::default();
    for line in lines {
        let mut s = line.split(" -> ").map(parse_pos);
        let mut cur = s.next().unwrap();
        for (x, y) in s {
            let (cx, cy) = cur;
            if x == cx {
                ret.extend((y.min(cy)..=y.max(cy)).map(|y| (x, y)));
            } else if y == cy {
                ret.extend((x.min(cx)..=x.max(cx)).map(|x| (x, y)));
            } else {
                panic!("diagonal input is not supported: {:?} -> {:?}", cur, (x, y));
            }
            cur = (x, y);
        }
    }
    ret
}

const START: Pos = (500, 0);

fn part1(lines: impl Iterator<Item = String>) {
    let mut blocks = parse(lines);
    let ymax = *blocks.iter().map(|(_, y)| y).max().unwrap();

    for count in 1.. {
        let mut pos = START;
        loop {
            let (x, y) = pos;
            if y > ymax {
                println!("{}", count - 1);
                return;
            }

            if let Some(next) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .iter()
                .find(|p| !blocks.contains(p))
            {
                pos = *next;
                continue;
            }

            blocks.insert(pos);
            break;
        }
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut blocks = parse(lines);
    let (ymax, xmax, xmin) = {
        let (mut ymax, mut xmax, mut xmin) = (0, 0, usize::MAX);
        for (x, y) in blocks.iter().copied() {
            (ymax, xmax, xmin) = (ymax.max(y), xmax.max(x), xmin.min(x));
        }
        (ymax + 1, xmax + 1, xmin - 1)
    };

    let mut count = 1;
    loop {
        let mut pos = START;
        loop {
            let (x, y) = pos;
            if y == ymax {
                blocks.insert(pos);
                break;
            }

            if let Some(next) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .iter()
                .filter(|(x, _)| (xmin..=xmax).contains(x))
                .find(|p| !blocks.contains(p))
            {
                pos = *next;
                continue;
            }

            if pos == START {
                println!("{}", count);
                return;
            }

            blocks.insert(pos);
            break;
        }

        count += if pos.0 == xmin || pos.0 == xmax {
            ymax - pos.1 + 1
        } else {
            1
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
