use std::env;
use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

type Pos<T> = (T, T, T);

#[derive(Debug)]
struct Hail<T> {
    init: Pos<T>,
    vel: Pos<T>,
}

impl<T> Hail<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fn parse(s: &str) -> Self {
        let (init, vel) = s.split_once(" @ ").unwrap();
        let mut init = init.split(", ").map(|s| s.trim().parse().unwrap());
        let init = (
            init.next().unwrap(),
            init.next().unwrap(),
            init.next().unwrap(),
        );
        let mut vel = vel.split(", ").map(|s| s.trim().parse().unwrap());
        let vel = (
            vel.next().unwrap(),
            vel.next().unwrap(),
            vel.next().unwrap(),
        );
        Self { init, vel }
    }
}

fn part1(lines: impl Iterator<Item = String>) {
    let hails: Vec<_> = lines.map(|l| Hail::parse(&l)).collect();

    const LOWER: f64 = 200000000000000.0;
    const UPPER: f64 = 400000000000000.0;

    fn will_cross(h1: &Hail<f64>, h2: &Hail<f64>) -> bool {
        // x = dx * t + px -> t = (x - px) / dx
        // y = dy * t + py -> t = (y - py) / dy
        // → (x - px) / dx = (y - py) / dy
        // → dy * x - dy * px = dx * y - dx * py
        // → dy * x - dx * y = dy * px - dx * py
        //
        // dy1 * x - dx1 * y = dy1 * px1 - dx1 * py1
        // dy2 * x - dx2 * y = dy2 * px2 - dx2 * py2
        //
        // → (dx2 * dy1 - dx1 * dy2) * y = dy1 * dy2 * px1 - dy1 * dy2 * px2 - dx1 * dy2 * py1 + dx2 * dy1 * py2
        // → y = (dy1 * dy2 * px1 - dy1 * dy2 * px2 - dx1 * dy2 * py1 + dx2 * dy1 * py2) / (dx2 * dy1 - dx1 * dy2)
        //
        // → (dy1 * dx2 - dy2 * dx1) * x = dy1 * dx2 * px1 - dx1 * dx2 * py1 - dy2 * dx1 * px2 + dx1 * dx2 * py2
        // → x = (dy1 * dx2 * px1 - dx1 * dx2 * py1 - dy2 * dx1 * px2 + dx1 * dx2 * py2) / (dy1 * dx2 - dy2 * dx1)

        let (px1, py1, _) = h1.init;
        let (dx1, dy1, _) = h1.vel;
        let (px2, py2, _) = h2.init;
        let (dx2, dy2, _) = h2.vel;

        let y = (dy1 * dy2 * px1 - dy1 * dy2 * px2 - dx1 * dy2 * py1 + dx2 * dy1 * py2)
            / (dx2 * dy1 - dx1 * dy2);
        let x = (dy1 * dx2 * px1 - dx1 * dx2 * py1 - dy2 * dx1 * px2 + dx1 * dx2 * py2)
            / (dy1 * dx2 - dy2 * dx1);
        let t1 = (x - px1) / dx1;
        let t2 = (x - px2) / dx2;

        t1 >= 0.0 && t2 >= 0.0 && LOWER <= x && x <= UPPER && LOWER <= y && y <= UPPER
    }

    let mut count = 0;
    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            if will_cross(&hails[i], &hails[j]) {
                count += 1;
            }
        }
    }
    println!("{count}");
}

#[cfg(feature = "z3")]
fn part2(lines: impl Iterator<Item = String>) {
    use z3::ast::{Ast as _, Int};
    use z3::{Config, Context, SatResult, Solver};

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Rock's position is represented as: P[r] + t * V[r]
    // For every hails `i`,
    //
    //     P[i] + t[i] * V[i] = P[r] + t[i] * V[r]
    //
    // should be met.

    // Use i64 since isize is not supported by z3 crate
    let hails: Vec<Hail<i64>> = lines.map(|l| Hail::parse(&l)).collect();

    // P[r]
    let rpx = Int::new_const(&ctx, "rpx");
    let rpy = Int::new_const(&ctx, "rpy");
    let rpz = Int::new_const(&ctx, "rpz");
    // V[r]
    let rdx = Int::new_const(&ctx, "rdx");
    let rdy = Int::new_const(&ctx, "rdy");
    let rdz = Int::new_const(&ctx, "rdz");

    let zero = Int::from_i64(&ctx, 0);
    for (i, hail) in hails.iter().enumerate() {
        // t[i]
        let t = Int::new_const(&ctx, format!("t_{i}"));
        solver.assert(&t.ge(&zero)); // Cannot go back to the past

        // P[i]
        let (px, py, pz) = hail.init;
        let px = Int::from_i64(&ctx, px);
        let py = Int::from_i64(&ctx, py);
        let pz = Int::from_i64(&ctx, pz);

        // V[i]
        let (dx, dy, dz) = hail.vel;
        let dx = Int::from_i64(&ctx, dx);
        let dy = Int::from_i64(&ctx, dy);
        let dz = Int::from_i64(&ctx, dz);

        // P[i] + t[i] * V[i] = P[r] + t[i] * V[r]
        solver.assert(&(&px + &t * &dx)._eq(&(&rpx + &t * &rdx)));
        solver.assert(&(&py + &t * &dy)._eq(&(&rpy + &t * &rdy)));
        solver.assert(&(&pz + &t * &dz)._eq(&(&rpz + &t * &rdz)));
    }

    assert_eq!(solver.check(), SatResult::Sat); // Check the quantifier is satisfiable

    // `Solver::get_model` returns a model for the previous `Solver::check` call.
    // https://docs.rs/z3/latest/z3/struct.Solver.html#method.get_model
    let model = solver.get_model().unwrap();

    // The second argument is `model_completion`: https://docs.rs/z3-sys/latest/z3_sys/fn.Z3_model_eval.html
    let result = model.eval(&(&rpx + &rpy + &rpz), true).unwrap();

    println!("{}", result.as_i64().unwrap());
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
