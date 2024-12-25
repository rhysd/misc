use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn new(s: &str) -> Self {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gate<'a> {
    op: Op,
    l: &'a str,
    r: &'a str,
    out: &'a str,
}

impl<'a> Gate<'a> {
    fn parse(line: &'a str) -> Self {
        let mut s = line.split_whitespace();
        let l = s.next().unwrap();
        let op = Op::new(s.next().unwrap());
        let r = s.next().unwrap();
        assert_eq!(s.next().unwrap(), "->");
        let out = s.next().unwrap();
        let (l, r) = if l < r { (l, r) } else { (r, l) }; // Left is always smaller
        Self { op, l, r, out }
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let mut lines = lines.iter();

    let mut vars = HashMap::new();
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        let mut s = l.split(": ");
        let (name, val) = (s.next().unwrap(), s.next().unwrap() == "1");
        vars.insert(name, val);
    }
    let gates: Vec<_> = lines.map(|l| Gate::parse(l)).collect();

    loop {
        let prev_len = vars.len();
        for gate in &gates {
            if vars.contains_key(&gate.out) {
                continue;
            }
            if let Some(&l) = vars.get(&gate.l) {
                if let Some(&r) = vars.get(&gate.r) {
                    let v = match gate.op {
                        Op::And => l && r,
                        Op::Or => l || r,
                        Op::Xor => l ^ r,
                    };
                    vars.insert(gate.out, v);
                }
            }
        }
        if prev_len == vars.len() {
            break;
        }
    }
    let mut result = 0u64;
    for (name, val) in vars.into_iter() {
        if !name.starts_with('z') || !val {
            continue;
        }
        let idx: u8 = name[1..].parse().unwrap();
        result |= 1 << idx;
    }
    println!("{result}");
}

// Because x + y = z, x00 & y00 & z00 should constructs half-adder and xNN & yNN & zNN should constructs
// full-adders. The wrong outputs should violate the restrictions for constructing these adders. What
// we should do is validating each connections and finding the outputs which are not matching the below
// patterns.
//
// Half-adder:
//   z(0) = x(0) XOR y(0)
//   c(0) = x(0) AND y(0)
//
// Full-adder:
//   z1(n) = x(n) XOR y(n)
//   c1(n) = x(n) AND y(n)
//   z(n) = z1(n) XOR c(n-1)
//   c2(n) = z1(n) AND c(n-1)
//   c(n) = c1(n) OR c2(n)
//
// Note that this solution assumes minimal multi-bits adder as input. If some circuits are redundant,
// this solution does not work. As far as checking the connections manually, there were no redundant
// circuits.
//
// ref: https://ja.wikipedia.org/wiki/%E5%8A%A0%E7%AE%97%E5%99%A8
fn part2(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let mut lines = lines.iter();

    for l in &mut lines {
        if l.is_empty() {
            break;
        }
    }

    let gates: HashMap<_, _> = lines
        .map(|l| {
            let g = Gate::parse(l);
            (g.out, g)
        })
        .collect();

    #[derive(Default)]
    struct Validator<'a> {
        invalid: Vec<&'a str>,
    }

    impl<'a> Validator<'a> {
        fn validate_x_y(&mut self, g: &Gate<'a>) -> bool {
            match (g.l.starts_with('x'), g.r.starts_with('y')) {
                (true, true) if (g.l[1..] != g.r[1..]) => {
                    self.invalid.push(g.out);
                    true
                }
                (true, true) => true,
                (true, false) | (false, true) => {
                    self.invalid.push(g.out);
                    false
                }
                (false, false) => false,
            }
        }

        fn validate_half_adder(&mut self, g: &Gate<'a>) {
            let &Gate { l, r, op, out } = g;
            if l != "x00" || r != "y00" ||
                // Validate `z(0) = x(0) XOR y(0)`
                op == Op::Xor && !g.out.starts_with("z") ||
                // Validate `c(0) = x(0) AND y(0)`
                op == Op::And && g.out.starts_with("z")
            {
                self.invalid.push(out);
            }
        }

        fn validate_full_adder_or(&mut self, g: &Gate<'a>, gates: &HashMap<&'a str, Gate<'a>>) {
            let &Gate { l, r, out, .. } = g;
            if out.starts_with("z") && out != "z45" {
                // z45 is the most significant bit
                self.invalid.push(out);
            }
            // Validate children `c1(n)` and `c2(n)`
            let lc = gates.get(l).unwrap();
            let rc = gates.get(r).unwrap();
            if lc.op != Op::And {
                self.invalid.push(l);
            }
            if rc.op != Op::And {
                self.invalid.push(r);
            }
            // Validate `c2(n)`
            let cc = if self.validate_x_y(lc) {
                if lc.op != Op::And {
                    self.invalid.push(lc.out);
                }
                rc
            } else if self.validate_x_y(rc) {
                if rc.op != Op::And {
                    self.invalid.push(rc.out);
                }
                lc
            } else {
                self.invalid.push(g.out);
                return;
            };
            let lcc = gates.get(cc.l).unwrap();
            let rcc = gates.get(cc.r).unwrap();
            // Validate `z1(n)`
            let zc = if self.validate_x_y(lcc) {
                lcc
            } else if self.validate_x_y(rcc) {
                rcc
            } else {
                self.invalid.push(cc.out);
                return;
            };
            // Validate `z1(n) = x(n) XOR y(n)`
            if zc.op != Op::Xor {
                self.invalid.push(zc.out);
            }
        }

        fn validate_full_adder(&mut self, g: &Gate<'a>, gates: &HashMap<&'a str, Gate<'a>>) {
            let &Gate { op, out, .. } = g;
            let is_x_y = self.validate_x_y(g);
            if is_x_y {
                // Validate `z1(n) = x(n) XOR y(n)` and `c1(n) = x(n) AND y(n)`
                if op == Op::Or || out.starts_with("z") {
                    self.invalid.push(out);
                }
                return;
            }

            match op {
                Op::Xor => {
                    // Validate `z(n) = z1(n) XOR c(n-1)`
                    if !out.starts_with("z") {
                        self.invalid.push(out);
                    }
                }
                Op::And => {
                    // Validate `c2(n) = z1(n) AND c(n-1)`
                    if out.starts_with("z") {
                        self.invalid.push(out);
                    }
                }
                Op::Or => self.validate_full_adder_or(g, gates),
            }
        }

        fn validate(&mut self, gates: &HashMap<&'a str, Gate<'a>>) {
            for gate in gates.values() {
                if gate.l == "x00" {
                    self.validate_half_adder(gate);
                } else {
                    self.validate_full_adder(gate, gates);
                }
            }
        }
    }

    let mut v = Validator::default();
    v.validate(&gates);
    v.invalid.sort();
    v.invalid.dedup();
    assert_eq!(v.invalid.len(), 8);
    println!("{}", v.invalid.join(","));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
