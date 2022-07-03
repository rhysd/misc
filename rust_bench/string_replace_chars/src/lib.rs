pub fn replace_chars_direct(s: &mut String, from: char, to: char) {
    let mut buf = [0; 4];
    let to = to.encode_utf8(&mut buf);
    let mut i = 0;
    while let Some(c) = s[i..].chars().next() {
        if c == from {
            s.replace_range(i..i + 1, to);
        }
        i += c.len_utf8();
    }
}

pub fn replace_chars_direct_ascii(mut s: &mut str, from: u8, to: u8) {
    let from = from as char;
    while let Some(c) = s.chars().next() {
        if c == from {
            let bytes = unsafe { s.as_bytes_mut() };
            bytes[0] = to;
        }
        s = &mut s[1..];
    }
}

pub fn replace_chars_copied(s: &str, from: char, to: char) -> String {
    let mut buf = [0; 4];
    let to = to.encode_utf8(&mut buf);
    s.replace(from, to)
}
