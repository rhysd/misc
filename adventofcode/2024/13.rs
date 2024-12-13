use std::env;
use std::io::{self, BufRead};

type Pos = (i64, i64);

fn parse_machine(lines: impl Iterator<Item = String>) -> Option<(Pos, Pos, Pos)> {
    let mut lines = lines.skip_while(|l| l.is_empty());
    let mut parse_pos = move |l, x, y| {
        let line = lines.next()?;
        let mut s = line.strip_prefix(l)?.split(", ");
        let x = s.next()?.strip_prefix(x)?.parse().ok()?;
        let y = s.next()?.strip_prefix(y)?.parse().ok()?;
        Some((x, y))
    };
    let a = parse_pos("Button A: ", "X+", "Y+")?;
    let b = parse_pos("Button B: ", "X+", "Y+")?;
    let p = parse_pos("Prize: ", "X=", "Y=")?;
    Some((a, b, p))
}

// Button A: X+AX, Y+AY
// Button B: X+BX, Y+BY
// Prize: X=PX, Y=PY
//
// A * AX + B * BX = PX
// A * AY + B * BY = PY
//
// A * AX * BY + B * BX * BY = BY * PX
// A * AY * BX + B * BX * BY = BX * PY
// A * (AX * BY - AY * BX) = BY * PX - BX * PY
// A = (BY * PX - BX * PY) / (AX * BY - AY * BX)
//
// A * AX * AY + B * AY * BX = AY * PX
// A * AX * AY + B * AX * BY = AX * PY
// B * (AY * BX - AX * BY) = AY * PX - AX * PY
// B = (AY * PX - AX * PY) / (AY * BX - AX * BY)
fn tokens(a: Pos, b: Pos, p: Pos) -> Option<i64> {
    let ((ax, ay), (bx, by), (px, py)) = (a, b, p);

    let (t1, t2) = (by * px - bx * py, ax * by - ay * bx);
    let a = (t2 != 0 && t1 % t2 == 0).then(|| t1 / t2)?;

    let (t1, t2) = (ay * px - ax * py, ay * bx - ax * by);
    let b = (t2 != 0 && t1 % t2 == 0).then(|| t1 / t2)?;

    (a >= 0 && b >= 0).then(|| a * 3 + b)
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut total = 0;
    while let Some((a, b, p)) = parse_machine(&mut lines) {
        total += tokens(a, b, p).unwrap_or(0);
    }
    println!("{total}");
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut total = 0;
    while let Some((a, b, mut p)) = parse_machine(&mut lines) {
        p.0 += 10000000000000;
        p.1 += 10000000000000;
        total += tokens(a, b, p).unwrap_or(0);
    }
    println!("{total}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
