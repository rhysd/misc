use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Insn {
    X(usize),
    Y(usize),
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let dots: Vec<_> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.split(',');
            let x: usize = s.next().unwrap().parse().unwrap();
            let y: usize = s.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let insn = lines
        .map(|l| {
            if let Some(s) = l.strip_prefix("fold along y=") {
                Insn::Y(s.parse().unwrap())
            } else if let Some(s) = l.strip_prefix("fold along x=") {
                Insn::X(s.parse().unwrap())
            } else {
                panic!("Invalid line: {:?}", l);
            }
        })
        .next()
        .unwrap();

    let mut next: HashSet<_> = dots.iter().copied().collect();
    for (dx, dy) in dots.into_iter() {
        let p = match insn {
            Insn::X(x) if dx > x => (x * 2 - dx, dy),
            Insn::Y(y) if dy > y => (dx, y * 2 - dy),
            _ => continue,
        };
        next.insert(p);
        next.remove(&(dx, dy));
    }

    println!("{}", next.len());
}
