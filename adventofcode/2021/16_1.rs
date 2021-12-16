use std::io;

fn parse_bits(input: &[bool], bits: usize) -> Option<(&[bool], usize)> {
    if input.len() < bits {
        return None;
    }
    let mut v = 0;
    for b in input.iter().take(bits) {
        v = (v << 1) + *b as usize;
    }
    Some((&input[bits..], v))
}

fn parse_lit(mut input: &[bool]) -> &[bool] {
    loop {
        let b = input[0];
        input = &input[5..]; // continue bit and 4 bits for literal value
        if !b {
            return input;
        }
    }
}

fn parse_sub(input: &[bool]) -> Option<(&[bool], usize)> {
    let (b, input) = input.split_first().unwrap();
    if *b {
        // the length is a 11-bit number representing the number of sub-packets
        let mut total = 0;
        let (mut input, len) = parse_bits(input, 11)?;
        for _ in 0..len {
            let (i, v) = parse(input).unwrap();
            total += v;
            input = i;
        }
        Some((input, total))
    } else {
        // the length is a 15-bit number representing the number of bits in the sub-packets
        let (input, bits) = parse_bits(input, 15)?;
        let v = parse_all(&input[..bits]);
        Some((&input[bits..], v))
    }
}

fn parse(input: &[bool]) -> Option<(&[bool], usize)> {
    let (input, version) = parse_bits(input, 3)?;
    let (input, id) = parse_bits(input, 3)?;
    let (input, child) = if id == 4 {
        (parse_lit(input), 0)
    } else {
        parse_sub(input)?
    };
    Some((input, version + child))
}

fn parse_all(mut input: &[bool]) -> usize {
    let mut total = 0;
    loop {
        if let Some((i, v)) = parse(input) {
            input = i;
            total += v;
        } else {
            return total;
        }
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
    println!("{}", parse_all(&bin));
}
