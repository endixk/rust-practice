fn main() {
    let mut s = String::from("Hello, world!");
    println!("{}", s);

    let word = first_word_len(&s);
    println!("Length of the first word: {}", word);

    s.clear();
    // `word` is now totally invalid!

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);

    let word = first_word(&s);
    println!("First word: {}", word);

    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("First word: {}", word);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("First word: {}", word);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("First word: {}", word);
    let word = first_word(my_string_literal);
    println!("First word: {}", word);
}

fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
