use std::env;
use std::io::{self, BufRead};

fn part1(mut lines: impl Iterator<Item = String>) {
    let mut disk = vec![];

    for (i, size) in lines.next().unwrap().chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        let block = (i % 2 == 0).then_some(i / 2);
        for _ in 0..size {
            disk.push(block);
        }
    }

    let mut start = 0;
    let mut end = disk.len() - 1;
    loop {
        while start < disk.len() && disk[start].is_some() {
            start += 1;
        }
        while disk[end].is_none() {
            end -= 1;
        }
        if end < start {
            break;
        }
        disk.swap(start, end);
    }

    let checksum: usize = disk.into_iter().enumerate().flat_map(|(i, b)| b.map(|id| id * i)).sum();
    println!("{checksum}");
}

struct Span {
    offset: usize,
    size: usize,
}

struct File {
    offset: usize,
    size: usize,
    id: usize,
}

fn part2(mut lines: impl Iterator<Item = String>) {
    let mut spans = vec![];
    let mut files = vec![];

    let mut offset = 0;
    for (i, size) in lines.next().unwrap().chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        let size = size as _;
        if i % 2 == 0 {
            files.push(File { offset, size, id: i / 2 })
        } else {
            spans.push(Span { offset, size });
        }
        offset += size;
    }

    for mut span in spans {
        let mut files = files.iter_mut().rev();
        while let Some(file) = files.find(|f| f.size <= span.size && span.offset < f.offset) {
            file.offset = span.offset;
            span.size -= file.size;
            span.offset += file.size;
            if span.size == 0 {
                break;
            }
        }
    }

    let checksum: usize =
        files.into_iter().map(|f| (f.offset..f.offset + f.size).sum::<usize>() * f.id).sum();
    println!("{checksum}");
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    match env::args().nth(1).as_deref() {
        Some("1") => part1(lines),
        Some("2") => part2(lines),
        x => panic!("invalid argument: {:?}", x),
    }
}
