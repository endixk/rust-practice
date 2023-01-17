fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world");
    let r2 = &mut s;
    r2.push_str("!");
    // println!("{}, {}", r1, r2); // error: cannot borrow `s` as mutable more than once at a time
    println!("{}", r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(", world");
    }
    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", r2);

    let mut s = String::from("hello");
    s.push_str(", world");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3); // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, and {}", r1, r2);

    println!("{}", no_dangle());
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// cannot borrow `*some_string` as mutable, as it is behind a `&` reference
/*
fn change(some_string: &String) {
    some_string.push_str(", world");
} */

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
} */
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}