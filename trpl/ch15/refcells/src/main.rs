use std::rc::Rc;
use std::cell::RefCell;
use crate::List::*;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("Before");
    println!("a : {:?}", a);
    println!("b : {:?}", b);
    println!("c : {:?}", c);

    *value.borrow_mut() += 10;
    println!("After");
    println!("a : {:?}", a);
    println!("b : {:?}", b);
    println!("c : {:?}", c);
}
