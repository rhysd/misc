use std::env;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let mut dial = 50;
    let mut count = 0;
    for line in lines {
        let mut cs = line.chars();
        let (c, i) = (cs.next().unwrap(), cs.as_str().parse::<i32>().unwrap());
        dial = match c {
            'L' => dial - i,
            'R' => dial + i,
            _ => unreachable!(),
        };
        if dial % 100 == 0 {
            count += 1;
        }
    }
    println!("{count}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut dial = 50;
    let mut count = 0;
    let mut prev_dir = 'R';
    for line in lines {
        let mut cs = line.chars();
        let (dir, clicks) = (cs.next().unwrap(), cs.as_str().parse::<i32>().unwrap());

        // Flip the dial when direction flips so that we only consider turning right
        if dir != prev_dir {
            dial = (100 - dial) % 100;
            prev_dir = dir
        }

        let next = dial + clicks;
        count += next / 100;
        dial = next % 100;
    }
    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
