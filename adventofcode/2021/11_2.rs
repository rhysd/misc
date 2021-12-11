use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let mut octs: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    for step in 0.. {
        if octs.iter().all(|row| row.iter().all(|oct| *oct == 0)) {
            println!("{}", step);
            break;
        }

        let mut deq = VecDeque::new();
        for (i, row) in octs.iter_mut().enumerate() {
            for (j, oct) in row.iter_mut().enumerate() {
                *oct += 1;
                if *oct == 10 {
                    deq.push_back((i, j));
                }
            }
        }

        while !deq.is_empty() {
            let (i, j) = deq.pop_front().unwrap();
            let (il, ir) = (i > 0, i + 1 < octs.len());
            let (jl, jr) = (j > 0, j + 1 < octs[i].len());
            for (x, y) in [
                il.then(|| (i - 1, j)),
                ir.then(|| (i + 1, j)),
                jl.then(|| (i, j - 1)),
                jr.then(|| (i, j + 1)),
                (il && jl).then(|| (i - 1, j - 1)),
                (il && jr).then(|| (i - 1, j + 1)),
                (ir && jl).then(|| (i + 1, j - 1)),
                (ir && jr).then(|| (i + 1, j + 1)),
            ]
            .into_iter()
            .flatten()
            {
                octs[x][y] += 1;
                if octs[x][y] == 10 {
                    deq.push_back((x, y));
                }
            }
        }

        for row in octs.iter_mut() {
            for oct in row.iter_mut() {
                if *oct >= 10 {
                    *oct = 0;
                }
            }
        }
    }
}
