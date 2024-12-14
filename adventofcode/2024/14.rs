use std::env;
use std::io::{self, BufRead};
use std::ops::Range;

type Pos = (i32, i32);

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robots(Vec<(Pos, Pos)>);

impl Robots {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let robo = lines
            .map(|l| {
                let mut s = l.split_whitespace();
                let mut pos = |label| {
                    let s = s.next().unwrap();
                    let mut s = s.strip_prefix(label).unwrap().split(',');
                    (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
                };
                (pos("p="), pos("v="))
            })
            .collect();

        Self(robo)
    }

    fn update(&mut self) {
        for r in self.0.iter_mut() {
            let ((x, y), (dx, dy)) = *r;
            // Note: Using `rem_euclid` is smarter but it is 14% slower probably due to extra non-zero check on rhs.
            // ```
            // r.0 = ((x + dx).rem_euclid(WIDTH), (y + dy).rem_euclid(HEIGHT));
            // ```
            let (mut x, mut y) = (x + dx, y + dy);
            if x < 0 {
                x += WIDTH;
            } else if WIDTH <= x {
                x -= WIDTH;
            }
            if y < 0 {
                y += HEIGHT;
            } else if HEIGHT <= y {
                y -= HEIGHT;
            }
            r.0 = (x, y);
        }
    }

    fn count_within(&self, xr: Range<i32>, yr: Range<i32>) -> usize {
        self.0.iter().filter(|&((x, y), _)| xr.contains(x) && yr.contains(y)).count()
    }

    fn is_merry_christmas(&self) -> bool {
        const TREE: &[&[u8]] = &[
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

        let mut rendered = [[b' '; WIDTH as usize]; HEIGHT as usize];
        for &((x, y), _) in &self.0 {
            rendered[y as usize][x as usize] = b'X';
        }

        rendered.windows(TREE.len()).any(|ys| {
            let Some(x) = ys[0].windows(TREE[0].len()).position(|w| w == TREE[0]) else {
                return false;
            };
            ys.iter().enumerate().skip(1).all(|(y, xs)| xs[x..].starts_with(TREE[y]))
        })
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut robots = Robots::parse(lines);

    // Note: This is as fast as `(x + dx * 100).rem_euclid(WIDTH)` thanks to optimizations.
    for _ in 0..100 {
        robots.update();
    }

    let factor: usize = [
        (0..WIDTH / 2, 0..HEIGHT / 2),
        (WIDTH / 2 + 1..WIDTH, 0..HEIGHT / 2),
        (0..WIDTH / 2, HEIGHT / 2 + 1..HEIGHT),
        (WIDTH / 2 + 1..WIDTH, HEIGHT / 2 + 1..HEIGHT),
    ]
    .into_iter()
    .map(|(xr, yr)| robots.count_within(xr, yr))
    .product();

    println!("{factor}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut robots = Robots::parse(lines);
    for secs in 1.. {
        robots.update();
        if robots.is_merry_christmas() {
            println!("{secs}");
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
