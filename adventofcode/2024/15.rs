use std::env;
use std::io::{self, BufRead};

type Pos = (usize, usize);

#[derive(Clone, Copy, Debug)]
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

    fn dest(self, (x, y): Pos) -> Pos {
        match self {
            Dir::U => (x, y - 1),
            Dir::R => (x + 1, y),
            Dir::D => (x, y + 1),
            Dir::L => (x - 1, y),
        }
    }
    fn dest_y(self, y: usize) -> usize {
        match self {
            Dir::U => y - 1,
            Dir::D => y + 1,
            _ => y,
        }
    }
}

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut robot = (0, 0);
    let mut field: Vec<Vec<char>> = (&mut lines)
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

    fn move_box(field: &mut [Vec<char>], (px, py): Pos, dir: Dir) -> bool {
        let (x, y) = dir.dest((px, py));
        let c = field[y][x];
        match c {
            '.' => {
                field[py][px] = '.';
                field[y][x] = 'O';
                true
            }
            'O' if move_box(field, (x, y), dir) => {
                field[py][px] = '.';
                field[y][x] = 'O';
                true
            }
            _ => false,
        }
    }

    for l in lines {
        for c in l.chars() {
            let dir = Dir::new(c);
            let (x, y) = dir.dest(robot);
            let c = field[y][x];
            match c {
                '.' => robot = (x, y),
                'O' if move_box(&mut field, (x, y), dir) => robot = (x, y),
                _ => {}
            }
        }
    }

    let gps: usize = field
        .iter()
        .enumerate()
        .map(|(y, xs)| {
            xs.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O')
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum();
    println!("{gps}");
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut robot = (0, 0);
    let mut field: Vec<Vec<char>> = (&mut lines)
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

    fn can_move_box(field: &[Vec<char>], (x, y): Pos, dir: Dir) -> bool {
        match field[y][x] {
            ']' => {
                can_move_box(field, dir.dest((x - 1, y)), dir)
                    && can_move_box(field, dir.dest((x, y)), dir)
            }
            '[' => {
                can_move_box(field, dir.dest((x, y)), dir)
                    && can_move_box(field, dir.dest((x + 1, y)), dir)
            }
            '.' => true,
            _ => false,
        }
    }

    fn move_box(field: &mut [Vec<char>], (x, y): Pos, dir: Dir) {
        let c = field[y][x];
        match c {
            ']' => {
                move_box(field, dir.dest((x - 1, y)), dir);
                move_box(field, dir.dest((x, y)), dir);
                let dy = dir.dest_y(y);
                field[dy][x - 1] = '[';
                field[dy][x] = ']';
                field[y][x - 1] = '.';
                field[y][x] = '.';
            }
            '[' => {
                move_box(field, dir.dest((x, y)), dir);
                move_box(field, dir.dest((x + 1, y)), dir);
                let dy = dir.dest_y(y);
                field[dy][x] = '[';
                field[dy][x + 1] = ']';
                field[y][x] = '.';
                field[y][x + 1] = '.';
            }
            _ => {}
        }
    }

    for l in lines {
        for dir in l.chars().map(Dir::new) {
            let (x, y) = dir.dest(robot);
            let c = field[y][x];
            match c {
                '.' => robot = (x, y),
                '[' | ']' => match dir {
                    Dir::L => {
                        let line = &mut field[y][..=x];
                        for (i, &c) in line.iter().enumerate().rev() {
                            match c {
                                '#' => break,
                                '.' => {
                                    line[i..].rotate_left(1);
                                    robot = (x, y);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Dir::R => {
                        let line = &mut field[y][x..];
                        for (i, &c) in line.iter().enumerate() {
                            match c {
                                '#' => break,
                                '.' => {
                                    line[..=i].rotate_right(1);
                                    robot = (x, y);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Dir::U | Dir::D => {
                        if can_move_box(&field, (x, y), dir) {
                            move_box(&mut field, (x, y), dir);
                            robot = (x, y);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let gps: usize = field
        .iter()
        .enumerate()
        .map(|(y, xs)| {
            xs.iter()
                .enumerate()
                .filter(|(_, &c)| c == '[')
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum();

    println!("{gps}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
