// This implementation is much faster than 19.rs (about 100x faster, 7.8s vs 0.08s on my machine).
// I read solution megathread for day 19 and knew using fingerprint dramatically reduced the order.
// This is my try of the idea.

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

type FP = HashMap<i32, (usize, usize)>;
type M = HashMap<usize, HashSet<usize>>; // Mapping from index of adjusted scanner to candidates of indices of unadjusted scanner

fn matches_fingerprints(fp1: &FP, fp2: &FP) -> Option<M> {
    let mut map = HashMap::new();
    for (fp1, (from1, to1)) in fp1.iter() {
        if let Some((from2, to2)) = fp2.get(fp1) {
            map.entry(*from1).or_insert_with(HashSet::new).insert(*from2);
            map.entry(*to1).or_insert_with(HashSet::new).insert(*to2);
        }
    }
    (map.len() >= 12).then(|| map)
}

fn matches_exactly(adjusted: &[P], candidate: &[P], mapping: &M) -> Option<(Vec<P>, P)> {
    for rot in ROT {
        let mut mapping = mapping.iter();
        let (i, v) = mapping.next().unwrap();
        let a = &adjusted[*i];
        for j in v.iter() {
            let c = rot(candidate[*j]);
            let offset = (a.0 - c.0, a.1 - c.1, a.2 - c.2);
            if mapping.all(|(i, v)| {
                let a = &adjusted[*i];
                v.iter().any(|j| {
                    let c = rot(candidate[*j]);
                    a == &(c.0 + offset.0, c.1 + offset.1, c.2 + offset.2)
                })
            }) {
                let new: Vec<_> = candidate
                    .iter()
                    .map(|b| {
                        let b = rot(*b);
                        (b.0 + offset.0, b.1 + offset.1, b.2 + offset.2)
                    })
                    .collect();
                return Some((new, offset));
            }
        }
    }
    None
}

// Fingerprint by square of distance. Distance is not affected by orientation. So we can reduce the combinations.
fn fingerprint(b1: &P, b2: &P) -> i32 {
    let (p, q, r) = (b1.0 - b2.0, b1.1 - b2.1, b1.2 - b2.2);
    p * p + q * q + r * r
}

fn beacon_fingerprints(scanner: &[P]) -> FP {
    let len = scanner.len();
    let mut fps = HashMap::with_capacity(len * (len - 1) / 2);
    for i in 0..len {
        for j in i + 1..len {
            let fp = fingerprint(&scanner[i], &scanner[j]);
            assert!(!fps.contains_key(&fp));
            fps.insert(fp, (i, j));
        }
    }
    fps
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

    let num_scanners = unadjusted.len() + 1;
    let mut fps = Vec::with_capacity(num_scanners);
    fps.push(beacon_fingerprints(adjusted.get(&0).unwrap()));
    for s in unadjusted.iter() {
        fps.push(beacon_fingerprints(s));
    }
    let fps = fps;

    let mut done = HashSet::new();
    let mut adjust_one = move |adjusted: &HashMap<usize, Vec<P>>, unadjusted: &[Vec<P>]| {
        for (ai, a) in adjusted.iter() {
            let ai = *ai;
            for i in 0..unadjusted.len() {
                let ui = i + 1;
                if ai == ui || done.contains(&(ai, ui)) {
                    continue;
                }
                done.insert((ai, ui));
                let afps = &fps[ai];
                let ufps = &fps[ui];
                if let Some(mapping) = matches_fingerprints(afps, ufps) {
                    let u = &unadjusted[i];
                    if let Some((new, offset)) = matches_exactly(a, u, &mapping) {
                        return (ui, new, offset);
                    }
                }
            }
        }
        unreachable!()
    };

    let mut scanners = vec![(0, 0, 0)];
    while adjusted.len() < num_scanners {
        let (idx, new, pos) = adjust_one(&adjusted, &unadjusted);
        adjusted.insert(idx, new);
        scanners.push(pos);
    }

    let count: usize = adjusted
        .values()
        .map(|a| a.iter())
        .flatten()
        .collect::<HashSet<_>>()
        .len();
    println!("Part 1: {:?}", count);

    let max: i32 = scanners
        .iter()
        .filter_map(|a| {
            scanners
                .iter()
                .filter(|b| *b != a)
                .map(|b| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
                .max()
        })
        .max()
        .unwrap();
    println!("Part 2: {}", max);
}
