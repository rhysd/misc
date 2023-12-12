use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}
impl Spring {
    fn new(c: char) -> Self {
        match c {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => unreachable!(),
        }
    }
}

fn parse(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let mut s = line.split_whitespace();
    let springs = s.next().unwrap().chars().map(Spring::new).collect();
    let sizes = s
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    (springs, sizes)
}

enum Eaten<'a> {
    Matched,
    Unmatched,
    Unknown(&'a mut [Spring], &'a [usize], usize),
}

fn eat<'a>(springs: &'a mut [Spring], mut sizes: &'a [usize]) -> Eaten<'a> {
    let mut size = 0;
    let mut last_damaged_end = 0;
    for (i, spring) in springs.iter().copied().enumerate() {
        match spring {
            Spring::Operational if size > 0 => match sizes.split_first() {
                Some((&h, t)) if size == h => {
                    last_damaged_end = i;
                    sizes = t;
                    size = 0;
                }
                _ => return Eaten::Unmatched,
            },
            Spring::Operational => last_damaged_end += 1,
            Spring::Damaged => {
                size += 1;
                if let Some(&s) = sizes.first() {
                    if size > s {
                        return Eaten::Unmatched;
                    }
                }
            }
            Spring::Unknown => {
                let springs = &mut springs[last_damaged_end..];
                return Eaten::Unknown(springs, sizes, i - last_damaged_end);
            }
        }
    }

    if size > 0 {
        if sizes.len() == 1 && sizes[0] == size {
            Eaten::Matched
        } else {
            Eaten::Unmatched
        }
    } else if sizes.is_empty() {
        Eaten::Matched
    } else {
        Eaten::Unmatched
    }
}

// Encode 4 springs per byte. It made this program 1.96x faster
fn encode(s: &[Spring]) -> Vec<u8> {
    s.chunks(4)
        .map(|ss| {
            ss.iter().fold(0u8, |enc, s| {
                (enc << 2)
                    | match s {
                        Spring::Operational => 0b01,
                        Spring::Damaged => 0b10,
                        Spring::Unknown => 0b11,
                    }
            })
        })
        .collect()
}

fn arranges(
    springs: &mut [Spring],
    sizes: &[usize],
    memo: &mut HashMap<(Vec<u8>, usize), usize>,
) -> usize {
    let state = (encode(springs), sizes.len());
    if let Some(&memo) = memo.get(&state) {
        return memo;
    }
    let total = match eat(springs, sizes) {
        Eaten::Matched => 1,
        Eaten::Unmatched => 0,
        Eaten::Unknown(springs, sizes, i) => {
            springs[i] = Spring::Operational;
            let on_operational = arranges(springs, sizes, memo);
            springs[i] = Spring::Damaged;
            let on_damaged = arranges(springs, sizes, memo);
            springs[i] = Spring::Unknown;
            on_operational + on_damaged
        }
    };
    memo.insert(state, total);
    total
}

fn part1(lines: impl Iterator<Item = String>) {
    let total: usize = lines
        .map(|line| {
            let (mut springs, sizes) = parse(&line);
            arranges(&mut springs, &sizes, &mut HashMap::new())
        })
        .sum();
    println!("{total}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let total: usize = lines
        .map(|line| {
            let (folded_springs, folded_sizes) = parse(&line);

            let mut springs = folded_springs.clone();
            let mut sizes = folded_sizes.clone();
            for _ in 0..4 {
                springs.push(Spring::Unknown);
                springs.extend_from_slice(&folded_springs);
                sizes.extend_from_slice(&folded_sizes);
            }

            arranges(&mut springs, &sizes, &mut HashMap::new())
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
