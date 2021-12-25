use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum C {
    E,
    S,
    Empty,
}

impl C {
    fn new(c: char) -> Self {
        match c {
            '>' => Self::E,
            'v' => Self::S,
            '.' => Self::Empty,
            c => panic!("invalid char {:?}", c),
        }
    }
}

impl Default for C {
    fn default() -> Self {
        Self::Empty
    }
}

fn main() {
    let mut state: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(C::new).collect())
        .collect();

    let height = state.len();
    let width = state[0].len();
    let mut step = 0usize;
    loop {
        step += 1;

        let mut east: Vec<Vec<_>> = state
            .iter()
            .map(|xs| xs.iter().map(|c| if *c == C::E { C::Empty } else { *c }).collect())
            .collect();
        for (y, xs) in state.iter().enumerate() {
            for (x, c) in xs.iter().enumerate().filter(|(_, c)| **c == C::E) {
                let mut next_x = if x + 1 >= width { 0 } else { x + 1 };
                if state[y][next_x] != C::Empty {
                    next_x = x;
                }
                east[y][next_x] = *c;
            }
        }

        let mut south: Vec<Vec<_>> = east
            .iter()
            .map(|xs| xs.iter().map(|c| if *c == C::S { C::Empty } else { *c }).collect())
            .collect();
        for (y, xs) in east.iter().enumerate() {
            for (x, c) in xs.iter().enumerate().filter(|(_, c)| **c == C::S) {
                let mut next_y = if y + 1 >= height { 0 } else { y + 1 };
                if east[next_y][x] != C::Empty {
                    next_y = y;
                }
                south[next_y][x] = *c;
            }
        }

        if state == south {
            break;
        }
        state = south;
    }
    println!("{}", step);
}
