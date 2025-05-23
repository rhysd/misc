use rayon::prelude::*;
use std::env;
use std::io::{self, BufRead};

fn parse(line: String) -> (usize, Vec<usize>) {
    let mut s = line.split(": ");
    let ans = s.next().unwrap().parse().unwrap();
    let ops = s.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    (ans, ops)
}

fn part1(lines: impl Iterator<Item = String>) {
    fn solve(ans: usize, cur: usize, ops: &[usize]) -> bool {
        let Some((head, tail)) = ops.split_first() else {
            return ans == cur;
        };
        ans >= cur && (solve(ans, cur + head, tail) || solve(ans, cur * head, tail))
    }

    // Note: Using rayon is slower in this case
    let total: usize = lines
        .map(parse)
        .filter_map(|(ans, ops)| solve(ans, ops[0], &ops[1..]).then_some(ans))
        .sum();
    println!("{total}");
}

fn part2(lines: impl Iterator<Item = String>) {
    fn combine(a: usize, b: usize) -> usize {
        a * 10usize.pow(b.ilog10() + 1) + b
    }

    fn solve(ans: usize, cur: usize, tail: &[usize]) -> bool {
        let Some((&head, tail)) = tail.split_first() else {
            return ans == cur;
        };
        ans >= cur
            && (solve(ans, cur + head, tail)
                || solve(ans, cur * head, tail)
                || solve(ans, combine(cur, head), tail))
    }

    let equations = lines.map(parse).collect::<Vec<_>>();
    let total: usize = equations
        .into_par_iter()
        .filter_map(|(ans, ops)| solve(ans, ops[0], &ops[1..]).then_some(ans))
        .sum();
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
