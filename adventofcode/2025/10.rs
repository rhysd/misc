use std::collections::{HashSet, VecDeque};
use std::env;
use std::io::{self, BufRead};
use z3::ast::Int;
use z3::{Optimize, SatResult};

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

impl Machine {
    fn parse(s: &str) -> Self {
        let s = s.strip_prefix('[').unwrap();
        let mut s = s.split("] (");
        // .rev() because the indices in (...) are left-to-right
        let lights =
            s.next().unwrap().chars().rev().fold(0, |acc, c| (acc << 1) | (c == '#') as usize);
        let mut s = s.next().unwrap().split(") {");
        let buttons = s
            .next()
            .unwrap()
            .split(") (")
            .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();
        let s = s.next().unwrap().strip_suffix('}').unwrap();
        let joltage = s.split(',').map(|s| s.parse().unwrap()).collect();
        Self { lights, buttons, joltage }
    }
}

fn min_presses_for_lights(m: &Machine) -> u32 {
    let mut seen = HashSet::new();
    let mut q = VecDeque::from([(0, 0)]);
    while let Some((presses, lights)) = q.pop_front() {
        if seen.contains(&lights) {
            continue;
        }

        if lights == m.lights {
            return presses;
        }

        for conn in &m.buttons {
            let mut l = lights;
            for &i in conn {
                l ^= 1 << i;
            }
            q.push_back((presses + 1, l));
        }

        seen.insert(lights);
    }
    unreachable!("not found")
}

fn part1(lines: impl Iterator<Item = String>) {
    let total: u32 = lines.map(|l| min_presses_for_lights(&Machine::parse(&l))).sum();
    println!("{total}");
}

fn min_presses_for_joltage(m: &Machine) -> u64 {
    let solver = Optimize::new();
    let zero = Int::from_u64(0);

    let mut eq = vec![zero.clone(); m.joltage.len()];
    let mut presses = zero.clone();
    for (i, conn) in m.buttons.iter().enumerate() {
        let count = Int::new_const(format!("count{i}"));
        solver.assert(&count.ge(&zero));
        for &j in conn {
            eq[j] += &count;
        }
        presses += count;
    }
    for (i, &jolt) in m.joltage.iter().enumerate() {
        solver.assert(&eq[i].eq(Int::from_u64(jolt)));
    }

    solver.minimize(&presses);

    assert_eq!(solver.check(&[]), SatResult::Sat);
    let model = solver.get_model().unwrap();
    model.eval(&presses, true).unwrap().as_u64().unwrap()
}

fn part2(lines: impl Iterator<Item = String>) {
    let total: u64 = lines.map(|l| min_presses_for_joltage(&Machine::parse(&l))).sum();
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
