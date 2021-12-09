use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = io::BufReader::new(stdin.lock()).lines();
    let mut prev: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut count = 0u32;
    for line in lines {
        let n = line.unwrap().parse().unwrap();
        if prev < n {
            count += 1;
        }
        prev = n;
    }
    println!("{}", count);
}
