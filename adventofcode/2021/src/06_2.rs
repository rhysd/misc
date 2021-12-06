use std::io::{self, BufRead};

fn main() {
    let mut fish = [0usize; 9];

    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    for num in line.split(',').map(|s| s.trim().parse::<usize>().unwrap()) {
        fish[num] += 1;
    }

    for _ in 0..256 {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    println!("{}", fish.iter().sum::<usize>());
}
