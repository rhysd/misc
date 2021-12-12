use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut nodes = HashMap::new();
    for l in lines.iter() {
        let mut s = l.split('-');
        let n1 = s.next().unwrap();
        let n2 = s.next().unwrap();
        nodes.entry(n1).or_insert(vec![]).push(n2);
        nodes.entry(n2).or_insert(vec![]).push(n1);
    }

    let mut count = 0;
    let mut q = VecDeque::new();
    q.push_back(("start", HashSet::new()));
    while !q.is_empty() {
        let (n, v) = q.pop_front().unwrap();
        for n in &nodes[n] {
            match *n {
                "end" => count += 1,
                "start" => { /* do nothing */ }
                n if n.chars().all(char::is_lowercase) => {
                    if !v.contains(n) {
                        let mut v = v.clone();
                        v.insert(n);
                        q.push_back((n, v));
                    }
                }
                n => q.push_back((n, v.clone())),
            }
        }
    }
    println!("{}", count);
}
