use std::collections::HashSet;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

fn main() {
    let stdin = io::stdin();
    let steps = stdin.lock().lines().map(|l| {
        let l = l.unwrap();
        let (on, l) = if let Some(l) = l.strip_prefix("on ") {
            (true, l)
        } else if let Some(l) = l.strip_prefix("off ") {
            (false, l)
        } else {
            panic!("neither on nor off");
        };
        let mut s = l.split(',');
        fn parse_range(s: &str) -> RangeInclusive<i32> {
            let (l, r) = s[2..].split_once("..").unwrap();
            let l = l.parse().unwrap();
            let r = r.parse().unwrap();
            l..=r
        }
        let x = parse_range(s.next().unwrap());
        let y = parse_range(s.next().unwrap());
        let z = parse_range(s.next().unwrap());
        (on, x, y, z)
    });

    let mut cubes = HashSet::new();
    for (on, x, y, z) in steps {
        for x in x {
            if x < -50 || 50 < x {
                continue;
            }
            for y in y.clone() {
                if y < -50 || 50 < y {
                    continue;
                }
                for z in z.clone() {
                    if z < -50 || 50 < z {
                        continue;
                    }
                    let c = (x, y, z);
                    if on {
                        cubes.insert(c);
                    } else {
                        cubes.remove(&c);
                    }
                }
            }
        }
    }
    println!("{}", cubes.len());
}
