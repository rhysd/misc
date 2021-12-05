use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    type Point = (usize, usize);
    fn parse_point(input: &str) -> Point {
        let mut s = input.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        (x, y)
    }

    let s = io::stdin();
    let s = s.lock();
    let lines = io::BufReader::new(s).lines().map(|l| {
        let l = l.unwrap();
        let mut s = l.split(" -> ");
        (parse_point(s.next().unwrap()), parse_point(s.next().unwrap()))
    });

    let mut points = HashMap::new();
    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                *points.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                *points.entry((x, y1)).or_insert(0) += 1;
            }
        } else {
            fn mark(m: &mut HashMap<Point, usize>, xs: impl Iterator<Item = usize>, ys: impl Iterator<Item = usize>) {
                for (x, y) in xs.zip(ys) {
                    *m.entry((x, y)).or_insert(0) += 1;
                }
            }
            match (x1 < x2, y1 < y2) {
                (true, true) => mark(&mut points, x1..=x2, y1..=y2),
                (true, false) => mark(&mut points, x1..=x2, (y2..=y1).rev()),
                (false, true) => mark(&mut points, (x2..=x1).rev(), y1..=y2),
                (false, false) => mark(&mut points, (x2..=x1).rev(), (y2..=y1).rev()),
            }
        }
    }

    println!("{}", points.values().filter(|&&v| v >= 2).count());
}
