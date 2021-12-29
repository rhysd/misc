#![feature(test)]

extern crate test;

use std::convert::TryInto;
use std::i32;

#[derive(Copy, Clone)]
pub enum V {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

pub fn read_i32_try_into(b: &[u8]) -> i32 {
    let b: [u8; 4] = b.try_into().unwrap();
    i32::from_be_bytes(b)
}

pub fn read_i32_copy_from_slice(b: &[u8]) -> i32 {
    let mut ret = [0; 4];
    ret.copy_from_slice(b);
    i32::from_be_bytes(ret)
}

pub fn read_i32_loop(b: &[u8]) -> i32 {
    let mut ret = [0; 4];
    for i in 0..4 {
        ret[i] = b[i]
    }
    i32::from_be_bytes(ret)
}

pub fn read_i32_try_into_le(b: &[u8]) -> i32 {
    let b: [u8; 4] = b.try_into().unwrap();
    i32::from_le_bytes(b)
}

pub fn read_i32_copy_from_slice_le(b: &[u8]) -> i32 {
    let mut ret = [0; 4];
    ret.copy_from_slice(b);
    i32::from_le_bytes(ret)
}

pub fn read_i32_loop_le(b: &[u8]) -> i32 {
    let mut ret = [0; 4];
    for i in 0..4 {
        ret[i] = b[i]
    }
    i32::from_le_bytes(ret)
}

pub fn read_i32_bits(b: u32) -> i32 {
    b as i32
}

pub fn read_i32_enum_value(v: V) -> i32 {
    match v {
        V::I32(i) => i,
        _ => panic!(),
    }
}

pub fn read_i64_try_into(b: &[u8]) -> i64 {
    let b: [u8; 8] = b.try_into().unwrap();
    i64::from_be_bytes(b)
}

pub fn read_i64_copy_from_slice(b: &[u8]) -> i64 {
    let mut ret = [0; 8];
    ret.copy_from_slice(b);
    i64::from_be_bytes(ret)
}

pub fn read_i64_loop(b: &[u8]) -> i64 {
    let mut ret = [0; 8];
    for i in 0..8 {
        ret[i] = b[i]
    }
    i64::from_be_bytes(ret)
}

pub fn read_i64_try_into_le(b: &[u8]) -> i64 {
    let b: [u8; 8] = b.try_into().unwrap();
    i64::from_le_bytes(b)
}

pub fn read_i64_copy_from_slice_le(b: &[u8]) -> i64 {
    let mut ret = [0; 8];
    ret.copy_from_slice(b);
    i64::from_le_bytes(ret)
}

pub fn read_i64_loop_le(b: &[u8]) -> i64 {
    let mut ret = [0; 8];
    for i in 0..8 {
        ret[i] = b[i]
    }
    i64::from_le_bytes(ret)
}

pub fn read_i64_bits(b: &[u32]) -> i64 {
    let hi = (b[0] as u64) << 32;
    let lo = b[1] as u64;
    (hi | lo) as i64
}

pub fn read_i64_enum_value(v: V) -> i64 {
    match v {
        V::I64(i) => i,
        _ => panic!(),
    }
}

pub fn read_f32_try_into(b: &[u8]) -> f32 {
    let b: [u8; 4] = b.try_into().unwrap();
    f32::from_be_bytes(b)
}

pub fn read_f32_copy_from_slice(b: &[u8]) -> f32 {
    let mut ret = [0; 4];
    ret.copy_from_slice(b);
    f32::from_be_bytes(ret)
}

pub fn read_f32_loop(b: &[u8]) -> f32 {
    let mut ret = [0; 4];
    for i in 0..4 {
        ret[i] = b[i]
    }
    f32::from_be_bytes(ret)
}

pub fn read_f32_try_into_le(b: &[u8]) -> f32 {
    let b: [u8; 4] = b.try_into().unwrap();
    f32::from_le_bytes(b)
}

pub fn read_f32_copy_from_slice_le(b: &[u8]) -> f32 {
    let mut ret = [0; 4];
    ret.copy_from_slice(b);
    f32::from_le_bytes(ret)
}

pub fn read_f32_loop_le(b: &[u8]) -> f32 {
    let mut ret = [0; 4];
    for i in 0..4 {
        ret[i] = b[i]
    }
    f32::from_le_bytes(ret)
}

pub fn read_f32_bits(b: u32) -> f32 {
    f32::from_bits(b)
}

pub fn read_f32_enum_value(v: V) -> f32 {
    match v {
        V::F32(f) => f,
        _ => panic!(),
    }
}

pub fn read_f64_try_into(b: &[u8]) -> f64 {
    let b: [u8; 8] = b.try_into().unwrap();
    f64::from_be_bytes(b)
}

pub fn read_f64_copy_from_slice(b: &[u8]) -> f64 {
    let mut ret = [0; 8];
    ret.copy_from_slice(b);
    f64::from_be_bytes(ret)
}

pub fn read_f64_loop(b: &[u8]) -> f64 {
    let mut ret = [0; 8];
    for i in 0..8 {
        ret[i] = b[i]
    }
    f64::from_be_bytes(ret)
}

pub fn read_f64_try_into_le(b: &[u8]) -> f64 {
    let b: [u8; 8] = b.try_into().unwrap();
    f64::from_le_bytes(b)
}

pub fn read_f64_copy_from_slice_le(b: &[u8]) -> f64 {
    let mut ret = [0; 8];
    ret.copy_from_slice(b);
    f64::from_le_bytes(ret)
}

pub fn read_f64_loop_le(b: &[u8]) -> f64 {
    let mut ret = [0; 8];
    for i in 0..8 {
        ret[i] = b[i]
    }
    f64::from_le_bytes(ret)
}

pub fn read_f64_bits(u: &[u32]) -> f64 {
    let hi = (u[0] as u64) << 32;
    let lo = u[1] as u64;
    f64::from_bits(hi | lo)
}

pub fn read_f64_enum_value(v: V) -> f64 {
    match v {
        V::F64(f) => f,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn read_i32_enum(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            v.push(V::I32(i));
        }
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_enum_value(v[i]), i as i32);
            }
        });
    }

    #[bench]
    fn read_i32_from_bits(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            v.push(i);
        }
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_bits(v[i]), i as i32);
            }
        });
    }

    fn prepare_be() -> Vec<u8> {
        let mut v = vec![];
        for i in 0i32..=10000 {
            for b in i.to_be_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    fn prepare_le() -> Vec<u8> {
        let mut v = vec![];
        for i in 0i32..=10000 {
            for b in i.to_le_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    #[bench]
    fn read_i32_from_bytes_try_into_be(b: &mut Bencher) {
        let v = prepare_be();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_try_into(&v[i * 4..(i + 1) * 4]), i as i32);
            }
        });
    }

    #[bench]
    fn read_i32_from_bytes_copy_from_slice_be(b: &mut Bencher) {
        let v = prepare_be();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_copy_from_slice(&v[i * 4..(i + 1) * 4]), i as i32);
            }
        });
    }

    #[bench]
    fn read_i32_from_bytes_for_loop_be(b: &mut Bencher) {
        let v = prepare_be();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_loop(&v[i * 4..(i + 1) * 4]), i as i32);
            }
        });
    }

    #[bench]
    fn read_i32_from_bytes_try_into_le(b: &mut Bencher) {
        let v = prepare_le();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_try_into_le(&v[i * 4..(i + 1) * 4]), i as i32);
            }
        });
    }

    #[bench]
    fn read_i32_from_bytes_copy_from_slice_le(b: &mut Bencher) {
        let v = prepare_le();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(
                    read_i32_copy_from_slice_le(&v[i * 4..(i + 1) * 4]),
                    i as i32
                );
            }
        });
    }

    #[bench]
    fn read_i32_from_bytes_for_loop_le(b: &mut Bencher) {
        let v = prepare_le();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i32_loop_le(&v[i * 4..(i + 1) * 4]), i as i32);
            }
        });
    }

    #[bench]
    fn read_i64_enum(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            v.push(V::I64(i));
        }
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i64_enum_value(v[i]), i as i64);
            }
        });
    }

    #[bench]
    fn read_i64_from_bits(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0i64..=10000 {
            let u = i as u64;
            v.push((u >> 32) as u32);
            v.push(u as u32);
        }
        b.iter(move || {
            for i in (0..=20000).step_by(2) {
                assert_eq!(read_i64_bits(&v[i..i + 2]), (i / 2) as i64);
            }
        });
    }

    fn prepare_be_64() -> Vec<u8> {
        let mut v = vec![];
        for i in 0i64..=10000 {
            for b in i.to_be_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    fn prepare_le_64() -> Vec<u8> {
        let mut v = vec![];
        for i in 0i64..=10000 {
            for b in i.to_le_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    #[bench]
    fn read_i64_from_bytes_try_into_be(b: &mut Bencher) {
        let v = prepare_be_64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i64_try_into(&v[i * 8..(i + 1) * 8]), i as i64);
            }
        });
    }

    #[bench]
    fn read_i64_from_bytes_copy_from_slice_be(b: &mut Bencher) {
        let v = prepare_be_64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i64_copy_from_slice(&v[i * 8..(i + 1) * 8]), i as i64);
            }
        });
    }

    #[bench]
    fn read_i64_from_bytes_for_loop_be(b: &mut Bencher) {
        let v = prepare_be_64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i64_loop(&v[i * 8..(i + 1) * 8]), i as i64);
            }
        });
    }

    #[bench]
    fn read_i64_from_bytes_try_into_le(b: &mut Bencher) {
        let v = prepare_le_64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i64_try_into_le(&v[i * 8..(i + 1) * 8]), i as i64);
            }
        });
    }

    #[bench]
    fn read_i64_from_bytes_copy_from_slice_le(b: &mut Bencher) {
        let v = prepare_le_64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(
                    read_i64_copy_from_slice_le(&v[i * 8..(i + 1) * 8]),
                    i as i64
                );
            }
        });
    }

    #[bench]
    fn read_i64_from_bytes_for_loop_le(b: &mut Bencher) {
        let v = prepare_le_64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_i64_loop_le(&v[i * 8..(i + 1) * 8]), i as i64);
            }
        });
    }

    #[bench]
    fn read_f32_enum(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            v.push(V::F32(i as f32));
        }
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_enum_value(v[i]), i as f32);
            }
        });
    }

    #[bench]
    fn read_f32_from_bits(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            v.push((i as f32).to_bits());
        }
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_bits(v[i]), i as f32);
            }
        });
    }

    fn prepare_be_f32() -> Vec<u8> {
        let mut v = vec![];
        for i in 0..=10000 {
            for b in (i as f32).to_be_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    fn prepare_le_f32() -> Vec<u8> {
        let mut v = vec![];
        for i in 0..=10000 {
            for b in (i as f32).to_le_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    #[bench]
    fn read_f32_from_bytes_try_into_be(b: &mut Bencher) {
        let v = prepare_be_f32();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_try_into(&v[i * 4..(i + 1) * 4]), i as f32);
            }
        });
    }

    #[bench]
    fn read_f32_from_bytes_copy_from_slice_be(b: &mut Bencher) {
        let v = prepare_be_f32();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_copy_from_slice(&v[i * 4..(i + 1) * 4]), i as f32);
            }
        });
    }

    #[bench]
    fn read_f32_from_bytes_for_loop_be(b: &mut Bencher) {
        let v = prepare_be_f32();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_loop(&v[i * 4..(i + 1) * 4]), i as f32);
            }
        });
    }

    #[bench]
    fn read_f32_from_bytes_try_into_le(b: &mut Bencher) {
        let v = prepare_le_f32();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_try_into_le(&v[i * 4..(i + 1) * 4]), i as f32);
            }
        });
    }

    #[bench]
    fn read_f32_from_bytes_copy_from_slice_le(b: &mut Bencher) {
        let v = prepare_le_f32();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(
                    read_f32_copy_from_slice_le(&v[i * 4..(i + 1) * 4]),
                    i as f32
                );
            }
        });
    }

    #[bench]
    fn read_f32_from_bytes_for_loop_le(b: &mut Bencher) {
        let v = prepare_le_f32();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f32_loop_le(&v[i * 4..(i + 1) * 4]), i as f32);
            }
        });
    }

    #[bench]
    fn read_f64_enum(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            v.push(V::F64(i as f64));
        }
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f64_enum_value(v[i]), i as f64);
            }
        });
    }

    #[bench]
    fn read_f64_from_bits(b: &mut Bencher) {
        let mut v = vec![];
        for i in 0..=10000 {
            let u = (i as f64).to_bits();
            v.push((u >> 32) as u32);
            v.push(u as u32);
        }
        b.iter(move || {
            for i in (0..=20000).step_by(2) {
                assert_eq!(read_f64_bits(&v[i..i + 2]), (i / 2) as f64);
            }
        });
    }

    fn prepare_be_f64() -> Vec<u8> {
        let mut v = vec![];
        for i in 0..=10000 {
            for b in (i as f64).to_be_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    fn prepare_le_f64() -> Vec<u8> {
        let mut v = vec![];
        for i in 0..=10000 {
            for b in (i as f64).to_le_bytes().iter() {
                v.push(*b);
            }
        }
        v
    }

    #[bench]
    fn read_f64_from_bytes_try_into_be(b: &mut Bencher) {
        let v = prepare_be_f64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f64_try_into(&v[i * 8..(i + 1) * 8]), i as f64);
            }
        });
    }

    #[bench]
    fn read_f64_from_bytes_copy_from_slice_be(b: &mut Bencher) {
        let v = prepare_be_f64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f64_copy_from_slice(&v[i * 8..(i + 1) * 8]), i as f64);
            }
        });
    }

    #[bench]
    fn read_f64_from_bytes_for_loop_be(b: &mut Bencher) {
        let v = prepare_be_f64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f64_loop(&v[i * 8..(i + 1) * 8]), i as f64);
            }
        });
    }

    #[bench]
    fn read_f64_from_bytes_try_into_le(b: &mut Bencher) {
        let v = prepare_le_f64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f64_try_into_le(&v[i * 8..(i + 1) * 8]), i as f64);
            }
        });
    }

    #[bench]
    fn read_f64_from_bytes_copy_from_slice_le(b: &mut Bencher) {
        let v = prepare_le_f64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(
                    read_f64_copy_from_slice_le(&v[i * 8..(i + 1) * 8]),
                    i as f64
                );
            }
        });
    }

    #[bench]
    fn read_f64_from_bytes_for_loop_le(b: &mut Bencher) {
        let v = prepare_le_f64();
        b.iter(move || {
            for i in 0..=10000 {
                assert_eq!(read_f64_loop_le(&v[i * 8..(i + 1) * 8]), i as f64);
            }
        });
    }
}
