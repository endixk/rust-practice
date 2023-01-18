fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    let w = vec![1, 2, 3];
    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v, w);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(2);
    println!("The third element is {:?}", third);
    let does_not_exist: Option<&i32> = v.get(100);
    println!("The 100th element is {:?}", does_not_exist);

    // invalid access
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    v.push(6);
    // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("The first element is: {}", first);

    // iteration
    let v = vec![100, 32, 57];
    for i in &v {
        print!("{} ", i);
    }
    println!();

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        print!("{} ", i);
    }
    println!();

    // enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}

fn strings() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");
    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2); // s2 is not moved
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved
    // println!("{}", s1); // error: borrow of moved value: `s1`
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // s1 is moved
    println!("{}", s);
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3); // s1 is not moved
    println!("{} {}", s, s1);

    // indexing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s); // prints Зд
    // error: byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
    // println!("{}", &hello[0..1]);

    // iteration
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!();
}

use std::collections::HashMap;
fn maps() {
    /* HashMap */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);

    let mut also_scores = HashMap::new();
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    also_scores.insert(&teams[0], &initial_scores[0]);
    also_scores.insert(&teams[1], &initial_scores[1]);
    println!("also_scores = {:?}", also_scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let even_also_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("even_also_scores = {:?}", even_also_scores);

    // assert_eq!(scores, also_scores); // error: mismatched types
    assert_eq!(also_scores, even_also_scores);

    /* Ownership */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("field_name = {}", field_name); // error: use of moved value: `field_name`

    /* Accessing values */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* Updating a HashMap */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // not inserted
    println!("{:?}", scores);

    /* Updating a value based on the old value */
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    vectors();
    println!("{}", "-".repeat(20));
    strings();
    println!("{}", "-".repeat(20));
    maps();
}
