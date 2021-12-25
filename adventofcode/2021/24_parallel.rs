// dashmap = "5.0.0"
// fnv = "1.0.7"
// rayon = "1.5.1"

use dashmap::DashSet;
use fnv::FnvBuildHasher;
use rayon::prelude::*;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Var {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl Var {
    fn parse(c: char) -> Option<Self> {
        match c {
            'x' => Some(Self::X),
            'y' => Some(Self::Y),
            'z' => Some(Self::Z),
            'w' => Some(Self::W),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Var(Var),
    Imm(i32),
}

#[derive(Clone, Copy, Debug)]
enum Insn {
    Inp(Var),
    Add(Var, Operand),
    Mul(Var, Operand),
    Div(Var, Operand),
    Mod(Var, Operand),
    Eql(Var, Operand),
}

impl Insn {
    fn parse(line: &str) -> Self {
        fn parse_var(line: &str) -> Option<(&str, Var)> {
            let mut chars = line.char_indices();
            let (_, c) = chars.next().unwrap();
            let v = Var::parse(c)?;
            let i = if let Some((i, space)) = chars.next() {
                assert_eq!(space, ' ');
                i + 1
            } else {
                1
            };
            Some((&line[i..], v))
        }

        fn parse_operands(line: &str) -> (Var, Operand) {
            let (line, l) = parse_var(line).unwrap();
            let r = if let Some((_, v)) = parse_var(line) {
                Operand::Var(v)
            } else {
                Operand::Imm(line.parse().unwrap())
            };
            (l, r)
        }

        if let Some(line) = line.strip_prefix("inp ") {
            Insn::Inp(parse_var(line).unwrap().1)
        } else if let Some(line) = line.strip_prefix("add ") {
            let (v, o) = parse_operands(line);
            Insn::Add(v, o)
        } else if let Some(line) = line.strip_prefix("mul ") {
            let (v, o) = parse_operands(line);
            Insn::Mul(v, o)
        } else if let Some(line) = line.strip_prefix("div ") {
            let (v, o) = parse_operands(line);
            Insn::Div(v, o)
        } else if let Some(line) = line.strip_prefix("mod ") {
            let (v, o) = parse_operands(line);
            Insn::Mod(v, o)
        } else if let Some(line) = line.strip_prefix("eql ") {
            let (v, o) = parse_operands(line);
            Insn::Eql(v, o)
        } else {
            panic!("Invalid instruction {:?}", line);
        }
    }
}

#[derive(Debug, Clone)]
struct Alu<'p> {
    prog: &'p [Insn],
    vars: [i32; 4],
}

impl<'p> Alu<'p> {
    fn new(prog: &'p [Insn]) -> Self {
        Self { prog, vars: [0; 4] }
    }

    fn load(&self, v: Var) -> i32 {
        self.vars[v as usize]
    }

    fn store(&mut self, v: Var, i: i32) {
        self.vars[v as usize] = i;
    }

    fn operand(&self, o: Operand) -> i32 {
        match o {
            Operand::Var(v) => self.load(v),
            Operand::Imm(i) => i,
        }
    }

    fn step(&mut self, input: i32) {
        use Insn::*;
        let mut did_input = false;
        while let Some((insn, prog)) = self.prog.split_first() {
            match *insn {
                Inp(_) if did_input => return,
                Inp(v) => {
                    self.store(v, input);
                    did_input = true;
                }
                Add(v, o) => self.store(v, self.load(v) + self.operand(o)),
                Mul(v, o) => self.store(v, self.load(v) * self.operand(o)),
                Div(v, o) => self.store(v, self.load(v) / self.operand(o)),
                Mod(v, o) => self.store(v, self.load(v) % self.operand(o)),
                Eql(v, o) => self.store(v, if self.load(v) == self.operand(o) { 1 } else { 0 }),
            }
            self.prog = prog;
        }
    }
}

impl<'p> Hash for Alu<'p> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prog.len().hash(state);
        self.vars.hash(state);
    }
}
impl<'p> PartialEq for Alu<'p> {
    fn eq(&self, other: &Self) -> bool {
        self.prog.len() == other.prog.len() && self.vars == other.vars
    }
}
impl<'p> Eq for Alu<'p> {}

macro_rules! solver {
    ($name:ident, $iter:expr) => {
        fn $name<'p>(alu: &Alu<'p>, depth: u8, known: &DashSet<Alu<'p>, FnvBuildHasher>) -> Option<u64> {
            $iter.find_map_first(|i| {
                let mut alu = alu.clone();
                alu.step(i);
                if known.contains(&alu) {
                    return None;
                }
                if depth == 13 {
                    if alu.load(Var::Z) == 0 {
                        return Some(i as u64);
                    }
                } else if let Some(n) = $name(&alu, depth + 1, known) {
                    return Some(n * 10 + i as u64);
                }
                known.insert(alu);
                None
            })
        }
    };
}
solver!(part1, (1..10).into_par_iter().rev());
solver!(part2, (1..10).into_par_iter());

fn rev(mut u: u64) -> u64 {
    let mut r = 0;
    while u != 0 {
        r = r * 10 + u % 10;
        u /= 10;
    }
    r
}

fn main() {
    let prog: Vec<_> = io::stdin().lock().lines().map(|l| Insn::parse(&l.unwrap())).collect();
    let alu = Alu::new(&prog);
    let known = DashSet::with_hasher(FnvBuildHasher::default());
    println!("Part 1: {}", rev(part1(&alu, 0, &known).unwrap()));
    println!("Part 2: {}", rev(part2(&alu, 0, &known).unwrap()));
}
