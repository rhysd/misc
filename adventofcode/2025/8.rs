use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

type Pos = (f64, f64, f64);

fn parse(lines: impl Iterator<Item = String>) -> (Vec<Pos>, Vec<(usize, usize, f64)>) {
    let boxes: Vec<Pos> = lines
        .map(|l| {
            let mut s = l.split(',').map(|s| s.parse().unwrap());
            (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
        })
        .collect();

    let mut dists = vec![];
    for i in 0..boxes.len() {
        let (x1, y1, z1) = boxes[i];
        for j in i + 1..boxes.len() {
            let (x2, y2, z2) = boxes[j];
            let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt();
            dists.push((i, j, dist));
        }
    }
    dists.sort_unstable_by(|l, r| l.2.total_cmp(&r.2));

    (boxes, dists)
}

fn connect(i: usize, j: usize, circuits: &mut Vec<HashSet<usize>>) -> bool {
    let ci = circuits.iter().position(|s| s.contains(&i));
    let cj = circuits.iter().position(|s| s.contains(&j));
    match (ci, cj) {
        (Some(ci), Some(cj)) if ci == cj => return false,
        (Some(ci), Some(cj)) => {
            let s = circuits.remove(ci.max(cj));
            circuits[ci.min(cj)].extend(s);
        }
        (Some(ci), None) => {
            circuits[ci].insert(j);
        }
        (None, Some(cj)) => {
            circuits[cj].insert(i);
        }
        (None, None) => circuits.push(HashSet::from([i, j])),
    }
    true
}

fn part1(lines: impl Iterator<Item = String>) {
    let (_, dists) = parse(lines);

    let mut circuits = vec![];
    for (i, j, _) in dists.into_iter().take(1000) {
        connect(i, j, &mut circuits);
    }
    circuits.sort_unstable_by_key(|s| s.len());
    let answer: usize = circuits[circuits.len() - 3..].iter().map(HashSet::len).product();
    println!("{answer}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let (boxes, dists) = parse(lines);

    let mut circuits = vec![];
    let mut last = (0, 0);
    for (i, j, _) in dists.into_iter() {
        if connect(i, j, &mut circuits) {
            last = (i, j);
        }
    }
    println!("{}", (boxes[last.0].0 * boxes[last.1].0) as u64);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
