use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let nums: Vec<u32> = io::BufReader::new(stdin.lock())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let mut sums = nums.windows(3).map(|s| s.iter().sum::<u32>());
    let mut prev = sums.next().unwrap();
    let mut count = 0;
    for sum in sums {
        if prev < sum {
            count += 1;
        }
        prev = sum;
    }
    println!("{}", count);
}
