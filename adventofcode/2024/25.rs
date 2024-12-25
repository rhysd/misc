use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut locks: Vec<Vec<u8>> = vec![];
    let mut keys: Vec<Vec<u8>> = vec![];

    let height = loop {
        let mut schema = vec![];
        let mut has_next = false;
        for line in &mut lines {
            if line.is_empty() {
                has_next = true;
                break;
            }
            schema.push(line.into_bytes());
        }

        if schema[0].iter().all(|&c| c == b'#') {
            let lock = (0..schema[0].len())
                .map(|x| (1..schema.len()).take_while(|&y| schema[y][x] == b'#').count() as u8)
                .collect();
            locks.push(lock);
        } else {
            let key = (0..schema[0].len())
                .map(|x| {
                    (0..schema.len() - 1).rev().take_while(|&y| schema[y][x] == b'#').count() as u8
                })
                .collect();
            keys.push(key);
        }

        if !has_next {
            break schema.len() as u8;
        }
    };

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k <= height - 2) {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
