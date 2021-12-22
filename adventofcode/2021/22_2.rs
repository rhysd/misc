use std::cmp;
use std::io::{self, BufRead};

type Range = (i32, i32); // Range
type Cuboid = (Range, Range, Range); // Cuboid

fn overwrap(c1: Cuboid, c2: Cuboid) -> Option<Cuboid> {
    fn overwrap_line((s1, e1): Range, (s2, e2): Range) -> Option<Range> {
        let s = cmp::max(s1, s2);
        let e = cmp::min(e1, e2);
        (s < e).then(|| (s, e))
    }
    let (x1, y1, z1) = c1;
    let (x2, y2, z2) = c2;
    let x = overwrap_line(x1, x2)?;
    let y = overwrap_line(y1, y2)?;
    let z = overwrap_line(z1, z2)?;
    Some((x, y, z))
}

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
        let mut parse_range = move || {
            let (l, r) = s.next().unwrap()[2..].split_once("..").unwrap();
            let l: i32 = l.parse().unwrap();
            let r: i32 = r.parse().unwrap();
            assert!(l <= r);
            (l, r + 1) // Make range exclusive. This is the most important point!
        };
        let x = parse_range();
        let y = parse_range();
        let z = parse_range();
        (on, (x, y, z))
    });

    let mut cubes = vec![];
    for (on, cube) in steps {
        for i in (0..cubes.len()).rev() {
            let c = cubes[i];
            if let Some(overwrapped) = overwrap(c, cube) {
                let ((oxs, oxe), (oys, oye), (ozs, oze)) = overwrapped;
                let ((xs, xe), (ys, ye), (zs, ze)) = c;
                for x in [(xs, oxs), (oxs, oxe), (oxe, xe)].into_iter().filter(|(s, e)| s < e) {
                    for y in [(ys, oys), (oys, oye), (oye, ye)].into_iter().filter(|(s, e)| s < e) {
                        for z in [(zs, ozs), (ozs, oze), (oze, ze)].into_iter().filter(|(s, e)| s < e) {
                            let c = (x, y, z);
                            if c != overwrapped {
                                cubes.push(c);
                            }
                        }
                    }
                }
                cubes.swap_remove(i);
            }
        }
        if on {
            cubes.push(cube);
        }
    }

    fn volume(((xs, xe), (ys, ye), (zs, ze)): Cuboid) -> u64 {
        let x = (xe - xs) as i64;
        let y = (ye - ys) as i64;
        let z = (ze - zs) as i64;
        (x * y * z) as u64
    }
    println!("{}", cubes.into_iter().map(volume).sum::<u64>());
}
