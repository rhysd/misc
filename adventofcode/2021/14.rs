use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut pairs = HashMap::new();
    let template = lines.next().unwrap();
    for (a, b) in template.chars().zip(template.chars().skip(1)) {
        *pairs.entry((a, b)).or_insert(0) += 1;
    }

    let rules: HashMap<_, _> = lines
        .skip_while(|l| l.is_empty())
        .map(|l| {
            let mut s = l.split(" -> ");
            let mut l = s.next().unwrap().chars();
            let mut r = s.next().unwrap().chars();
            ((l.next().unwrap(), l.next().unwrap()), r.next().unwrap())
        })
        .collect();

    for _ in 0..40 {
        let mods: Vec<_> = pairs
            .iter()
            .filter_map(|(pair, count)| rules.get(pair).map(|c| (*pair, *count, *c)))
            .collect();
        for ((l, r), count, c) in mods {
            *pairs.get_mut(&(l, r)).unwrap() -= count;
            *pairs.entry((l, c)).or_insert(0) += count;
            *pairs.entry((c, r)).or_insert(0) += count;
        }
        pairs.retain(|_, c| *c > 0);
    }

    let mut counts = HashMap::new();
    for ((l, r), count) in pairs.into_iter() {
        *counts.entry(l).or_insert(0) += count;
        *counts.entry(r).or_insert(0) += count;
    }

    // Characters at both edges of the output are appearing only once. Adjust them for `/ 2` later
    let mut c = template.chars();
    *counts.entry(c.next().unwrap()).or_insert(0) += 1;
    *counts.entry(c.last().unwrap()).or_insert(0) += 1;

    let max: usize = counts.iter().map(|(_, c)| *c).max().unwrap() / 2;
    let min: usize = counts.iter().map(|(_, c)| *c).min().unwrap() / 2;
    println!("{}", max - min);
}
