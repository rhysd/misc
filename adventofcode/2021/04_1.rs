use std::io::{self, BufRead};

#[derive(Debug)]
struct Board(Vec<Vec<(usize, bool)>>);

impl Board {
    fn mark(&mut self, num: usize) {
        for row in self.0.iter_mut() {
            for (n, b) in row.iter_mut() {
                if *n == num {
                    *b = true;
                }
            }
        }
    }

    fn wins(&self) -> bool {
        for row in self.0.iter() {
            if row.iter().all(|(_, b)| *b) {
                return true;
            }
        }
        for i in 0..self.0[0].len() {
            if self.0.iter().all(|row| row[i].1) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        self.0
            .iter()
            .map(|r| r.iter().filter(|(_, b)| !*b).map(|(x, _)| *x).sum::<usize>())
            .sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let nums: Vec<usize> = lines.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    lines.next();

    let mut boards = vec![];
    'outer: loop {
        let mut rows = vec![];
        loop {
            match lines.next() {
                None => {
                    boards.push(Board(rows));
                    break 'outer;
                }
                Some(l) if l.is_empty() => {
                    boards.push(Board(rows));
                    break;
                }
                Some(l) => {
                    let row = l
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| (s.parse().unwrap(), false))
                        .collect();
                    rows.push(row);
                }
            }
        }
    }

    for num in nums {
        for board in boards.iter_mut() {
            board.mark(num);
        }

        for board in boards.iter() {
            if board.wins() {
                println!("{}", board.score() * num);
                return;
            }
        }
    }
}
