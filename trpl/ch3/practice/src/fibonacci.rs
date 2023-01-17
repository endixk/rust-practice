use std::io::{self, Write};

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        let mut x: u32 = 1;
        let mut y: u32 = 1;
        for _ in 2..n {
            let t = match x.checked_add(y) {
                Some(t) => t,
                None => {
                    println!("Overflow!");
                    return 0;
                }
            };
            x = y;
            y = t;
        }
        y
    }
}

pub fn interface() {
    println!("Fibonacci Sequence");

    loop {
        println!("1. Print Fibonacci Sequence");
        println!("2. Exit");
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
                    print!("N: ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = match input.trim().parse::<u32>() {
                        Ok(0) => {
                            println!("F_0 = 0");
                            break;
                        },
                        Ok(input) => input,
                        Err(_) => {
                            println!("Invalid input!");
                            continue;
                        }
                    };

                    let res = fibonacci(input);
                    if res > 0 {
                        println!("F_{} = {}", input, res);
                    }
                    break;
                }
            },
            2 => break,
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}