type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
    println!("took a long type");
    f();
}
fn returns_long_type() -> Thunk {
    Box::new(|| println!("i am a new thunk!"))
}

use std::fmt;
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

pub fn bar() -> ! {
    panic!("This call never returns.");
    // println!("You will never see this line.");
}

fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let t: Thunk = Box::new(|| println!("i am a thunk!"));
    takes_long_type(t);
    takes_long_type(returns_long_type());

    let quote: Box<str> = "Life is like a box of chocolates.".into();
    println!("quote: {}", quote.chars().map(|x| match x {' ' => '_', _ => x}).collect::<String>());
}

#[test]
#[should_panic]
fn panic_attack(){ bar(); }