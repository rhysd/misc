use std::env;
use std::io::{self, BufRead};

type Pos = (i32, i32);

fn part1(lines: impl Iterator<Item = String>) {
    let mut robots: Vec<_> = lines
        .map(|l| {
            fn parse_pos(s: &str) -> Pos {
                let mut s = s.split(',');
                (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
            }
            let mut s = l.split_whitespace();
            (
                parse_pos(s.next().unwrap().strip_prefix("p=").unwrap()),
                parse_pos(s.next().unwrap().strip_prefix("v=").unwrap()),
            )
        })
        .collect();

    let width = 101;
    let height = 103;
    for _ in 0..100 {
        for r in robots.iter_mut() {
            let ((x, y), (dx, dy)) = *r;
            let (mut x, mut y) = (x + dx, y + dy);
            if x < 0 {
                x += width;
            } else if width <= x {
                x -= width;
            }
            if y < 0 {
                y += height;
            } else if height <= y {
                y -= height;
            }
            r.0 = (x, y);
        }
    }

    let factor: usize = [
        (0..width / 2, 0..height / 2),
        (width / 2 + 1..width, 0..height / 2),
        (0..width / 2, height / 2 + 1..height),
        (width / 2 + 1..width, height / 2 + 1..height),
    ]
    .into_iter()
    .map(|(xr, yr)| robots.iter().filter(|&((x, y), _)| xr.contains(x) && yr.contains(y)).count())
    .product();

    println!("{factor}");
}

const THE_TREE: &[&[u8]] = &[
    b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    b"X                             X",
    b"X                             X",
    b"X                             X",
    b"X                             X",
    b"X              X              X",
    b"X             XXX             X",
    b"X            XXXXX            X",
    b"X           XXXXXXX           X",
    b"X          XXXXXXXXX          X",
    b"X            XXXXX            X",
    b"X           XXXXXXX           X",
    b"X          XXXXXXXXX          X",
    b"X         XXXXXXXXXXX         X",
    b"X        XXXXXXXXXXXXX        X",
    b"X          XXXXXXXXX          X",
    b"X         XXXXXXXXXXX         X",
    b"X        XXXXXXXXXXXXX        X",
    b"X       XXXXXXXXXXXXXXX       X",
    b"X      XXXXXXXXXXXXXXXXX      X",
    b"X        XXXXXXXXXXXXX        X",
    b"X       XXXXXXXXXXXXXXX       X",
    b"X      XXXXXXXXXXXXXXXXX      X",
    b"X     XXXXXXXXXXXXXXXXXXX     X",
    b"X    XXXXXXXXXXXXXXXXXXXXX    X",
    b"X             XXX             X",
    b"X             XXX             X",
    b"X             XXX             X",
    b"X                             X",
    b"X                             X",
    b"X                             X",
    b"X                             X",
    b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
];

fn part2(lines: impl Iterator<Item = String>) {
    let mut robots: Vec<_> = lines
        .map(|l| {
            fn parse_pos(s: &str) -> Pos {
                let mut s = s.split(',');
                (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
            }
            let mut s = l.split_whitespace();
            (
                parse_pos(s.next().unwrap().strip_prefix("p=").unwrap()),
                parse_pos(s.next().unwrap().strip_prefix("v=").unwrap()),
            )
        })
        .collect();

    let width = 101;
    let height = 103;
    for i in 0..10000 {
        for r in robots.iter_mut() {
            let ((x, y), (dx, dy)) = *r;
            let (mut x, mut y) = (x + dx, y + dy);
            if x < 0 {
                x += width;
            } else if width <= x {
                x -= width;
            }
            if y < 0 {
                y += height;
            } else if height <= y {
                y -= height;
            }
            r.0 = (x, y);
        }

        for y in 0..height - THE_TREE.len() as i32 {
            'tile: for x in 0..width - THE_TREE[0].len() as i32 {
                for (ty, txs) in THE_TREE.iter().enumerate() {
                    for (tx, &b) in txs.iter().enumerate() {
                        let (x, y) = (x + tx as i32, y + ty as i32);
                        let is_robot = robots.iter().any(|&(p, _)| p == (x, y));
                        if b == b' ' && is_robot || b == b'X' && !is_robot {
                            continue 'tile;
                        }
                    }
                }
                println!("{}", i + 1);
                return;
            }
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
