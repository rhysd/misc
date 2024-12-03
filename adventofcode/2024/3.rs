use regex::{Captures, Regex};
use std::env;
use std::io::{self, BufRead};

fn eval(capture: Captures<'_>) -> Option<i32> {
    let left = capture.get(1)?.as_str().parse::<i32>().ok()?;
    let right = capture.get(2)?.as_str().parse::<i32>().ok()?;
    Some(left * right)
}

fn part1(lines: impl Iterator<Item = String>) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total = lines.map(|line| re.captures_iter(&line).flat_map(eval).sum::<i32>()).sum::<i32>();
    println!("{total}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let total: i32 = lines
        .map(|line| {
            re.captures_iter(&line)
                .flat_map(|cap| match &cap[0] {
                    "do()" => {
                        enabled = true;
                        None
                    }
                    "don't()" => {
                        enabled = false;
                        None
                    }
                    _ if enabled => eval(cap),
                    _ => None,
                })
                .sum::<i32>()
        })
        .sum();
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
