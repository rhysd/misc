use std::io::{self, BufRead};

fn main() {
    let mut scores: Vec<usize> = io::stdin()
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
                    _ => return None,
                }
            }

            let mut score = 0;
            for c in stack.into_iter().rev() {
                score = score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        c => panic!("oops {:?}", c),
                    };
            }
            Some(score)
        })
        .collect();

    scores.sort_unstable();
    println!("{}", scores[scores.len() / 2]);
}
