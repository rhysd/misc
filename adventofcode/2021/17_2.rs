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

    let sum: usize = (ys..-ys)
        .into_iter()
        .map(|vy| {
            (-xe..=xe)
                .into_iter()
                .filter(|vx| {
                    let (mut vx, mut vy) = (*vx, vy);
                    let (mut x, mut y) = (0, 0);
                    loop {
                        x += vx;
                        y += vy;
                        if vx > 0 {
                            vx -= 1;
                        }
                        vy -= 1;

                        let (x_ok, y_ok) = (xr.contains(&x), yr.contains(&y));
                        if x_ok && y_ok {
                            return true;
                        }
                        if !x_ok && (vx == 0 || x > xe) || y < ys {
                            return false;
                        }
                    }
                })
                .count()
        })
        .sum();

    println!("{}", sum);
}
