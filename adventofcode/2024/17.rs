use std::env;
use std::fmt::Write;
use std::io::{self, BufRead};

use z3::ast::Ast;

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

struct Program<'a> {
    insns: &'a [Insn],
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    output: Vec<u64>,
}

impl<'a> Program<'a> {
    fn new(insns: &'a [Insn], (a, b, c): (u64, u64, u64)) -> Self {
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

    fn execute(&mut self, insn: Insn) {
        use Opcode::*;
        match insn.code {
            Adv => self.a /= 1 << self.operand(insn.op),
            Bxl => self.b ^= self.operand(insn.op),
            Bst => self.b = self.operand(insn.op) % 8,
            Jnz if self.a != 0 => {
                self.ip = self.operand(insn.op) as usize;
                return;
            }
            Jnz => {}
            Bxc => self.b ^= self.c,
            Out => self.output.push(self.operand(insn.op) % 8),
            Bdv => self.b = self.a / (1 << self.operand(insn.op)),
            Cdv => self.c = self.a / (1 << self.operand(insn.op)),
        }
        self.ip += 1;
    }

    fn run(&mut self) -> &'_ [u64] {
        while let Some(insn) = self.insns.get(self.ip).copied() {
            self.execute(insn);
        }
        &self.output
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

fn parse(mut lines: impl Iterator<Item = String>) -> (Vec<u8>, Vec<Insn>, (u64, u64, u64)) {
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
    prog.run();
    println!("{}", prog.print());
}

// fn program(mut a: u64, mut b: u64, mut c: u64) {
//     loop {
//         b = a % 8;
//         b = b ^ 5;
//         c = a / (1 << b);
//         a = a / (1 << 3);
//         b = b ^ 6;
//         b = b ^ c;
//         print(b % 8);
//         if a == 0 {
//             break;
//         }
//     }
// }

fn part2(_: impl Iterator<Item = String>) {
    use z3::ast::BV;
    use z3::{Config, Context, Optimize, SatResult};

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Optimize::new(&ctx);
    let answer = BV::new_const(&ctx, "A", 64);

    let mut a = answer.clone();
    for i in [2, 4, 1, 5, 7, 5, 0, 3, 1, 6, 4, 3, 5, 5, 3, 0] {
        let mut b = a.bvsmod(&BV::from_u64(&ctx, 8, 64));
        b = &b ^ BV::from_u64(&ctx, 5, 64);
        let c = a.bvsdiv(&(BV::from_u64(&ctx, 1, 64) << &b));
        a = a.bvsdiv(&BV::from_u64(&ctx, 1 << 3, 64));
        b = &b ^ BV::from_u64(&ctx, 6, 64);
        b = &b ^ &c;
        solver.assert(&b.bvsmod(&BV::from_u64(&ctx, 8, 64))._eq(&BV::from_u64(&ctx, i, 64)));
    }
    solver.assert(&a._eq(&BV::from_u64(&ctx, 0, 64)));
    solver.minimize(&answer); // We need the smallest number

    assert_eq!(solver.check(&[]), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let result = model.eval(&answer, true).unwrap();
    println!("{}", result.as_u64().unwrap());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
