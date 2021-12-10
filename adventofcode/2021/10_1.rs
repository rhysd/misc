use std::io::{self, BufRead};

fn main() {
    let score: usize = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            let mut stack = vec![];
            for c in l.unwrap().chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' if stack.pop() == Some('(') => (),
                    ']' if stack.pop() == Some('[') => (),
                    '}' if stack.pop() == Some('{') => (),
                    '>' if stack.pop() == Some('<') => (),
                    c => return Some(c),
                }
            }
            None
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            x => panic!("unexpected {:?}", x),
        })
        .sum();
    println!("{}", score);
}
