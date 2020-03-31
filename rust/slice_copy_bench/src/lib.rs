#![feature(test)]
#![cfg(test)]
extern crate test;

use test::Bencher;

fn array_copy_from_slice(b: &[u8]) -> [u8; 4] {
    let mut ret = [0; 4];
    ret.copy_from_slice(b);
    ret
}

fn array_loop(b: &[u8]) -> [u8; 4] {
    let mut ret = [0; 4];
    for i in 0..4 {
        ret[i] = b[i]
    }
    ret
}

fn prepare() -> Vec<u8> {
    let mut v = vec![];
    for _ in 0..=10000 {
        for u in 0..4 {
            v.push(u);
        }
    }
    v
}

#[bench]
fn array_from_bytes_copy_from_slice_le(b: &mut Bencher) {
    let v = prepare();
    b.iter(move || {
        for i in 0..=10000 {
            array_copy_from_slice(&v[i * 4..(i + 1) * 4]);
        }
    });
}

#[bench]
fn array_from_bytes_for_loop_le(b: &mut Bencher) {
    let v = prepare();
    b.iter(move || {
        for i in 0..=10000 {
            array_loop(&v[i * 4..(i + 1) * 4]);
        }
    });
}
