use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let start = lines.next().unwrap().as_bytes().iter().position(|&b| b == b'S').unwrap();
    let mut count = 0;
    let mut tachyons = HashSet::from([start]);
    for line in lines {
        for (i, b) in line.into_bytes().into_iter().enumerate() {
            if b == b'^' && tachyons.remove(&i) {
                tachyons.insert(i - 1);
                tachyons.insert(i + 1);
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn paths_at(
    time: u32,
    pos: usize,
    splits: &[Vec<bool>],
    // Note: In this case using Vec<Vec<u64>> consuming more memory is 1.17x faster than using HashMap
    cache: &mut HashMap<(u32, usize), u64>,
) -> u64 {
    let key = (time, pos);
    if let Some(&ret) = cache.get(&key) {
        return ret;
    }
    let t = time + 1;
    let ret = match splits.split_first() {
        Some((s, ss)) if s[pos] => {
            paths_at(t, pos - 1, ss, cache) + paths_at(t, pos + 1, ss, cache)
        }
        Some((_, ss)) => paths_at(t, pos, ss, cache),
        None => 1,
    };
    cache.insert(key, ret);
    ret
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let start = lines.next().unwrap().as_bytes().iter().position(|&b| b == b'S').unwrap();
    let splits: Vec<Vec<_>> =
        lines.map(|l| l.into_bytes().into_iter().map(|b| b == b'^').collect()).collect();
    println!("{}", paths_at(0, start, &splits, &mut HashMap::new()));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
