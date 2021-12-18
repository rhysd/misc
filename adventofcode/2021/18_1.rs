use std::fmt;
use std::io::{self, BufRead};

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

#[derive(PartialEq, Eq)]
enum Explode {
    Happen(usize, usize),
    Left(usize),
    Right(usize),
    Done,
    NotYet,
}

impl N {
    fn add(self, other: N) -> N {
        N::P(Box::new(self), Box::new(other))
    }

    fn action(&mut self) {
        while (self.explode(0) != Explode::NotYet) || self.split() {}
    }

    fn explode(&mut self, depth: usize) -> Explode {
        use Explode::*;

        if depth >= 4 {
            if let N::P(l, r) = self {
                if let (N::T(l), N::T(r)) = (&**l, &**r) {
                    return Happen(*l, *r);
                }
            }
        }

        match self {
            N::T(_) => NotYet,
            N::P(l, r) => {
                match l.explode(depth + 1) {
                    Happen(a, b) => {
                        *l = Box::new(N::T(0));
                        r.add_leftmost(b);
                        return Left(a);
                    }
                    Right(v) => {
                        r.add_leftmost(v);
                        return Done;
                    }
                    Left(v) => return Left(v),
                    Done => return Done,
                    NotYet => {}
                }
                match r.explode(depth + 1) {
                    Happen(a, b) => {
                        *r = Box::new(N::T(0));
                        l.add_rightmost(a);
                        return Right(b);
                    }
                    Left(v) => {
                        l.add_rightmost(v);
                        return Done;
                    }
                    Right(v) => return Right(v),
                    Done => return Done,
                    NotYet => {}
                }
                NotYet
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
                let l = *n / 2;
                let r = *n / 2 + *n % 2;
                *self = N::P(Box::new(N::T(l)), Box::new(N::T(r)));
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
    let b = s.as_bytes()[0];
    if b.is_ascii_digit() {
        let (idx, _) = s
            .char_indices()
            .find(|(_, c)| !c.is_ascii_digit())
            .unwrap_or((s.len(), '🐶'));
        let n = s[..idx].parse().unwrap();
        (&s[idx..], N::T(n))
    } else {
        let (s, l) = parse(&s[1..]); // eat [
        let (s, r) = parse(&s[1..]); // eat ,
        (&s[1..], N::P(Box::new(l), Box::new(r))) // eat ]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut ns = stdin.lock().lines().map(|l| parse(&l.unwrap()).1);
    let mut sum = ns.next().unwrap();
    for n in ns {
        sum = sum.add(n);
        sum.action();
    }
    println!("{}", sum);
    println!("{}", sum.magnitude());
}
