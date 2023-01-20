fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic copy version (fast, but only works for types that implement Copy)
fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic clone version (slower, but works with non-copy types)
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}

// generic reference version
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::fmt::{self, Debug, Display};

struct Point<T: Display> {
    x: T,
    y: T,
}

impl<T: Display> Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl<T: Display> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct PointVar<T: Display, U: Display> {
    x: T,
    y: U,
}

impl<T, U> Debug for PointVar<T, U> where T: Display, U: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl<T, U> PointVar<T, U> where T: Display, U: Display {
    fn mixup_move<V, W>(self, other: PointVar<V, W>) -> PointVar<T, W>
        where V: Display, W: Display {
        PointVar {
            x: self.x,
            y: other.y,
        }
    }

    fn mixup_copy<V, W>(self, other: &PointVar<V, W>) -> PointVar<T, W>
    where V: Display, W: Display+Copy {
        PointVar {
            x: self.x,
            y: (*other).y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("The largest number is {}", largest_i32(&number_list));
    println!("The largest char is {}", largest_char(&char_list));

    // generic versions
    println!("Generic copy: The largest number is {}", largest_copy(&number_list));
    println!("Generic copy: The largest char is {}", largest_copy(&char_list));
    println!("Generic clone: The largest number is {}", largest_clone(&number_list));
    println!("Generic clone: The largest char is {}", largest_clone(&char_list));
    println!("Generic ref: The largest number is {}", largest_ref(&number_list));
    println!("Generic ref: The largest char is {}", largest_ref(&char_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    let integer_and_float = PointVar { x: 5, y: 4.0 };
    let integer_and_char = PointVar { x: 5, y: 'a' };

    println!("integer = {:?}", integer);
    println!("float = {:?}", float);
    println!("integer_and_float = {:?}", integer_and_float);
    println!("integer_and_char = {:?}", integer_and_char);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointVar { x: 5, y: 10.4 };
    let p2 = PointVar { x: "Hello", y: 'c' };
    let p3 = p1.mixup_move(p2);
    println!("p3 = {:?}", p3);
    // println!("p2 = {:?}", p2); // error: p2 is moved

    let p1 = PointVar { x: 5, y: 10.4 };
    let p2 = PointVar { x: "Hello", y: 'c' };
    let p3 = p1.mixup_copy(&p2);
    println!("p3 = {:?}", p3);
    println!("p2 = {:?}", p2); // p2 is not moved


}
