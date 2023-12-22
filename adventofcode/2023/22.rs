use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize, usize);
type Surface = ((usize, usize), (usize, usize));

#[derive(Debug, Clone)]
struct Brick {
    upper: Pos,
    lower: Pos,
}

impl Brick {
    fn parse(line: &str) -> Self {
        let (l, r) = line.split_once('~').unwrap();

        fn parse_pos(s: &str) -> Pos {
            let mut i = s.split(',').map(|s| s.parse().unwrap());
            (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
        }

        let (l, r) = (parse_pos(l), parse_pos(r));
        if r > l {
            Self { upper: r, lower: l }
        } else {
            Self { upper: l, lower: r }
        }
    }

    fn surface(&self) -> Surface {
        ((self.lower.0, self.lower.1), (self.upper.0, self.upper.1))
    }
}

fn settle(bricks: &mut Vec<Brick>) -> usize {
    let mut count = 0;

    for i in 0..bricks.len() {
        let b = &bricks[i];
        let ((x1, y1), (x2, y2)) = b.surface();

        let mut z = b.lower.2;
        'fall: while z > 1 {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for b in &bricks[..i] {
                        if b.upper.2 + 1 != z {
                            continue;
                        }
                        let ((x1, y1), (x2, y2)) = b.surface();
                        if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
                            break 'fall;
                        }
                    }
                }
            }
            z -= 1;
        }

        let b = &mut bricks[i];
        if b.lower.2 != z {
            b.upper.2 -= b.lower.2 - z;
            b.lower.2 = z;
            count += 1;
        }
    }

    count
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut tower: Vec<_> = lines.map(|l| Brick::parse(&l)).collect();
    tower.sort_by_key(|b| b.lower.2);
    settle(&mut tower);
    tower.sort_by_key(|b| b.lower.2);

    fn is_safe(tower: &[Brick]) -> bool {
        if tower.iter().all(|b| b.lower.2 != 1) {
            return false;
        }
        'upper: for upper in tower {
            let z = upper.lower.2;
            if z == 1 {
                continue;
            }
            let ((ux1, uy1), (ux2, uy2)) = upper.surface();
            for lower in tower {
                if z != lower.upper.2 + 1 {
                    continue;
                }
                let ((lx1, ly1), (lx2, ly2)) = lower.surface();
                for x in lx1..=lx2 {
                    for y in ly1..=ly2 {
                        if (ux1..=ux2).contains(&x) && (uy1..=uy2).contains(&y) {
                            continue 'upper;
                        }
                    }
                }
            }
            return false;
        }
        true
    }

    let mut count: usize = 0;
    for i in 0..tower.len() {
        let removed = tower.remove(i);
        if is_safe(&tower) {
            count += 1;
        }
        tower.insert(i, removed);
    }

    println!("{count}");
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut snapshot: Vec<_> = lines.map(|l| Brick::parse(&l)).collect();
    snapshot.sort_by_key(|b| b.lower.2);

    let mut settled = Vec::<Brick>::new();
    for mut brick in snapshot {
        let ((x1, y1), (x2, y2)) = brick.surface();

        let mut z = brick.lower.2;
        'fall: while z > 1 {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for b in settled.iter() {
                        if b.upper.2 + 1 != z {
                            continue;
                        }
                        let ((x1, y1), (x2, y2)) = b.surface();
                        if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
                            break 'fall;
                        }
                    }
                }
            }
            z -= 1;
        }

        brick.upper.2 -= brick.lower.2 - z;
        brick.lower.2 = z;
        settled.push(brick);
    }
    settled.sort_by_key(|b| b.lower.2);

    let mut count: usize = 0;
    for i in 0..settled.len() {
        let mut tower = settled[..i].to_vec();
        tower.extend_from_slice(&settled[i + 1..]);
        count += settle(&mut tower);
    }

    println!("{count}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {x:?}"),
    }
}
