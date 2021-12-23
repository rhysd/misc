use std::cmp;
use std::collections::HashMap;
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

type State = (u8, u8, u8, u8);
fn state(p: [Player; 2], p1_turn: bool) -> State {
    let (i, j) = (!p1_turn as usize, p1_turn as usize);
    (p[i].pos, p[i].score, p[j].pos, p[j].score)
}

// Note: Using fnv::FnvHashMap, this program can run about 9% faster

fn play(players: [Player; 2], player1_turn: bool, memo: &mut HashMap<State, (u64, u64)>) -> (u64, u64) {
    let state = state(players, player1_turn);
    if let Some(sum) = memo.get(&state) {
        return *sum;
    }

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
                let (p1, p2) = play([p, players[1]], !player1_turn, memo);
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
                let (p1, p2) = play([players[0], p], !player1_turn, memo);
                sum.0 += p1 * appears;
                sum.1 += p2 * appears;
            }
        }
    }
    memo.insert(state, if player1_turn { (sum.1, sum.0) } else { sum });
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let players = [
        Player::parse(lines.next().unwrap()),
        Player::parse(lines.next().unwrap()),
    ];
    let mut memo = HashMap::new();
    let (player1_wins, player2_wins) = play(players, true, &mut memo);
    println!("{}", cmp::max(player1_wins, player2_wins));
}
