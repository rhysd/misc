use std::io::{self, BufRead};

enum Kind {
    Forward,
    Down,
    Up,
}

#[derive(Default)]
struct State {
    pos: usize,
    depth: usize,
}

struct Command(Kind, usize);

impl Command {
    fn parse(input: &str) -> Self {
        let (input, kind) = if let Some(i) = input.strip_prefix("forward ") {
            (i, Kind::Forward)
        } else if let Some(i) = input.strip_prefix("down ") {
            (i, Kind::Down)
        } else if let Some(i) = input.strip_prefix("up ") {
            (i, Kind::Up)
        } else {
            panic!("invalid line {:?}", input);
        };
        Self(kind, input.parse().unwrap())
    }

    fn run(&self, state: &mut State) {
        match self.0 {
            Kind::Forward => state.pos += self.1,
            Kind::Down => state.depth += self.1,
            Kind::Up => state.depth -= self.1,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut state = State::default();
    for line in stdin.lock().lines() {
        let cmd = Command::parse(&line.unwrap());
        cmd.run(&mut state);
    }
    println!("{}", state.pos * state.depth);
}
