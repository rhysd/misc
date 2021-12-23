use std::cmp::{max, min};
use std::io::{self, BufRead};

type Range = (i32, i32);
type Cuboid = (Range, Range, Range);

fn main() {
    let stdin = io::stdin();
    let steps = stdin.lock().lines().map(|l| {
        let l = l.unwrap();
        let (on, l) = if let Some(l) = l.strip_prefix("on ") {
            (true, l)
        } else if let Some(l) = l.strip_prefix("off ") {
            (false, l)
        } else {
            panic!("neither on nor off: {:?}", l);
        };
        let mut s = l.split(',');
        let mut parse_range = move || {
            let (l, r) = s.next().unwrap()[2..].split_once("..").unwrap();
            let l: i32 = l.parse().unwrap();
            let r: i32 = r.parse().unwrap();
            assert!(l <= r);
            (l, r + 1) // Make range exclusive. This is the most important point!
        };
        let cuboid = (parse_range(), parse_range(), parse_range());
        (on, cuboid)
    });

    fn overwrap(c1: Cuboid, c2: Cuboid) -> Option<Cuboid> {
        fn overwrap_line((s1, e1): Range, (s2, e2): Range) -> Option<Range> {
            let (s, e) = (max(s1, s2), min(e1, e2));
            (s < e).then(|| (s, e))
        }
        let ((x1, y1, z1), (x2, y2, z2)) = (c1, c2);
        let x = overwrap_line(x1, x2)?;
        let y = overwrap_line(y1, y2)?;
        let z = overwrap_line(z1, z2)?;
        Some((x, y, z))
    }

    let mut cuboids = vec![];
    for (on, cuboid) in steps {
        for i in (0..cuboids.len()).rev() {
            let c = cuboids[i];
            if let Some(overwrapped) = overwrap(c, cuboid) {
                let ((cx, cy, cz), (ox, oy, oz)) = (c, overwrapped);
                let candidates = |(s, e), (os, oe)| [(s, os), (os, oe), (oe, e)].into_iter().filter(|(s, e)| s < e);
                for x in candidates(cx, ox) {
                    for y in candidates(cy, oy) {
                        for z in candidates(cz, oz) {
                            let c = (x, y, z);
                            if c != overwrapped {
                                cuboids.push(c);
                            }
                        }
                    }
                }
                cuboids.swap_remove(i);
            }
        }
        if on {
            cuboids.push(cuboid);
        }
    }

    fn volume(((xs, xe), (ys, ye), (zs, ze)): Cuboid) -> u64 {
        let x = (xe - xs) as i64;
        let y = (ye - ys) as i64;
        let z = (ze - zs) as i64;
        assert!(x > 0 && y > 0 && z > 0);
        (x * y * z) as u64
    }
    println!("{}", cuboids.into_iter().map(volume).sum::<u64>());
}
