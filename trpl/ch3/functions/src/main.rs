/* functions basic */
fn main1() {
    println!("Hello, world!");
    another_function1();
}
fn another_function1() {
    println!("Another function.");
}

/* functions with parameters */
fn main2() {
    another_function2(5);
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

/* functions with multiple parameters */
fn main3() {
    another_function3(5, 6);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/* functions and expressions */
fn main4() {
    // let x = (let y = 6); // error
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The values of x, y are: {}, {}", x, y);
}

/* functions with return values */
fn five() -> i32 {5}
fn main5(){
    let x = five();
    println!("The value of x is: {}", x);
}

fn main6(){
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main(){
    main1();
    main2();
    main3();
    main4();
    main5();
    main6();
}