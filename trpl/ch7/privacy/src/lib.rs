pub mod outermost {
    pub fn middle_function() {
        println!("called middle_function");
        middle_secret_function();
    }
    fn middle_secret_function() {
        println!("called middle_secret_function");
        inside::inner_function();
    }
    fn middle_other_function() {
        println!("called middle_other_function");
    }

    mod inside {
        pub fn inner_function() {
            println!("called inner_function");
            secret_function();
        }
        fn secret_function() {
            println!("called secret_function");
            crate::outermost::middle_other_function();
        }
    }
}

pub fn try_me() {
    outermost::middle_function();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
