use std::env;
use std::io;

fn part1(line: &str) {
    let mut total = 0u64;
    for range in line.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let sub_start = if start.len() % 2 == 0 {
            start[..start.len() / 2].parse().unwrap()
        } else {
            10u64.pow(start.len() as u32 / 2)
        };
        let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

        for sub in sub_start.. {
            let id = sub * 10u64.pow(sub.ilog10() + 1) + sub;
            if id >= start && id <= end {
                total += id;
            } else if end < id {
                break;
            }
        }
    }
    println!("{total}");
}

fn is_invalid(id: u64) -> bool {
    'outer: for digits in 1..=id.ilog10().div_ceil(2) {
        let div = 10u64.pow(digits);
        let sub = id % div;
        let mut u = id / div;
        while u > 0 {
            // For example, 101 is not invalid though it's 0101
            if u % div != sub || u.ilog10() + 1 < digits {
                continue 'outer;
            }
            u /= div;
        }
        return true;
    }
    false
}

// Note: Iterating candidates and checking if they are in the input ranges would be a better idea because
// the number of candidates seems much smaller than the number of integers within the input ranges.
fn part2(line: &str) {
    let mut total = 0;
    for range in line.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
        total += (start..=end).filter(|&i| is_invalid(i)).sum::<u64>();
    }
    println!("{total}");
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("1") => part1(&io::stdin().lines().next().unwrap().unwrap()),
        Some("2") => part2(&io::stdin().lines().next().unwrap().unwrap()),
        x => panic!("invalid argument: {x:?}"),
    }
}
