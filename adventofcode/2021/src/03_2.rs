use std::collections::HashSet;
use std::io::{self, BufRead};

fn calc(mut h: HashSet<Vec<char>>, cmp: fn(usize, usize) -> bool) -> usize {
    let mut idx = 0;
    loop {
        let ones = h.iter().filter(|b| b[idx] == '1').count();
        let survive = if cmp(ones, h.len() - ones) { '1' } else { '0' };
        h.retain(|x| x[idx] == survive);
        if h.len() == 1 {
            let v = h.into_iter().next().unwrap();
            return usize::from_str_radix(&v.into_iter().collect::<String>(), 2).unwrap();
        }
        idx += 1;
    }
}

fn main() {
    let mut h = HashSet::new();
    for l in io::BufReader::new(io::stdin().lock()).lines() {
        h.insert(l.unwrap().chars().collect::<Vec<_>>());
    }

    let oxy = calc(h.clone(), |x, y| x >= y);
    let co2 = calc(h, |x, y| x < y);
    println!("{}", oxy * co2);
}
