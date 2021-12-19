use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};

type P = (i32, i32, i32);

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
const BY: [fn(&P, &P) -> Ordering; 3] = [|a, b| a.0.cmp(&b.0), |a, b| a.1.cmp(&b.1), |a, b| a.2.cmp(&b.2)];

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

fn adjust_to(base: &[P], unadjusted: &[P]) -> Option<Vec<P>> {
    fn sub(a: &P, b: &P) -> P {
        (a.0 - b.0, a.1 - b.1, a.2 - b.2)
    }

    let mut base = base.to_owned();
    for rot in ROT {
        let unadjusted: Vec<_> = unadjusted.iter().map(|p| rot(*p)).collect();
        for by in BY {
            let mut unadjusted = unadjusted.clone();
            let diffs = |beacons: &mut [P]| {
                beacons.sort_by(by);
                beacons
                    .iter()
                    .zip(beacons.iter().skip(1))
                    .map(|(a, b)| sub(b, a))
                    .collect::<HashSet<_>>()
            };

            let base_diffs = diffs(&mut base);
            let unadjusted_diffs = diffs(&mut unadjusted);
            let common_diffs = base_diffs.intersection(&unadjusted_diffs);

            if let Some(diff) = common_diffs.into_iter().next() {
                let index = |beacons: &[P]| {
                    beacons
                        .iter()
                        .zip(beacons.iter().skip(1))
                        .position(|(a, b)| sub(b, a) == *diff)
                        .unwrap()
                };
                let offset = sub(&unadjusted[index(&unadjusted)], &base[index(&base)]);
                let mut count = 0;
                for p in unadjusted.iter_mut() {
                    let adjusted = sub(p, &offset);
                    count += base.iter().any(|p| p == &adjusted) as usize;
                    *p = adjusted;
                }
                if count >= 12 {
                    return Some(unadjusted);
                }
            }
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut adjusted = vec![parse_scanner(&mut lines).unwrap()]; // Scanner 0

    let mut unadjusted = vec![];
    while let Some(s) = parse_scanner(&mut lines) {
        unadjusted.push(s);
    }

    'adjusting: while !unadjusted.is_empty() {
        for a in adjusted.iter() {
            for idx in 0..unadjusted.len() {
                if let Some(new) = adjust_to(a, &unadjusted[idx]) {
                    unadjusted.remove(idx);
                    adjusted.push(new);
                    continue 'adjusting;
                }
            }
        }
    }
    let mut s = HashSet::new();
    for a in adjusted {
        for b in a {
            s.insert(b);
        }
    }
    println!("{:?}", s.len());
}
