use std::io;

fn parse_bits(input: &[bool], bits: usize) -> Option<(&[bool], u64)> {
    if input.len() < bits {
        return None;
    }
    let mut v = 0;
    for b in &input[..bits] {
        v = (v << 1) + *b as u64;
    }
    Some((&input[bits..], v))
}

fn parse_lit(mut input: &[bool]) -> Option<(&[bool], u64)> {
    let mut val = 0;
    loop {
        let b = input[0];
        let (i, v) = parse_bits(&input[1..], 4)?;
        val = (val << 4) + v;
        input = i;
        if !b {
            return Some((input, val));
        }
    }
}

fn parse_sub(input: &[bool], pred: fn(u64, u64) -> u64) -> Option<(&[bool], u64)> {
    let (b, input) = input.split_first().unwrap();
    if *b {
        // the length is a 11-bit number representing the number of sub-packets
        let (input, len) = parse_bits(input, 11)?;
        let (mut input, mut ret) = parse(input).unwrap();
        for _ in 0..len - 1 {
            let (i, v) = parse(input).unwrap();
            ret = pred(ret, v);
            input = i;
        }
        Some((input, ret))
    } else {
        // the length is a 15-bit number representing the number of bits in the sub-packets
        let (input, bits) = parse_bits(input, 15)?;
        let (nested, input) = input.split_at(bits as usize);
        let (mut nested, mut ret) = parse(nested)?;
        while let Some((i, v)) = parse(nested) {
            ret = pred(ret, v);
            nested = i;
        }
        Some((input, ret))
    }
}

fn parse(input: &[bool]) -> Option<(&[bool], u64)> {
    let (input, _) = parse_bits(input, 3)?;
    let (input, id) = parse_bits(input, 3)?;
    match id {
        0 => parse_sub(input, |a, b| a + b),
        1 => parse_sub(input, |a, b| a * b),
        2 => parse_sub(input, |a, b| if a < b { a } else { b }),
        3 => parse_sub(input, |a, b| if a > b { a } else { b }),
        4 => parse_lit(input),
        5 => parse_sub(input, |a, b| (a > b) as u64),
        6 => parse_sub(input, |a, b| (a < b) as u64),
        7 => parse_sub(input, |a, b| (a == b) as u64),
        _ => unreachable!(),
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let bin: Vec<_> = line
        .trim()
        .chars()
        .map(|c| {
            let h = c.to_digit(16).unwrap();
            (0..4).rev().map(move |i| h & (1 << i) != 0)
        })
        .flatten()
        .collect();
    let (_, v) = parse(&bin).unwrap();
    println!("{}", v);
}
