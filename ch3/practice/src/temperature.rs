use std::io::{self, Write};

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

pub fn interface() {
    println!("Temperature Converter");

    loop {
        println!("1. Celcius to Fahrenheit");
        println!("2. Fahrenheit to Celcius");
        println!("3. Exit");
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
            1 => {
                loop {
                    print!("Celcius: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = match input.trim().parse::<f64>() {
                        Ok(input) => input,
                        Err(_) => {
                            println!("Invalid input!");
                            continue;
                        }
                    };

                    println!("Fahrenheit: {}", celcius_to_fahrenheit(input));
                    break;
                }
            },
            2 => {
                loop {
                    print!("Fahrenheit: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = match input.trim().parse::<f64>() {
                        Ok(input) => input,
                        Err(_) => {
                            println!("Invalid input!");
                            continue;
                        }
                    };

                    println!("Celcius: {}", fahrenheit_to_celcius(input));
                    break;
                }
            },
            3 => break,
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}