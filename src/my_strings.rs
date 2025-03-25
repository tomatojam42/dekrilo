pub fn my_strings(file: &[u8]) -> Vec<&str> {
    let is_printable = |c: &u8| -> bool {
        (32..=126).contains(c) || (9..=10).contains(c) || c == &13
    };
    let mut s: Vec<&str> = Vec::new();
    let mut counter: usize = 0;
    let l = file.len();
    for ch in 0..l {
        let chr = file[ch];
        let printable = is_printable(&chr);
        if printable {
            counter += 1
        }
        if (chr==0 && counter!=0) || ch == l-1 {
            s.push(
                std::str::from_utf8(&file[ch-counter..ch]).unwrap() );
        }
        if ! printable { counter = 0; }
    } s
}