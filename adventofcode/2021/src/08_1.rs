use std::io::{self, BufRead};

fn main() {
    let mut count = 0;
    for l in io::stdin().lock().lines() {
        let l = l.unwrap();
        let s = l.split(" | ").nth(1).unwrap();
        for s in s.split(' ') {
            match s.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => {}
            }
        }
    }
    println!("{}", count);
}
