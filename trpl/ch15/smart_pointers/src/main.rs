#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

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

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::rc::Rc;
#[derive(Debug)]
enum List2 { // Rc<List2>
    Cons2(i32, Rc<List2>),
    Nil2,
}

use std::cell::RefCell;
#[derive(Debug)]
enum List3 { // Rc<RefCell<i32>>, Rc<List3>
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

#[derive(Debug)]
enum List4 { // RefCell<Rc<List4>>
    Cons4(i32, RefCell<Rc<List4>>),
    Nil4,
}

use crate::List4::*;
impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            Cons4(_, item) => Some(item),
            Nil4 => None,
        }
    }
}

use std::rc::Weak;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    println!("{}", &String::from(format!("{} {}", "> Box<T>", "-".repeat(50)))[..50]);
    let b = Box::new(5);
    println!("b = {}", b);

    use List::*;
    let list =
        Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));
    println!("list = {:?}", list);

    println!("{}", &String::from(format!("{} {}", "> Deref", "-".repeat(50)))[..50]);
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

    println!("{}", &String::from(format!("{} {}", "> Drop", "-".repeat(50)))[..50]);
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created: {:?}, {:?}", c, d);
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    println!("{}", &String::from(format!("{} {}", "> Rc<T>", "-".repeat(50)))[..50]);
    use List2::*;
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("b : {:?}", b);
        println!("c : {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("{}", &String::from(format!("{} {}", "> RefCell<T>", "-".repeat(50)))[..50]);
    use List3::*;
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    let b = Cons3(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("Before");
    println!("a : {:?}", a);
    println!("b : {:?}", b);
    println!("c : {:?}", c);

    *value.borrow_mut() += 10;
    println!("After");
    println!("a : {:?}", a);
    println!("b : {:?}", b);
    println!("c : {:?}", c);

    println!("{}", &String::from(format!("{} {}", "> Reference cycles", "-".repeat(50)))[..50]);
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing b = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    // println!("a next item = {:?}", a.tail()); // infinite cycle

    println!("{}", &String::from(format!("{} {}", "> Weak<T>", "-".repeat(50)))[..50]);

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf value = {}", leaf.value);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf children = {:?}", leaf.children.borrow());

    println!("{}", &String::from(format!("{} {}", "> End of main", "-".repeat(50)))[..50]);
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