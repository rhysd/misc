use std::env;
use std::fmt::Write;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Operand {
    Lit(u64),
    RegA,
    RegB,
    RegC,
    Unused,
}

impl Operand {
    fn combo(u: u8) -> Self {
        match u {
            0..4 => Self::Lit(u as u64),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            _ => unreachable!(),
        }
    }

    fn literal(u: u8) -> Self {
        Self::Lit(u as u64)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn decode(u: u8) -> Self {
        match u {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Insn {
    code: Opcode,
    op: Operand,
}

impl Insn {
    fn decode(c: u8, op: u8) -> Self {
        let code = Opcode::decode(c);

        use Opcode::*;
        let op = match code {
            Adv | Bdv | Cdv | Bst | Out => Operand::combo(op),
            Bxl | Jnz => Operand::literal(op),
            Bxc => Operand::Unused,
        };

        Self { code, op }
    }
}

type Regs = (u64, u64, u64);

struct Program<'a> {
    insns: &'a [Insn],
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    output: Vec<u64>,
}

impl<'a> Program<'a> {
    fn new(insns: &'a [Insn], (a, b, c): Regs) -> Self {
        Self { insns, a, b, c, ip: 0, output: vec![] }
    }

    fn operand(&self, op: Operand) -> u64 {
        match op {
            Operand::Lit(v) => v,
            Operand::RegA => self.a,
            Operand::RegB => self.b,
            Operand::RegC => self.c,
            Operand::Unused => unreachable!("invalid operand usage"),
        }
    }

    fn execute(&mut self, insn: Insn) -> Option<u64> {
        use Opcode::*;
        match insn.code {
            Adv => self.a /= 1 << self.operand(insn.op),
            Bxl => self.b ^= self.operand(insn.op),
            Bst => self.b = self.operand(insn.op) % 8,
            Jnz if self.a != 0 => {
                self.ip = self.operand(insn.op) as usize;
                return None;
            }
            Jnz => {}
            Bxc => self.b ^= self.c,
            Out => {
                self.ip += 1;
                return Some(self.operand(insn.op) % 8);
            }
            Bdv => self.b = self.a / (1 << self.operand(insn.op)),
            Cdv => self.c = self.a / (1 << self.operand(insn.op)),
        }
        self.ip += 1;
        None
    }

    fn eval(&mut self) -> &'_ [u64] {
        while let Some(insn) = self.insns.get(self.ip).copied() {
            if let Some(out) = self.execute(insn) {
                self.output.push(out);
            }
        }
        &self.output
    }

    fn eval_until_out(&mut self) -> Option<u64> {
        while let Some(insn) = self.insns.get(self.ip).copied() {
            if let Some(out) = self.execute(insn) {
                return Some(out);
            }
        }
        None
    }

    fn print(&self) -> String {
        let mut s = self.output.iter().fold(String::new(), |mut acc, i| {
            let _ = write!(acc, "{i},");
            acc
        });
        s.pop(); // Remove redundant comma at last
        s
    }
}

fn parse(mut lines: impl Iterator<Item = String>) -> (Vec<u8>, Vec<Insn>, Regs) {
    let a = lines.next().unwrap().strip_prefix("Register A: ").unwrap().parse().unwrap();
    let b = lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse().unwrap();
    let c = lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse().unwrap();
    assert!(lines.next().unwrap().is_empty());

    let bytes: Vec<u8> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let insns = bytes.chunks(2).map(|c| Insn::decode(c[0], c[1])).collect();
    (bytes, insns, (a, b, c))
}

fn part1(lines: impl Iterator<Item = String>) {
    let (_, insns, regs) = parse(lines);
    let mut prog = Program::new(&insns, regs);
    prog.eval();
    println!("{}", prog.print());
}

fn part2(lines: impl Iterator<Item = String>) {
    // This solution is dedicated to the puzzle input and not a generic solution.
    // The input consists of a single loop. Each iteration outputs a single value and divides A register by 8.
    // The lowest octal digit of A register value affects each output value. B and C registers are initialized
    // on each iteration. So we can search the A register value assuming that the each octal digits produce
    // each expected output values in order.
    //
    // To solve this problem generically, we can convert the program into Z3 solver's equations and solve the
    // equations by Z3 solver.

    let (raw, mut insns, (_, b, c)) = parse(lines);

    // Remove unnecessary instructions from the program to evaluate only one iteration of the loop.
    insns.remove(insns.iter().position(|i| i.code == Opcode::Adv).unwrap());
    insns.remove(insns.iter().position(|i| i.code == Opcode::Jnz).unwrap());

    fn solve(insns: &[Insn], (a, b, c): Regs, want: &[u64]) -> Option<u64> {
        let Some((&last, init)) = want.split_last() else {
            return Some(a);
        };
        (0..8)
            .map(|octal| (a << 3) + octal)
            .filter(|&a| {
                let mut prog = Program::new(insns, (a, b, c));
                let digit = prog.eval_until_out().unwrap();
                last == digit
            })
            .find_map(|a| solve(insns, (a, b, c), init))
    }

    let want: Vec<u64> = raw.into_iter().map(|i| i as _).collect();
    println!("{}", solve(&insns, (0, b, c), &want).unwrap());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
