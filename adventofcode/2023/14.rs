use std::env;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Rock {
    Round = 0,
    Cube = 1,
    None = 2,
}
impl Rock {
    fn new(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::None,
            _ => unreachable!(),
        }
    }
}

type System = Vec<Vec<Rock>>;

fn parse(lines: impl Iterator<Item = String>) -> System {
    let mut system: System = vec![];
    for line in lines {
        for (i, r) in line.chars().map(Rock::new).enumerate() {
            if let Some(v) = system.get_mut(i) {
                v.push(r);
            } else {
                system.push(vec![r]);
            }
        }
    }
    assert_eq!(system.len(), system[0].len()); // Ensure the shape of the system is square
    system
}

fn tilt(system: &mut System) {
    for line in system.iter_mut() {
        for rocks in line.split_mut(|&r| r == Rock::Cube) {
            rocks.sort_by_key(|r| *r as u8);
        }
    }
}

fn total_load(system: &System) -> usize {
    system
        .iter()
        .map(|line| {
            let len = line.len();
            line.iter()
                .enumerate()
                .flat_map(|(i, &r)| (r == Rock::Round).then_some(len - i))
                .sum::<usize>()
        })
        .sum()
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut system = parse(lines);
    tilt(&mut system);
    println!("{}", total_load(&system));
}

// Rotate by 90 degree in right direction
fn rotate(from: &mut System, to: &mut System) {
    let len = from.len();
    for x in 0..len {
        for y in 0..len {
            to[len - y - 1][x] = from[x][y];
        }
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut system = parse(lines);
    let mut system = &mut system;
    let mut next = system.clone(); // Buffer for applying rotation
    let mut next = &mut next;

    let mut history = vec![system.clone()];
    loop {
        // Apply 1 cycle
        for _ in 0..4 {
            tilt(system);
            rotate(system, next);
            (next, system) = (system, next);
        }

        // Find the start index of repetition
        if let Some(i) = history.iter().position(|h| h == system) {
            // Calculate the index of system after 1000000000th cycles considering the repetition
            let i = (1000000000 - i) % (history.len() - i) + i;
            println!("{}", total_load(&history[i]));
            return;
        }

        history.push(system.clone());
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
