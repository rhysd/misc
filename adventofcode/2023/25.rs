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

    // Randomized algorithm. Try to find the answer and retry if not found. There are only 3 cut nodes.
    // So almost all edges belong to one of the two node groups. Optimistically consider randomly chosen
    // edges are not a cut.
    // When one node of a edge is in a group, assume another one is also in the group. Repeat it until all
    // edges are processed. Finally check the built two sets meet the requirements by counting cut nodes.
    loop {
        let (mut l, mut r) = (HashSet::new(), HashSet::new());
        let mut e = edges.clone();
        e.shuffle(&mut rng);

        // Assume the first two edges are belong to `l` and `r` respectively
        for set in [&mut l, &mut r] {
            let (s, d) = e.pop().unwrap();
            set.insert(s);
            set.insert(d);
        }

        while !e.is_empty() {
            let mut i = 0;
            while i < e.len() {
                let (src, dst) = e[i];

                let inserted = if l.contains(&src) {
                    // When `src` is in `l` and `dst` is in `r`, we assume this edge is a cut. Cut nodes are
                    // useless to separate nodes into two groups so we ignore them.
                    !r.contains(&dst) && l.insert(dst)
                } else if r.contains(&src) {
                    !l.contains(&dst) && r.insert(dst)
                } else if l.contains(&dst) {
                    !r.contains(&src) && l.insert(src)
                } else if r.contains(&dst) {
                    !l.contains(&src) && r.insert(src)
                } else {
                    // We don't know which set this edge belongs to. Skip and check it again in later iteration
                    i += 1;
                    continue;
                };

                e.swap_remove(i);

                if inserted && !l.is_empty() && !r.is_empty() && l.len() + r.len() == nodes.len() {
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
