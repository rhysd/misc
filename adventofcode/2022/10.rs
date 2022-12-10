use std::env;
use std::fmt;
use std::io::{self, BufRead};

fn part1(lines: impl Iterator<Item = String>) {
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut next_signal: i32 = 20;
    let mut signal: i32 = 0;
    for line in lines {
        let mut s = line.split(' ');
        let (elapsed, delta) = match s.next().unwrap() {
            "noop" => (1, 0),
            "addx" => {
                let v = s.next().unwrap().parse().unwrap();
                (2, v)
            }
            _ => panic!(),
        };
        cycle += elapsed;
        if cycle >= next_signal {
            signal += next_signal * x;
            next_signal += 40;
        }
        x += delta;
    }
    println!("{}", signal);
}

#[derive(Clone, Copy)]
enum Insn {
    Noop,
    AddX(i32),
}

impl Insn {
    fn cycles(self) -> usize {
        match self {
            Insn::Noop => 1,
            Insn::AddX(_) => 2,
        }
    }
}

struct Decoder<I: Iterator<Item = String>> {
    lines: I,
}

impl<I: Iterator<Item = String>> Iterator for Decoder<I> {
    type Item = Insn;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let mut s = line.split(' ');
        match s.next().unwrap() {
            "noop" => Some(Insn::Noop),
            "addx" => Some(Insn::AddX(s.next().unwrap().parse().unwrap())),
            line => panic!("decode error: {:?}", line),
        }
    }
}

struct Crt([bool; 40 * 6]);

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..6 {
            if y > 0 {
                writeln!(f)?;
            }
            for x in 0..40 {
                let c = if self.0[y * 40 + x] { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

impl Default for Crt {
    fn default() -> Self {
        Self([false; 40 * 6]) // Default is not implemented for [bool; 240] since array is too large
    }
}

impl Crt {
    fn draw(&mut self, cycle: usize, pos: i32) {
        let col = cycle % 40;
        let row = (cycle / 40) % 6;
        self.0[row * 40 + col] = (pos - 1..=pos + 1).contains(&(col as i32));
    }
}

struct Cpu<I> {
    dec: I,
    cycle: usize,
    timing: usize,
    insn: Insn,
    reg: i32,
    crt: Crt,
}

impl<I: Iterator<Item = Insn>> Cpu<I> {
    fn new(dec: I) -> Self {
        Cpu {
            dec,
            cycle: 0,
            timing: 1,
            insn: Insn::Noop,
            reg: 1,
            crt: Crt::default(),
        }
    }

    fn tick(&mut self) -> bool {
        self.timing -= 1;
        if self.timing == 0 {
            match self.insn {
                Insn::Noop => {}
                Insn::AddX(v) => self.reg += v,
            }
            if let Some(insn) = self.dec.next() {
                self.timing = insn.cycles();
                self.insn = insn;
            } else {
                return false;
            }
        }

        self.crt.draw(self.cycle, self.reg);
        self.cycle += 1;
        true
    }
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut cpu = Cpu::new(Decoder { lines });
    while cpu.tick() {}
    println!("{}", cpu.crt);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
