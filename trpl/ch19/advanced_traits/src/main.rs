use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct GenericPoint<T, U> {
    x: T,
    y: U,
}
impl<T: Add<Output = T>, U: Add<Output = U>> Add for GenericPoint<T, U> {
    type Output = GenericPoint<T, U>;
    fn add(self, rhs: Self) -> Self::Output {
        GenericPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait Pilot {fn fly(&self);}
trait Wizard {fn fly(&self);}
struct Human;
impl Pilot for Human {fn fly(&self) {println!("This is your captain speaking.");}}
impl Wizard for Human {fn fly(&self) {println!("I am a Quidditch master.");}}
impl Human {fn fly(&self) {println!("*waving arms furiously*");}}

trait Animal {fn baby_name() -> String;}
struct Dog;
impl Dog {fn baby_name() -> String {String::from("Spot")}}
impl Animal for Dog {fn baby_name() -> String {String::from("puppy")}}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
impl OutlinePrint for Wrapper {}

fn main() {
    assert_eq!(Point { x: 1, y: 2 } + Point { x: 3, y: 4 }, Point { x: 4, y: 6 });
    assert_eq!(
        GenericPoint { x: 1 as u8, y: 2 as i8 } + GenericPoint { x: 3 as u8, y: 4 as i8 },
        GenericPoint { x: 4 as u8, y: 6 as i8 }
    );
    assert_eq!(Millimeters(1) + Meters(1), Millimeters(1001));

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog as an animal is called a {}", <Dog as Animal>::baby_name());

    let p = Point { x: 1, y: 2 };
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    w.outline_print();
}
