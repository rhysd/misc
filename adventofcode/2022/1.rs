use std::cmp;
use std::env;
use std::io::{self, BufRead};

fn part1() {
    let mut max = 0;
    let mut cur = 0;
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        if line.is_empty() {
            max = cmp::max(max, cur);
            cur = 0;
        } else {
            cur += line.parse::<usize>().unwrap();
        }
    }
    max = cmp::max(max, cur);
    println!("{}", max);
}

fn part2() {
    let mut top = [0; 4];
    let mut cur = 0;
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        if line.is_empty() {
            (top[0], cur) = (cur, 0);
            top.sort();
        } else {
            cur += line.parse::<usize>().unwrap();
        }
    }
    top[0] = cur;
    top.sort();
    println!("{}", top.iter().skip(1).sum::<usize>());
}

fn main() {
    let n = env::args().nth(1);
    match n.as_deref() {
        Some("1") => part1(),
        Some("2") => part2(),
        x => panic!("invalid argument: {:?}", x),
    }
}
