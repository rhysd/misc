use serde::Deserialize;
use std::cmp::Ordering;
use std::env;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Term(u8),
    Nest(Vec<Packet>),
}
impl PartialEq for Packet {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
impl Eq for Packet {}
impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Packet {
    fn cmp(&self, rhs: &Self) -> Ordering {
        use Packet::*;
        match (self, rhs) {
            (Term(l), Term(r)) => l.cmp(r),
            (Nest(l), Nest(r)) => l.cmp(r),
            (Term(l), Nest(r)) => [Term(*l)][..].cmp(r),
            (Nest(l), Term(r)) => l.as_slice().cmp(&[Term(*r)]),
        }
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut packets = lines
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Packet>(&l).unwrap());
    let mut sum = 0;
    for i in 1.. {
        let Some(left) = packets.next() else { break; };
        let right = packets.next().unwrap();
        if left < right {
            sum += i;
        }
    }
    println!("{:?}", sum);
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut all: Vec<Packet> = lines
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(&l).unwrap())
        .collect();
    let div1: Packet = serde_json::from_str("[[2]]").unwrap();
    let div2: Packet = serde_json::from_str("[[6]]").unwrap();
    all.push(div1.clone());
    all.push(div2.clone());
    all.sort_unstable();
    let idx1 = all.iter().position(|p| p == &div1).unwrap() + 1;
    let idx2 = all.iter().position(|p| p == &div2).unwrap() + 1;
    println!("{}", idx1 * idx2);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
