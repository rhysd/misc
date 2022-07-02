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

pub fn replace_chars_copied(s: &str, from: char, to: char) -> String {
    let mut buf = [0; 4];
    let to = to.encode_utf8(&mut buf);
    s.replace(from, to)
}
