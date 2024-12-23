use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

type Nodes<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_nodes(lines: &[String]) -> Nodes<'_> {
    let mut m = HashMap::<&str, Vec<&str>>::new();
    for line in lines {
        let mut s = line.split('-');
        let (l, r) = (s.next().unwrap(), s.next().unwrap());
        m.entry(l).or_default().push(r);
        m.entry(r).or_default().push(l);
    }
    m
}

fn collect_minimal_cycles<'a>(nodes: &Nodes<'a>) -> HashSet<[&'a str; 3]> {
    let mut ret = HashSet::new();
    for (&n1, ns) in nodes {
        for &n2 in ns {
            for &n3 in &nodes[n2] {
                if nodes[n3].contains(&n1) {
                    let mut s = [n1, n2, n3];
                    s.sort();
                    ret.insert(s);
                }
            }
        }
    }
    ret
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let nodes = parse_nodes(&lines);
    let sets = collect_minimal_cycles(&nodes);
    let count = sets.iter().filter(|s| s.iter().any(|n| n.starts_with('t'))).count();
    println!("{count}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let nodes = parse_nodes(&lines);
    let init = collect_minimal_cycles(&nodes);

    let is_interconnected = |s1: &HashSet<&str>, s2: &HashSet<&str>| {
        s1.iter().all(|n1| {
            let adj = &nodes[n1];
            s2.iter().all(|n2| n1 == n2 || adj.contains(n2))
        })
    };

    let mut largest = HashSet::new();
    let mut sets: Vec<_> = init.into_iter().map(HashSet::from).collect();
    while let Some(s) = sets.pop() {
        if let Some(dest) = sets.iter_mut().find(|s2| is_interconnected(&s, s2)) {
            dest.extend(s.into_iter());
        } else if largest.len() < s.len() {
            largest = s;
        };
    }

    let mut largest: Vec<_> = largest.into_iter().collect();
    largest.sort();
    println!("{}", largest.join(","));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
