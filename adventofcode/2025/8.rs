use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::io::{self, BufRead};

type Pos = (f64, f64, f64);

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
struct SetHandle(usize);

#[derive(Default)]
pub struct DisjointSets<T> {
    items: HashMap<T, SetHandle>,
    links: Vec<SetHandle>,
}

impl<T: PartialEq + Eq + Hash> DisjointSets<T> {
    fn add(&mut self, value: T) {
        debug_assert!(!self.items.contains_key(&value));
        let handle = SetHandle(self.links.len());
        self.links.push(handle);
        self.items.insert(value, handle);
    }

    fn union(&mut self, a: SetHandle, b: SetHandle) {
        let (i, h) = if a.0 < b.0 { (b.0, a) } else { (a.0, b) };
        self.links[i] = h;
    }

    fn resolve(&mut self, handle: SetHandle) -> SetHandle {
        let h = self.links[handle.0];
        if handle == h {
            return handle;
        }
        let h = self.resolve(h);
        self.links[handle.0] = h;
        h
    }

    fn find(&mut self, a: &T) -> Option<SetHandle> {
        let handle = self.items.get(a).copied()?;
        Some(self.resolve(handle))
    }

    fn sizes(&mut self) -> Vec<usize> {
        for i in 0..self.links.len() {
            self.resolve(SetHandle(i));
        }
        let mut sizes = vec![0; self.links.len()];
        for h in self.items.values() {
            sizes[self.links[h.0].0] += 1;
        }
        sizes
    }
}

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

fn part1(lines: impl Iterator<Item = String>) {
    let (boxes, dists) = parse(lines);

    let mut d = DisjointSets::default();
    for i in 0..boxes.len() {
        d.add(i);
    }
    for (i, j, _) in dists.into_iter().take(1000) {
        let a = d.find(&i).unwrap();
        let b = d.find(&j).unwrap();
        d.union(a, b);
    }

    let mut sizes = d.sizes();
    let i = sizes.len() - 3;
    let (_, a, b) = sizes.select_nth_unstable(i);
    println!("{}", *a * b[0] * b[1]);
}

fn part2(lines: impl Iterator<Item = String>) {
    let (boxes, dists) = parse(lines);

    let mut d = DisjointSets::default();
    for i in 0..boxes.len() {
        d.add(i);
    }

    let mut last = (0, 0);
    for (i, j, _) in dists.into_iter() {
        let a = d.find(&i).unwrap();
        let b = d.find(&j).unwrap();
        if a == b {
            continue;
        }
        d.union(a, b);
        last = (i, j);
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
