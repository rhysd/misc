use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type P = (i32, i32, i32);

fn parse_scanner(mut lines: impl Iterator<Item = String>) -> Option<Vec<P>> {
    if let Some(l) = lines.next() {
        assert!(l.starts_with("--- scanner "));
    } else {
        return None;
    }
    let mut coords = vec![];
    for l in lines {
        if l.is_empty() {
            break;
        }
        let mut s = l.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        coords.push((x, y, z));
    }
    Some(coords)
}

fn distance(p1: &P, p2: &P) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

fn adjust_to(scan1: &[P], scan2: &[P]) -> Option<(Vec<P>, P)> {
    const ROT: [fn(P) -> P; 24] = [
        |p| (-p.2, -p.1, -p.0),
        |p| (-p.2, -p.0, p.1),
        |p| (-p.2, p.0, -p.1),
        |p| (-p.2, p.1, p.0),
        |p| (-p.1, -p.2, p.0),
        |p| (-p.1, -p.0, -p.2),
        |p| (-p.1, p.0, p.2),
        |p| (-p.1, p.2, -p.0),
        |p| (-p.0, -p.2, -p.1),
        |p| (-p.0, -p.1, p.2),
        |p| (-p.0, p.1, -p.2),
        |p| (-p.0, p.2, p.1),
        |p| (p.0, -p.2, p.1),
        |p| (p.0, -p.1, -p.2),
        |p| (p.0, p.1, p.2),
        |p| (p.0, p.2, -p.1),
        |p| (p.1, -p.2, -p.0),
        |p| (p.1, -p.0, p.2),
        |p| (p.1, p.0, -p.2),
        |p| (p.1, p.2, p.0),
        |p| (p.2, -p.1, p.0),
        |p| (p.2, -p.0, -p.1),
        |p| (p.2, p.0, p.1),
        |p| (p.2, p.1, -p.0),
    ];

    fn diff((l, r): (P, P)) -> P {
        (r.0 - l.0, r.1 - l.1, r.2 - l.2)
    }

    for rot in ROT {
        let mut adjusts = HashMap::new();

        for bi in 0..scan1.len() {
            for bj in bi + 1..scan1.len() {
                let base = (scan1[bi], scan1[bj]);
                let base_diff = diff(base);
                for ui in 0..scan2.len() {
                    for uj in 0..scan2.len() {
                        if ui == uj {
                            continue;
                        }
                        let unadjusted = (rot(scan2[ui]), rot(scan2[uj]));
                        if base_diff == diff(unadjusted) {
                            let offset = diff((unadjusted.0, base.0));
                            assert_eq!(offset, diff((unadjusted.1, base.1)));
                            let e = adjusts.entry(offset).or_insert_with(HashMap::new);
                            e.insert(bi, ui);
                            e.insert(bj, uj);
                        }
                    }
                }
            }
        }

        if let Some((offset, mappings)) = adjusts.into_iter().max_by(|a, b| a.1.len().cmp(&b.1.len())) {
            if mappings.len() >= 12 {
                let adjusted = scan2
                    .iter()
                    .map(|p| {
                        let p = rot(*p);
                        (offset.0 + p.0, offset.1 + p.1, offset.2 + p.2)
                    })
                    .collect();
                return Some((adjusted, offset));
            }
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut adjusted = HashMap::new();
    adjusted.insert(0, parse_scanner(&mut lines).unwrap()); // Scanner 0

    let mut unadjusted = vec![];
    while let Some(s) = parse_scanner(&mut lines) {
        unadjusted.push(s);
    }
    let unadjusted = unadjusted;

    let num_scanners = unadjusted.len() + 1;
    let mut scanners = vec![(0, 0, 0)];

    let mut done = HashSet::new();
    let mut adjust_one = move |adjusted: &HashMap<usize, Vec<P>>, unadjusted: &[Vec<P>]| -> (usize, Vec<P>, P) {
        for (i, a) in adjusted.iter() {
            let i = *i;
            for (j, u) in unadjusted.iter().enumerate() {
                let j = j + 1;
                if i == j || done.contains(&(i, j)) {
                    continue;
                }
                done.insert((i, j));
                done.insert((j, i));
                if let Some((new, pos)) = adjust_to(a, u) {
                    return (j, new, pos);
                }
            }
        }
        unreachable!("Adjustable scanner was not found");
    };

    while adjusted.len() < num_scanners {
        let (num, scanner, pos) = adjust_one(&adjusted, &unadjusted);
        adjusted.insert(num, scanner);
        scanners.push(pos);
    }

    let mut s = HashSet::new();
    for a in adjusted.values() {
        for b in a.iter() {
            s.insert(b);
        }
    }
    println!("Part 1: {}", s.len());

    let max: i32 = scanners
        .iter()
        .filter_map(|a| scanners.iter().filter(|b| *b != a).map(|b| distance(a, b)).max())
        .max()
        .unwrap();
    println!("Part 2: {}", max);
}
