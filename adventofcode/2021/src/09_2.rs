use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let b: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut lows = vec![];
    for y in 0..b.len() {
        let row = &b[y];
        for x in 0..row.len() {
            let v = row[x];
            if x > 0 && row[x - 1] <= v {
                continue;
            }
            if x + 1 < row.len() && row[x + 1] <= v {
                continue;
            }
            if y > 0 && b[y - 1][x] <= v {
                continue;
            }
            if y + 1 < b.len() && b[y + 1][x] <= v {
                continue;
            }
            if v >= 9 {
                continue;
            }
            lows.push((x, y));
        }
    }

    let mut basins: Vec<_> = lows
        .into_iter()
        .map(|low| {
            let mut seen = HashSet::new();
            let mut size = 0;
            let mut deq = VecDeque::new();
            deq.push_back(low);
            while !deq.is_empty() {
                let (x, y) = deq.pop_front().unwrap();
                let v = b[y][x];
                for p in [
                    (x > 0).then(|| (x - 1, y)),
                    (x + 1 < b[y].len()).then(|| (x + 1, y)),
                    (y > 0).then(|| (x, y - 1)),
                    (y + 1 < b.len()).then(|| (x, y + 1)),
                ]
                .into_iter()
                .flatten()
                {
                    if (v..9).contains(&b[p.1][p.0]) && !seen.contains(&p) {
                        seen.insert(p);
                        deq.push_back(p);
                    }
                }
                size += 1;
            }
            size
        })
        .collect();

    basins.sort_unstable();
    let score: usize = basins.into_iter().rev().take(3).product();

    println!("{}", score);
}
