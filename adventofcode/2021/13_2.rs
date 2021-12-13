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
    let mut dots: HashSet<_> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.split(',');
            let x: usize = s.next().unwrap().parse().unwrap();
            let y: usize = s.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let insns = lines.map(|l| {
        if let Some(s) = l.strip_prefix("fold along y=") {
            Insn::Y(s.parse().unwrap())
        } else if let Some(s) = l.strip_prefix("fold along x=") {
            Insn::X(s.parse().unwrap())
        } else {
            panic!("Invalid line: {:?}", l);
        }
    });

    for insn in insns {
        dots = dots
            .into_iter()
            .map(|(x, y)| match insn {
                Insn::X(fx) if x > fx => (fx * 2 - x, y),
                Insn::Y(fy) if y > fy => (x, fy * 2 - y),
                _ => (x, y),
            })
            .collect();
    }

    let max_x: usize = dots.iter().map(|(x, _)| *x).max().unwrap();
    let max_y: usize = dots.iter().map(|(_, y)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = if dots.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
}
