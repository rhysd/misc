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

    fn top_z(&self) -> usize {
        self.upper.2 + 1
    }

    fn fall_to(&mut self, z: usize) {
        self.upper.2 -= self.lower.2 - z;
        self.lower.2 = z;
    }
}

fn collide(((lx1, ly1), (ux1, uy1)): Surface, ((lx2, ly2), (ux2, uy2)): Surface) -> bool {
    fn overwrap(s1: usize, e1: usize, s2: usize, e2: usize) -> bool {
        !(e1 < s2 || e2 < s1)
    }
    overwrap(lx1, ux1, lx2, ux2) && overwrap(ly1, uy1, ly2, uy2)
}

fn settle(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let falling = &bricks[i];
        let surface = falling.surface();
        let z = falling.lower.2;
        let z = if let Some(collided) = bricks[..i]
            .iter()
            .filter(|b| b.top_z() <= z && collide(surface, b.surface()))
            .max_by_key(|b| b.upper.2)
        {
            collided.top_z()
        } else {
            1
        };

        let b = &mut bricks[i];
        if b.lower.2 != z {
            b.fall_to(z);
        }
    }
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
            let upper_surface = upper.surface();
            for lower in tower {
                if z != lower.top_z() {
                    continue;
                }
                if collide(upper_surface, lower.surface()) {
                    continue 'upper;
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
    let mut tower: Vec<_> = lines.map(|l| Brick::parse(&l)).collect();
    tower.sort_by_key(|b| b.lower.2);
    settle(&mut tower);
    tower.sort_by_key(|b| b.lower.2);

    let mut count: usize = 0;
    for i in 0..tower.len() {
        let mut settled = Vec::with_capacity(tower.len() - 1);
        settled.extend_from_slice(&tower[..i]);
        let maybe_fall = &tower[i + 1..];

        for b in maybe_fall.iter() {
            let surface = b.surface();

            let mut z = b.lower.2;
            'fall: while z > 1 {
                for b in settled.iter() {
                    if b.top_z() != z {
                        continue;
                    }
                    if collide(surface, b.surface()) {
                        break 'fall;
                    }
                }
                z -= 1;
            }

            let mut b = b.clone();
            if b.lower.2 != z {
                b.fall_to(z);
                count += 1;
            }
            settled.push(b);
        }
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
