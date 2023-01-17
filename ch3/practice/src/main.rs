mod temperature;
mod fibonacci;
mod christmas;

use std::io::{self, Write};
fn main() {
    println!("Rust CLI");
    loop {
        println!("1. Temperature Converter");
        println!("2. Fibonacci Sequence");
        println!("3. Christmas Tree");
        println!("4. Exit");
        print!("Select: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = match input.trim().parse::<u8>() {
            Ok(input) => input,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match input {
            1 => temperature::interface(),
            2 => fibonacci::interface(),
            3 => christmas::twelve_days_of_christmas(),
            4 => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}
