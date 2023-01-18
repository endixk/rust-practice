enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("routing V4"),
        IpAddrKind::V6 => println!("routing V6"),
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => println!("{}", s),
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(s) => println!("Write {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    home.print();
    loopback.print();

    let msgs = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
        ];
    for msg in msgs.iter() {
        msg.call();
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{:?} {:?} {:?}", some_number, some_string, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}
