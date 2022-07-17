pub fn align_left(s: &str, width: usize) -> String {
    let mut len = char_len(&s);
    if len >= width {
        return s.to_string();
    }

    let mut r = String::with_capacity(width);
    r.push_str(s);
    while len < width {
        r.push(' ');
        len = len + 1;
    }

    return r;
}

pub fn align_right(s: &str, width: usize) -> String {
    let mut len = char_len(&s);
    if len >= width {
        return s.to_string();
    }

    let mut r = String::with_capacity(width);
    r.push_str(s);
    while len < width {
        r.insert(0, ' ');
        len = len + 1;
    }

    return r;
}

pub fn char_len(s: &str) -> usize {
    let mut len = 0;
    for _ in s.chars() {
        len = len + 1;
    }
    len
}
