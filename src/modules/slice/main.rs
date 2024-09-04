fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

pub fn get_first_word() {
    let s: String = String::from("Hello world");
    let word: &str = first_word(&s);
    println!("{word}");
}
