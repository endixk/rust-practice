struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::{Deref, DerefMut};
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn add_bar_and_hello(s: &mut String) {
    s.push_str("bar");
    hello(s);
}

fn main() {
    // deref coercion: immut -> immut
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // deref coercion: mut -> mut
    let mut m = MyBox::new(String::from("Rust"));
    add_bar_and_hello(&mut m);

    // deref coercion: mut -> immut
    let mut m = MyBox::new(String::from("Rust"));
    m.push_str("baz");
    hello(&m);

    // deref coercion: immut -> mut (not allowed)
    // let m = MyBox::new(String::from("Rust"));
    // add_bar_and_hello(&mut m);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dereference_operator() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn dereferencing_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn dereferencing_mybox(){
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}