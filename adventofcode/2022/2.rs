use std::env;
use std::io::{self, BufRead};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Game {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Game {
    fn new(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            x => unreachable!("{:?}", x),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

impl Move {
    fn new(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scisors,
            x => unreachable!("{:?}", x),
        }
    }

    fn result(self, opp: Move) -> Game {
        use Move::*;
        match (self, opp) {
            (Rock, Scisors) | (Paper, Rock) | (Scisors, Paper) => Game::Win,
            (Rock, Rock) | (Paper, Paper) | (Scisors, Scisors) => Game::Draw,
            _ => Game::Lose,
        }
    }

    fn score(self, opp: Move) -> u8 {
        self as u8 + self.result(opp) as u8
    }
}

fn part1() {
    let total: usize = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut s = line.split(' ');
            let you = Move::new(s.next().unwrap());
            let me = Move::new(s.next().unwrap());
            me.score(you) as usize
        })
        .sum();
    println!("{}", total);
}

fn part2() {
    let total: usize = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            use Move::*;
            let mut s = line.split(' ');
            let you = Move::new(s.next().unwrap());
            let result = Game::new(s.next().unwrap());
            let me = [Rock, Paper, Scisors]
                .into_iter()
                .find(|m| m.result(you) == result)
                .unwrap();
            me as usize + result as usize
        })
        .sum();
    println!("{}", total);
}

fn main() {
    let n = env::args().nth(1);
    match n.as_deref() {
        Some("1") => part1(),
        Some("2") => part2(),
        x => panic!("invalid argument: {:?}", x),
    }
}
