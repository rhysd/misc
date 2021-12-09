use std::io::{self, BufRead};

fn main() {
    let mut ones = [0usize; 12];
    let mut total = 0;
    for l in io::stdin().lock().lines() {
        for (idx, c) in l.unwrap().chars().enumerate() {
            if c == '1' {
                ones[idx] += 1;
            }
        }
        total += 1;
    }
    let mut gamma = 0;
    let threshold = total / 2;
    for count in ones {
        gamma = (gamma << 1) + (count > threshold) as usize;
    }
    let epsilon = !gamma & 0xfff;
    println!("{}", gamma * epsilon);
}
