use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Type {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Type {
    fn with_label_counts(labels: HashMap<char, u8>) -> Self {
        match labels.len() {
            1 => Self::FiveOfKind,
            2 => match labels.values().next().unwrap() {
                4 | 1 => Self::FourOfKind,
                2 | 3 => Self::FullHouse,
                _ => unreachable!(),
            },
            3 if labels.values().any(|v| *v == 3) => Self::ThreeOfAKind,
            3 if labels.values().filter(|v| **v == 2).count() == 2 => Self::TwoPair,
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }

    fn new(s: &str) -> Self {
        let mut labels = HashMap::new();
        for c in s.chars() {
            *labels.entry(c).or_default() += 1;
        }
        Self::with_label_counts(labels)
    }

    fn new_jokers(s: &str) -> Self {
        let mut labels = HashMap::new();
        let mut jokers = 0;
        for c in s.chars() {
            if c == 'J' {
                jokers += 1;
            } else {
                *labels.entry(c).or_default() += 1;
            }
        }
        if jokers > 0 {
            if let Some(max) = labels.values_mut().max() {
                *max += jokers;
            } else {
                return Self::FiveOfKind;
            }
        }
        Self::with_label_counts(labels)
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    ty: Type,
    bid: usize,
}

impl Hand {
    fn new(line: &str, f: fn(&str) -> Type) -> Self {
        let mut s = line.split_whitespace();
        let cards = s.next().unwrap();
        let bid = s.next().unwrap().parse().unwrap();
        let ty = f(cards);
        Self {
            cards: cards.to_string(),
            ty,
            bid,
        }
    }

    fn cmp_with_rank(&self, other: &Self, f: fn(c: char) -> u32) -> Ordering {
        (self.ty as u8)
            .cmp(&(other.ty as u8))
            .then_with(|| self.cards.chars().map(f).cmp(other.cards.chars().map(f)))
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    fn rank(c: char) -> u32 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap(),
            _ => unreachable!(),
        }
    }

    let mut hands: Vec<_> = lines.map(|line| Hand::new(&line, Type::new)).collect();
    hands.sort_by(|l, r| l.cmp_with_rank(r, rank));
    let sum: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();
    println!("{sum}");
}

fn part2(lines: impl Iterator<Item = String>) {
    fn rank(c: char) -> u32 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap(),
            _ => unreachable!(),
        }
    }

    let mut hands: Vec<_> = lines
        .map(|line| Hand::new(&line, Type::new_jokers))
        .collect();
    hands.sort_by(|l, r| l.cmp_with_rank(r, rank));
    let sum: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();
    println!("{sum}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
