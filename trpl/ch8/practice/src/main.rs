extern crate rand;
use rand::Rng;
mod stats;
mod pig_latin;
mod employees;

fn main() {
    // generate a vector of size 100 with random numbers
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..100 {
        v.push(rand::thread_rng().gen_range(1..=30));
    }
    stats::stats(&v);

    println!("{}", "-".repeat(30));

    let lorip = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    println!("Original : {}", lorip);
    println!("Pig Latin: {}", pig_latin::pig_latin(lorip));

    println!("{}", "-".repeat(30));

    employees::interface();
}
