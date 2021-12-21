use std::io::{self, BufRead};

fn main() {
    let mut players: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let i = l.find("starting position: ").unwrap() + "starting position: ".len();
            let pos = l[i..].parse::<usize>().unwrap() - 1;
            (pos, 0)
        })
        .collect();

    let mut dice = 1;
    let mut rolls = 0;
    'game: loop {
        for (pos, score) in players.iter_mut() {
            for _ in 0..3 {
                *pos += dice;
                dice = if dice == 100 { 1 } else { dice + 1 };
            }
            *pos %= 10;
            *score += *pos + 1;
            rolls += 3;
            if *score >= 1000 {
                break 'game;
            }
        }
    }

    let min = players.iter().map(|p| p.1).min().unwrap();
    println!("{}", min * rolls);
}
