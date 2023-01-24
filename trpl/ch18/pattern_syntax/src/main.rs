fn main() {
    // matching literals
    println!("{}", &format!("> {} {}", "matching literals", "-".repeat(60)).as_str()[..60]);
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    // matching named variables
    println!("{}", &format!("\n> {} {}", "matching named variables", "-".repeat(60)).as_str()[..60]);
    let x = Some(5);
    let y = 10;
    match x {
        Some(60) => println!("Got 60"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}, y = {:?}", x, y),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // multiple patterns
    println!("{}", &format!("\n> {} {}", "multiple patterns", "-".repeat(60)).as_str()[..60]);
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with `..`
    println!("{}", &format!("\n> {} {}", "matching ranges of values with `..`", "-".repeat(60)).as_str()[..60]);
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring structs
    println!("{}", &format!("\n> {} {}", "destructuring structs", "-".repeat(60)).as_str()[..60]);
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("a = {}, b = {}, p = {:?}", a, b, p);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    println!("x = {}, y = {}, p = {:?}", x, y, p);

    let p = Point { x: 7, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // destructuring enums
    println!("{}", &format!("\n> {} {}", "destructuring enums", "-".repeat(60)).as_str()[..60]);
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs = vec![
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(0, 160, 255)];
    for msg in msgs {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x, y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r, g, b
                );
            }
        }
    }

    // destructuring nested structs and enums
    println!("{}", &format!("\n> {} {}", "destructuring nested structs and enums", "-".repeat(60)).as_str()[..60]);
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2 {
        ChangeColor(Color),
    }
    let msgs = vec![
        Message2::ChangeColor(Color::Hsv(0, 160, 255)),
        Message2::ChangeColor(Color::Rgb(0, 160, 255)),
    ];
    for msg in msgs {
        match msg {
            Message2::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r, g, b
                );
            }
            Message2::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h, s, v
                );
            }
        }
    }

    // destructuring references
    println!("{}", &format!("\n> {} {}", "destructuring references", "-".repeat(60)).as_str()[..60]);
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    println!("sum_of_squares = {}", sum_of_squares);

    // destructuring structs and tuples
    println!("{}", &format!("\n> {} {}", "destructuring structs and tuples", "-".repeat(60)).as_str()[..60]);
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);

    // ignoring an entire value with `_`
    println!("{}", &format!("\n> {} {}", "ignoring an entire value with `_`", "-".repeat(60)).as_str()[..60]);
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // ignoring parts of a value with a nested _
    println!("{}", &format!("\n> {} {}", "ignoring parts of a value with a nested `_`", "-".repeat(60)).as_str()[..60]);
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    // ignoring an unused variable by starting its name with _
    println!("{}", &format!("\n> {} {}", "ignoring an unused variable by starting its name with `_`", "-".repeat(60)).as_str()[..60]);
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s); // s is moved to _s, even though _s is not used
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s); // s is still valid

    // ignoring remaining parts of a value with `..`
    println!("{}", &format!("\n> {} {}", "ignoring remaining parts of a value with `..`", "-".repeat(60)).as_str()[..60]);
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    // ref and ref mut to create references in patterns
    println!("{}", &format!("\n> {} {}", "ref and ref mut to create references in patterns", "-".repeat(60)).as_str()[..60]);
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name); // robot_name is still valid
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name); // robot_name has been changed

    // extra conditionals with match guards
    println!("{}", &format!("\n> {} {}", "extra conditionals with match guards", "-".repeat(60)).as_str()[..60]);
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 5;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    println!("{}", &format!("\n> {} {}", "@ bindings", "-".repeat(60)).as_str()[..60]);
    enum Hello { Hello { id: i32 } }
    let msg = Hello::Hello { id: 5 };
    match msg {
        Hello::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Hello::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Hello::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
