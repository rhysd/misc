use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str;

fn calc(mut h: HashSet<Vec<u8>>, cmp: fn(usize, usize) -> bool) -> usize {
    let mut idx = 0;
    loop {
        let ones = h.iter().filter(|b| b[idx] == b'1').count();
        let survive = if cmp(ones, h.len() - ones) { b'1' } else { b'0' };
        h.retain(|x| x[idx] == survive);
        if h.len() == 1 {
            let v = h.into_iter().next().unwrap();
            return usize::from_str_radix(str::from_utf8(&v).unwrap(), 2).unwrap();
        }
        idx += 1;
    }
}

fn main() {
    let h = io::BufReader::new(io::stdin().lock())
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect::<HashSet<_>>();
    let oxy = calc(h.clone(), |ones, zeros| ones >= zeros);
    let co2 = calc(h, |ones, zeros| ones < zeros);
    println!("{}", oxy * co2);
}
