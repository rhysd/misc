use std::cmp;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone)]
enum N {
    T(usize),
    P(Box<N>, Box<N>),
}

impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            N::T(n) => write!(f, "{}", n),
            N::P(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl N {
    fn add(self, other: N) -> N {
        N::P(Box::new(self), Box::new(other))
    }

    fn actions(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
        if depth >= 4 {
            if let N::P(l, r) = self {
                if let (N::T(l), N::T(r)) = (&**l, &**r) {
                    return Some((Some(*l), Some(*r)));
                }
            }
        }

        match self {
            N::T(_) => None,
            N::P(l, r) => {
                if let Some((lv, rv)) = l.explode(depth + 1) {
                    if let Some(rv) = rv {
                        if lv.is_some() {
                            *l = Box::new(N::T(0));
                        }
                        r.add_leftmost(rv);
                    }
                    return Some((lv, None));
                }
                if let Some((lv, rv)) = r.explode(depth + 1) {
                    if let Some(lv) = lv {
                        if rv.is_some() {
                            *r = Box::new(N::T(0));
                        }
                        l.add_rightmost(lv);
                    }
                    return Some((None, rv));
                }
                None
            }
        }
    }

    fn add_leftmost(&mut self, v: usize) {
        match self {
            N::T(n) => *n += v,
            N::P(l, _) => l.add_leftmost(v),
        }
    }
    fn add_rightmost(&mut self, v: usize) {
        match self {
            N::T(n) => *n += v,
            N::P(_, r) => r.add_rightmost(v),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            N::T(n) if *n >= 10 => {
                let (l, r) = (*n / 2, *n / 2 + *n % 2);
                *self = N::T(l).add(N::T(r));
                true
            }
            N::T(_) => false,
            N::P(l, r) => l.split() || r.split(),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            N::T(n) => *n,
            N::P(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn parse(s: &str) -> (&str, N) {
    if s.as_bytes()[0].is_ascii_digit() {
        let found = s.char_indices().find(|(_, c)| !c.is_ascii_digit());
        let idx = found.map(|(i, _)| i).unwrap_or(s.len());
        let n = s[..idx].parse().unwrap();
        (&s[idx..], N::T(n))
    } else {
        let (s, l) = parse(&s[1..]); // eat [
        let (s, r) = parse(&s[1..]); // eat ,
        (&s[1..], N::P(Box::new(l), Box::new(r))) // eat ]
    }
}

fn main() {
    let ns: Vec<_> = io::stdin().lock().lines().map(|l| parse(&l.unwrap()).1).collect();
    let mut max = 0;
    for i in 0..ns.len() {
        for j in 0..ns.len() {
            if i == j {
                continue;
            }
            let mut t = ns[i].clone().add(ns[j].clone());
            t.actions();
            max = cmp::max(t.magnitude(), max);
        }
    }
    println!("{}", max);
}
