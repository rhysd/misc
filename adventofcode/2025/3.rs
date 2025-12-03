use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let mut total = 0;
    for line in lines {
        let v: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let (i, &l) = v[..v.len() - 1]
            .iter()
            .enumerate()
            .max_by(|(i, l), (j, r)| l.cmp(r).then(j.cmp(i)))
            .unwrap();
        let &r = v[i + 1..].iter().max().unwrap();
        total += l * 10 + r;
    }
    println!("{total}");
}

// Note: Returning zero means no solution found
fn max_joltage(rem: u8, digits: &[u64], cache: &mut HashMap<(u8, u8), u64>) -> u64 {
    let key = (rem, digits.len() as u8); // About 1.65x faster by using u8 as key instead of usize made
    if let Some(ret) = cache.get(&key) {
        return *ret;
    }
    if rem == 1 {
        return digits.iter().copied().max().unwrap_or(0);
    }
    let mut ret = 0;
    let mut max = 0;
    for (i, &l) in digits.iter().enumerate() {
        if l < max {
            continue; // About 3.81x faster by this pruning
        }
        let r = max_joltage(rem - 1, &digits[i + 1..], cache);
        if r > 0 {
            ret = ret.max(l * 10u64.pow(rem as u32 - 1) + r);
            max = l;
        }
    }
    cache.insert(key, ret);
    ret
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut total = 0;
    for line in lines {
        let digits: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        assert!(digits.len() < u8::MAX as usize);
        total += max_joltage(12, &digits, &mut HashMap::new());
    }
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
