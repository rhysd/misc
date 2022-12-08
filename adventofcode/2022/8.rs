use std::cmp;
use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let vv: Vec<Vec<u8>> = lines
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut count = 0;
    for y in 1..vv.len() - 1 {
        let v = &vv[y];
        for x in 1..v.len() - 1 {
            let height = v[x];
            if v[..x].iter().all(|&h| h < height)
                || v[x + 1..].iter().all(|&h| h < height)
                || vv[..y].iter().all(|v| v[x] < height)
                || vv[y + 1..].iter().all(|v| v[x] < height)
            {
                count += 1;
            }
        }
    }
    count += vv[0].len() * 2 + (vv.len() - 2) * 2;

    println!("{:?}", count);
}

fn part2(lines: impl Iterator<Item = String>) {
    let vv: Vec<Vec<_>> = lines
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let (xend, yend) = (vv[0].len() - 1, vv.len() - 1);
    let mut max = 0;
    for y in 1..yend {
        let v = &vv[y];
        for x in 1..xend {
            let height = v[x];
            let left = v[1..x].iter().rev().take_while(|&&h| h < height).count() + 1;
            let right = v[x + 1..xend].iter().take_while(|&&h| h < height).count() + 1;
            let top = vv[1..y].iter().rev().take_while(|v| v[x] < height).count() + 1;
            let bottom = vv[y + 1..yend].iter().take_while(|v| v[x] < height).count() + 1;
            max = cmp::max(max, left * right * top * bottom);
        }
    }

    println!("{:?}", max);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
