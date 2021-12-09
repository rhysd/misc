use std::io::{self, BufRead};

fn main() {
    let board: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut total = 0;
    for y in 0..board.len() {
        let row = &board[y];
        for x in 0..row.len() {
            let v = row[x];
            if x > 0 && row[x - 1] <= v {
                continue;
            }
            if x + 1 < row.len() && row[x + 1] <= v {
                continue;
            }
            if y > 0 && board[y - 1][x] <= v {
                continue;
            }
            if y + 1 < board.len() && board[y + 1][x] <= v {
                continue;
            }
            total += v + 1;
        }
    }
    println!("{}", total);
}
