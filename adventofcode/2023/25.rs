use rand::prelude::*;
use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    const NUM_CUT_EDGES: usize = 3;
    let mut rng = rand::thread_rng();
    let lines: Vec<_> = lines.collect();

    let mut edges = vec![];
    let mut nodes = HashSet::new();
    for line in lines.iter() {
        let (src, r) = line.split_once(": ").unwrap();
        nodes.insert(src);
        for dst in r.split(' ') {
            edges.push((src, dst));
            nodes.insert(dst);
        }
    }
    let (edges, nodes) = (edges, nodes);

    // Algorithm: https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/kew43y5/
    'again: loop {
        let (mut l, mut r) = (HashSet::new(), HashSet::new());
        let mut e = edges.clone();
        e.shuffle(&mut rng);

        for set in [&mut l, &mut r] {
            let (s, d) = e.pop().unwrap();
            set.insert(s);
            set.insert(d);
        }

        while e.len() > NUM_CUT_EDGES {
            let mut i = 0;
            while i < e.len() {
                let (src, dst) = e[i];
                if l.contains(&src) {
                    if r.contains(&dst) {
                        continue 'again;
                    }
                    l.insert(dst);
                } else if r.contains(&src) {
                    if l.contains(&dst) {
                        continue 'again;
                    }
                    r.insert(dst);
                } else if l.contains(&dst) {
                    if r.contains(&src) {
                        continue 'again;
                    }
                    l.insert(src);
                } else if r.contains(&dst) {
                    if l.contains(&src) {
                        continue 'again;
                    }
                    r.insert(src);
                } else {
                    i += 1;
                    continue; // We don't know which set this edge belongs to. Skip and check it later
                }

                e.swap_remove(i);

                if !l.is_empty() && !r.is_empty() && l.len() + r.len() == nodes.len() {
                    let cuts = edges
                        .iter()
                        .filter(|(s, d)| {
                            l.contains(s) && r.contains(d) || l.contains(d) && r.contains(s)
                        })
                        .count();
                    if cuts == NUM_CUT_EDGES {
                        println!("{}", l.len() * r.len());
                        return;
                    }
                }
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
