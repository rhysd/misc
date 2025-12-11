use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

fn parse(lines: &[String]) -> HashMap<&str, Vec<&str>> {
    lines
        .iter()
        .map(|line| {
            let mut s = line.split(": ");
            let from = s.next().unwrap();
            let to = s.next().unwrap().split_whitespace().collect();
            (from, to)
        })
        .collect()
}

fn paths<'line>(
    conns: &HashMap<&'line str, Vec<&'line str>>,
    from: &'line str,
    to: &'line str,
    cache: &mut HashMap<(&'line str, &'line str), u64>,
) -> u64 {
    if from == to {
        return 1;
    }
    if let Some(count) = cache.get(&(from, to)) {
        return *count;
    }
    let Some(next) = conns.get(&from) else {
        return 0;
    };

    let count = next.iter().map(|n| paths(conns, n, to, cache)).sum();
    cache.insert((from, to), count);
    count
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    println!("{}", paths(&parse(&lines), "you", "out", &mut HashMap::new()));
}

fn part2(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let conns = parse(&lines);
    let mut cache = HashMap::new();
    let a = paths(&conns, "svr", "fft", &mut cache);
    let b = paths(&conns, "fft", "dac", &mut cache);
    let c = paths(&conns, "dac", "out", &mut cache);
    let d = paths(&conns, "svr", "dac", &mut cache);
    let e = paths(&conns, "dac", "fft", &mut cache);
    let f = paths(&conns, "fft", "out", &mut cache);
    println!("{}", a * b * c + d * e * f);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
