use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate<'a> {
    op: Op,
    l: &'a str,
    r: &'a str,
}

fn parse(lines: &[String]) -> (HashMap<&'_ str, bool>, HashMap<&'_ str, Gate<'_>>) {
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

    let mut gates = HashMap::new();
    for l in lines {
        let mut s = l.split(" -> ");
        let (expr, out) = (s.next().unwrap(), s.next().unwrap());
        let (mut s, op) = if expr.contains(" AND ") {
            (expr.split(" AND "), Op::And)
        } else if expr.contains(" OR ") {
            (expr.split(" OR "), Op::Or)
        } else if expr.contains(" XOR ") {
            (expr.split(" XOR "), Op::Xor)
        } else {
            unreachable!();
        };
        let (l, r) = (s.next().unwrap(), s.next().unwrap());
        let (l, r) = if l < r { (l, r) } else { (r, l) }; // Left is always smaller
        gates.insert(out, Gate { op, l, r });
    }

    (vars, gates)
}

fn part1(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let (mut vars, gates) = parse(&lines);
    loop {
        let prev_len = vars.len();
        for (&out, gate) in &gates {
            if vars.contains_key(&out) {
                continue;
            }
            if let Some(&l) = vars.get(&gate.l) {
                if let Some(&r) = vars.get(&gate.r) {
                    let v = match gate.op {
                        Op::And => l && r,
                        Op::Or => l || r,
                        Op::Xor => l ^ r,
                    };
                    vars.insert(out, v);
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
//   z'(n) = x(n) XOR y(n)
//   c'(n) = x(n) AND y(n)
//   z(n) = z'(n) XOR c(n-1)
//   c''(n) = z'(n) AND c(n-1)
//   c(n) = c'(n) OR c''(n)
//
// Note that this solution assumes minimal multi-bits adder as input. If some circuits are redundant,
// this solution does not work. As far as checking the connections manually, there were no redundant
// circuits.
//
// ref: https://ja.wikipedia.org/wiki/%E5%8A%A0%E7%AE%97%E5%99%A8
fn part2(lines: impl Iterator<Item = String>) {
    let lines: Vec<_> = lines.collect();
    let (_, gates) = parse(&lines);

    fn is_x_y(l: &str, r: &str) -> bool {
        l.starts_with('x') && r.starts_with('y') && l[1..] == r[1..]
    }

    let mut wrong = vec![];
    for (&out, gate) in &gates {
        let &Gate { l, r, op } = gate;
        if is_x_y(l, r) {
            // Validate half-adder
            if &l[1..] == "00" {
                match op {
                    // Validate `z(0) = x(0) XOR y(0)`
                    Op::Xor if !out.starts_with("z") => wrong.push(out),
                    // Validate `c(0) = x(0) AND y(0)`
                    Op::And if out.starts_with("z") => wrong.push(out),
                    _ => {}
                }
            } else {
                // Validate `z'(n) = x(n) XOR y(n)` and `c'(n) = x(n) AND y(n)`
                if op == Op::Or || out.starts_with("z") {
                    wrong.push(out);
                }
            }
        } else {
            match op {
                // Validate `z(n) = z'(n) XOR c(n-1)`
                Op::Xor if !out.starts_with("z") => wrong.push(out),
                Op::Xor => {}
                // Validate `c''(n) = z'(n) AND c(n-1)`
                Op::And if out.starts_with("z") => wrong.push(out),
                Op::And => {}
                // Validate `c(n) = c'(n) OR c''(n)`
                Op::Or if out.starts_with("z") && out != "z45" => wrong.push(out),
                Op::Or => {
                    // Validate children `c'(n)` and `c''(n)`
                    let lc = gates.get(l).unwrap();
                    let rc = gates.get(r).unwrap();
                    if lc.op != Op::And {
                        wrong.push(l);
                    }
                    if rc.op != Op::And {
                        wrong.push(r);
                    }
                    // Validate `c''(n)`
                    let cc = if is_x_y(lc.l, lc.r) {
                        rc
                    } else if is_x_y(rc.l, rc.r) {
                        lc
                    } else {
                        continue;
                    };
                    let lcc = gates.get(cc.l).unwrap();
                    let rcc = gates.get(cc.r).unwrap();
                    // Validate `z'(n)`
                    let (n, zc) = if is_x_y(lcc.l, lcc.r) {
                        (cc.l, lcc)
                    } else if is_x_y(rcc.l, rcc.r) {
                        (cc.r, rcc)
                    } else {
                        continue;
                    };
                    // Validate `z'(n) = x(n) XOR y(n)`
                    if zc.op != Op::Xor {
                        wrong.push(n);
                    }
                }
            }
        }
    }
    wrong.sort();
    wrong.dedup();
    println!("{}", wrong.join(","));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
