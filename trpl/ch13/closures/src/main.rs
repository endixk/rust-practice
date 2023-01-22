use std::thread;
use std::time::Duration;

/*
fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
} */

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

struct Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Clone+Eq+Hash,
          V: Clone {
    calculation: T,
    map: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Clone+Eq+Hash,
          V: Clone {
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.map.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.map.insert(arg, v.clone());
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_strings() {
        let mut c = Cacher::new(|a: &str| a.to_string());

        let v1 = c.value("1");
        let v2 = c.value("2");

        assert_eq!(v1, "1".to_string());
        assert_eq!(v2, String::from("2"));
    }

    #[test]
    fn call_with_integers() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn catch_local_variables() {
        let x = 4;
        let equal_to_x = |z| z == x;

        let y = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn catch_local_vectors() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;

        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}