fn main() {
    {
        // using stack memory
        let s = "hello";
        println!("{}", s);
    } // s is dropped here
    {
        // using heap memory
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    } // s is dropped here

    {
        // moving data
        let s1 = String::from("hello");
        let s2 = s1; // s1 is moved to s2 (not a shallow copy!!!)
        // println!("{}, world!", s1); // error: use of moved value: `s1`
        println!("{}, world!", s2);
    }

    {
        // cloning data
        let mut s1 = String::from("hello");
        let s2 = s1.clone(); // s1 is cloned to s2 (deep copy, costly)
        s1.push_str(", world!"); // s1 is still valid
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        // copying data
        let x = "hello";
        let y = x; // x is copied to y (immutable data can be copied)
        println!("x = {}, y = {}", x, y); // x is still valid

        let tup = (1, 2, 3);
        let dup = tup; // tup is copied to dup (tuple is immutable)
        println!("tup = {:?}, dup = {:?}", tup, dup); // tup is still valid
    }

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
    // println!("{}", s) // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
    println!("{}", x); // but i32 is Copy, so it's okay to still use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    // println!("{}", s2); // error: use of moved value: `s2`
    println!("{}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}