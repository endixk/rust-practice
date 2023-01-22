fn main() {
    let mut v1 = vec![1, 2, 3];

    // immutable borrow of v1
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("{}", "-".repeat(20));

    // mutable borrow of v1
    let v1_iter = v1.iter_mut();
    for val in v1_iter {
        *val += 50;
    }
    for val in v1.iter() {
        println!("Got: {}", val);
    }
    println!("{}", "-".repeat(20));

    // move of v1
    let v1_iter = v1.into_iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("{}", "-".repeat(20));

    // println!("Got: {}", v1[0]); // v1 is no longer valid

    let v1 = vec![1, 2, 3];
    let v2 = v1.iter().map(|x| x + 1);
    for val in v2 {
        println!("Got: {}", val);
    }
    println!("{}", "-".repeat(20));


}
