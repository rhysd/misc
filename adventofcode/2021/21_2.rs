use std::cmp;
use std::io::{self, BufRead};

const WIN: u8 = 21;

#[derive(Debug, Clone, Copy)]
struct Player {
    pos: u8,
    score: u8,
}

impl Player {
    fn parse(l: String) -> Self {
        let i = l.find("starting position: ").unwrap();
        let pos = l[i + "starting position: ".len()..].parse::<u8>().unwrap() - 1;
        Self { pos, score: 0 }
    }

    fn roll(&self, v: u8) -> Self {
        let pos = (self.pos + v) % 10;
        let score = self.score + pos + 1;
        Self { pos, score }
    }
}

fn play(players: [Player; 2], player1_turn: bool) -> (u64, u64) {
    // 1 2 3
    // 2 3 4 3 4 5 4 5 6
    // 3 4 5 4 5 6 5 6 7 4 5 6 5 6 7 6 7 8 5 6 7 6 7 8 7 8 9
    const DISTRIBUTION: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut sum = (0, 0);
    if player1_turn {
        for (dice, appears) in DISTRIBUTION {
            let p = players[0].roll(dice);
            if p.score >= WIN {
                sum.0 += appears;
            } else {
                let (p1, p2) = play([p, players[1]], !player1_turn);
                sum.0 += p1 * appears;
                sum.1 += p2 * appears;
            }
        }
    } else {
        for (dice, appears) in DISTRIBUTION {
            let p = players[1].roll(dice);
            if p.score >= WIN {
                sum.1 += appears;
            } else {
                let (p1, p2) = play([players[0], p], !player1_turn);
                sum.0 += p1 * appears;
                sum.1 += p2 * appears;
            }
        }
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let players = [
        Player::parse(lines.next().unwrap()),
        Player::parse(lines.next().unwrap()),
    ];
    let (player1_wins, player2_wins) = play(players, true);
    println!("{}", cmp::max(player1_wins, player2_wins));
}
