pub fn is_printable (c: &u8) -> bool {
    (32..=126).contains(c) || (9..=10).contains(c) || *c == 13
}

pub fn my_strings(file: &[u8]) -> Vec<&str> {
    file.split(|ch| ! is_printable(ch))
        .filter(|chunk| ! chunk.is_empty())
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect()
}
