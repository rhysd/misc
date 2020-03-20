#![feature(test)]

extern crate test;

use std::convert::TryInto;
use std::i32;

pub fn read_i32(b: &[u8]) -> i32 {
    let b: [u8; 4] = b.try_into().unwrap();
    i32::from_be_bytes(b)
}

pub fn read_i32_2(b: &[u8]) -> i32 {
    let mut ret = [0; 4];
    for i in 0..4 {
        ret[i] = b[i]
    }
    i32::from_be_bytes(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn read_i32_from_bytes_try_into(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0i32..=10000 {
            for b in i.to_be_bytes().iter() {
                v.push(*b);
            }
        }
        b.iter(move || {
            for i in 0..=10000 {
                read_i32(&v[i * 4..(i + 1) * 4]);
            }
        });
    }

    #[bench]
    fn read_i32_from_bytes_for_loop(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0i32..=10000 {
            for b in i.to_be_bytes().iter() {
                v.push(*b);
            }
        }
        b.iter(move || {
            for i in 0..=10000 {
                read_i32_2(&v[i * 4..(i + 1) * 4]);
            }
        });
    }
}
