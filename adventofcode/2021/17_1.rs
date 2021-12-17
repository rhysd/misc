use std::cmp;
use std::io::{self, BufRead};

fn parse_input() -> ((i32, i32), (i32, i32)) {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let line = line.strip_prefix("target area: ").unwrap();
    let mut s = line.split(", ");
    let mut parse = || {
        let s = &s.next().unwrap()[2..];
        let mut s = s.split("..");
        let l = s.next().unwrap().parse().unwrap();
        let r = s.next().unwrap().parse().unwrap();
        (l, r)
    };
    (parse(), parse())
}

fn main() {
    let ((xs, xe), (ys, ye)) = parse_input();
    let xr = xs..=xe;
    let yr = ys..=ye;

    let max = (0..-ys)
        .into_iter()
        .filter_map(|vy| {
            (0..=xe)
                .into_iter()
                .filter_map(|vx| {
                    let (mut vx, mut vy) = (vx, vy);
                    let (mut x, mut y) = (0, 0);
                    let mut max_y = 0;
                    loop {
                        x += vx;
                        y += vy;
                        if vx > 0 {
                            vx -= 1;
                        }
                        vy -= 1;
                        max_y = cmp::max(max_y, y);

                        let (x_ok, y_ok) = (xr.contains(&x), yr.contains(&y));
                        if x_ok && y_ok {
                            return Some(max_y);
                        }
                        if !x_ok && (vx == 0 || x > xe) || y < ys {
                            return None;
                        }
                    }
                })
                .max()
        })
        .max()
        .unwrap();

    println!("{}", max);
}
