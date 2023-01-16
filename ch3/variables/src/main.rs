fn main() {
    /* variables */
    // let x = 5; // error: immutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    /* shadowing */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    /* data types */
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let fx = 2.0; // f64
    let fy: f32 = 3.0; // f32
    println!("The value of fx is: {}", fx);
    println!("The value of fy is: {}", fy);

    /* numeric operations */
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}", sum, difference, product, quotient, remainder);

    /* boolean */
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {}, f: {}", t, f);

    /* character */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    /* compound types */
    /* tuple */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of x, y, z are: {}, {}, {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The values of five_hundred, six_point_four, one are: {}, {}, {}", five_hundred, six_point_four, one);

    /* array */
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The values of first, second are: {}, {}", first, second);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let jan = months[0];
    let feb = months[1];
    println!("The values of jan, feb are: {}, {}", jan, feb);

    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
