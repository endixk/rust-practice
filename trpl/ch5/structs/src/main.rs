struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);

    let user2 = build_user("a@b.c".to_string(), "abc".to_string());
    print_user(&user2);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    print_user(&user2);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("name: {}\nemail: {}\nactive: {}\nsign_in_count: {}\n", user.username, user.email, user.active, user.sign_in_count);
}