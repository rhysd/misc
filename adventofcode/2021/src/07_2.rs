use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let crabs: Vec<isize> = line.split(',').map(|s| s.parse().unwrap()).collect();

    let max_pos = *crabs.iter().max().unwrap();
    let min: isize = (0..=max_pos)
        .map(|crab| crabs.iter().map(|c| (1..=(c - crab).abs()).sum::<isize>()).sum())
        .min()
        .unwrap();

    println!("{}", min);
}
