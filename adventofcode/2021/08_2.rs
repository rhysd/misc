use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let total: usize = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut sp = l.split(" | ");
            let left = sp.next().unwrap();
            let mut left = left
                .split(' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            let mut ans = HashMap::<u8, _>::new();
            ans.insert(1, left.swap_remove(left.iter().position(|cs| cs.len() == 2).unwrap()));
            ans.insert(4, left.swap_remove(left.iter().position(|cs| cs.len() == 4).unwrap()));
            ans.insert(7, left.swap_remove(left.iter().position(|cs| cs.len() == 3).unwrap()));
            ans.insert(8, left.swap_remove(left.iter().position(|cs| cs.len() == 7).unwrap()));

            let v: Vec<_> = left
                .iter()
                .enumerate()
                .filter(|(_, h)| h.len() == 6)
                .map(|(idx, cs)| {
                    let n = if ans[&4].intersection(cs).count() == 4 {
                        9
                    } else if ans[&1].intersection(cs).count() == 2 {
                        0
                    } else {
                        6
                    };
                    (idx, n)
                })
                .collect();
            for (idx, n) in v.into_iter().rev() {
                ans.insert(n, left.swap_remove(idx));
            }

            let v: Vec<_> = left
                .iter()
                .enumerate()
                .filter(|(_, h)| h.len() == 5)
                .map(|(idx, cs)| {
                    let n = if ans[&1].intersection(cs).count() == 2 {
                        3
                    } else if ans[&6].intersection(cs).count() == 5 {
                        5
                    } else {
                        2
                    };
                    (idx, n)
                })
                .collect();
            for (idx, n) in v.into_iter().rev() {
                ans.insert(n, left.swap_remove(idx));
            }

            let mut n = 0;
            for r in sp.next().unwrap().split(' ').map(|s| s.chars().collect::<HashSet<_>>()) {
                let (k, _) = ans.iter().find(|(_, v)| &r == *v).unwrap();
                n = n * 10 + *k as usize;
            }
            n
        })
        .sum();
    println!("{}", total);
}
