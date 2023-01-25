#[macro_export]
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

use macros::HelloMacro;
use macros_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    Pancakes::hello_macro();
}
