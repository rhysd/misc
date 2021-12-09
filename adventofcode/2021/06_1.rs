use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut fish: Vec<usize> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
    for _ in 0..80 {
        for i in 0..fish.len() {
            let f = fish[i];
            if f == 0 {
                fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }
    }
    println!("{}", fish.len());
}
