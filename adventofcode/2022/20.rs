use std::env;
use std::io::{self, BufRead};

fn mix(n: &mut [(usize, i64)]) {
    let len_after_pop = n.len() as i64 - 1;
    for idx in 0..n.len() {
        let (from, v) = n
            .iter()
            .enumerate()
            .find_map(|(i, (j, v))| (*j == idx).then_some((i, *v)))
            .unwrap();
        n[from..].rotate_left(1); // Move the target element to the end to pop it from the list temporarily
        let to = (from as i64 + v).rem_euclid(len_after_pop) as usize;
        n[to..].rotate_right(1); // Push the element to the position after the move
    }
}

fn answer(n: &[(usize, i64)]) -> i64 {
    let mut indices = (0..n.len()).cycle();
    while n[indices.next().unwrap()].1 != 0 {}
    let mut f = || n[indices.nth(999).unwrap()].1;
    f() + f() + f()
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut n = lines
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();
    mix(&mut n);
    println!("{}", answer(&n));
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut n = lines
        .map(|s| s.parse::<i64>().unwrap() * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    for _ in 0..10 {
        mix(&mut n);
    }
    println!("{}", answer(&n));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
