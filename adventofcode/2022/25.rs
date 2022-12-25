use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let mut sum: i64 = lines
        .map(|l| {
            l.into_bytes()
                .into_iter()
                .rev()
                .enumerate()
                .map(|(d, b)| {
                    let n = match b {
                        b'2' => 2,
                        b'1' => 1,
                        b'0' => 0,
                        b'-' => -1,
                        b'=' => -2,
                        _ => panic!("invalid SNAFU digit {:?}", b as char),
                    };
                    5i64.pow(d as u32) * n
                })
                .sum::<i64>()
        })
        .sum();

    let mut snafu = vec![];
    while sum != 0 {
        let c = match (sum + 2) % 5 - 2 {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        };
        snafu.push(c);
        sum = (sum + 2) / 5;
    }
    snafu.reverse();

    for c in snafu {
        print!("{}", c);
    }
    println!();
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
