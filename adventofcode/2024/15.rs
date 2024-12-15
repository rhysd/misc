use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);
type Field = Vec<Vec<char>>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn new(c: char) -> Self {
        match c {
            '^' => Dir::U,
            '>' => Dir::R,
            'v' => Dir::D,
            '<' => Dir::L,
            _ => unreachable!(),
        }
    }

    fn advance(self, (x, y): Pos) -> Pos {
        match self {
            Dir::U => (x, y - 1),
            Dir::R => (x + 1, y),
            Dir::D => (x, y + 1),
            Dir::L => (x - 1, y),
        }
    }
}

fn gps(field: &Field) -> usize {
    field
        .iter()
        .enumerate()
        .map(|(y, xs)| {
            xs.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O' || c == '[')
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum()
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut robot = (0, 0);
    let mut field: Field = (&mut lines)
        .enumerate()
        .take_while(|(_, l)| !l.is_empty())
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        robot = (x, y);
                        '.'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();

    fn move_box(field: &mut Field, (ox, oy): Pos, dir: Dir) -> bool {
        let (mut x, mut y) = (ox, oy);
        loop {
            match field[y][x] {
                '.' => {
                    field[oy][ox] = '.';
                    field[y][x] = 'O';
                    return true;
                }
                '#' => return false,
                _ => (x, y) = dir.advance((x, y)),
            }
        }
    }

    for l in lines {
        for dir in l.chars().map(Dir::new) {
            let (x, y) = dir.advance(robot);
            let c = field[y][x];
            match c {
                '.' => robot = (x, y),
                'O' if move_box(&mut field, (x, y), dir) => robot = (x, y),
                _ => {}
            }
        }
    }

    println!("{}", gps(&field));
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut robot = (0, 0);
    let mut field: Field = (&mut lines)
        .enumerate()
        .take_while(|(_, l)| !l.is_empty())
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '@' => {
                        robot = (x * 2, y);
                        ['.', '.']
                    }
                    'O' => ['[', ']'],
                    c => [c, c],
                })
                .collect()
        })
        .collect();

    fn move_box_left(field: &mut Field, (x, y): Pos) -> bool {
        let line = &mut field[y][..=x];
        for (i, &c) in line.iter().enumerate().rev() {
            match c {
                '#' => break,
                '.' => {
                    line[i..].rotate_left(1);
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn move_box_right(field: &mut Field, (x, y): Pos) -> bool {
        let line = &mut field[y][x..];
        for (i, &c) in line.iter().enumerate() {
            match c {
                '#' => break,
                '.' => {
                    line[..=i].rotate_right(1);
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn can_move_box_virtical(field: &Field, (x, y): Pos, up: bool) -> bool {
        let (lx, rx) = match field[y][x] {
            ']' => (x - 1, x),
            '[' => (x, x + 1),
            '.' => return true,
            _ => return false,
        };
        let y = if up { y - 1 } else { y + 1 };
        can_move_box_virtical(field, (lx, y), up) && can_move_box_virtical(field, (rx, y), up)
    }

    fn move_box_virtical(field: &mut Field, (x, y): Pos, up: bool) {
        let (lx, rx) = match field[y][x] {
            ']' => (x - 1, x),
            '[' => (x, x + 1),
            '.' => return,
            _ => unreachable!(),
        };
        let dy = if up { y - 1 } else { y + 1 };
        move_box_virtical(field, (lx, dy), up);
        move_box_virtical(field, (rx, dy), up);
        (field[dy][lx], field[dy][rx]) = ('[', ']');
        (field[y][lx], field[y][rx]) = ('.', '.');
    }

    for l in lines {
        for dir in l.chars().map(Dir::new) {
            let (x, y) = dir.advance(robot);
            let c = field[y][x];
            match c {
                '.' => robot = (x, y),
                '[' | ']' => match dir {
                    Dir::L if move_box_left(&mut field, (x, y)) => robot = (x, y),
                    Dir::R if move_box_right(&mut field, (x, y)) => robot = (x, y),
                    Dir::U | Dir::D if can_move_box_virtical(&field, (x, y), dir == Dir::U) => {
                        move_box_virtical(&mut field, (x, y), dir == Dir::U);
                        robot = (x, y);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    println!("{}", gps(&field));
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
