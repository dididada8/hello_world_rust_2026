use helloworld::{print_line_separator, print_type_of};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn demo_1() {
    let user = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user_json = serde_json::to_string_pretty(&user).expect("serialize user to json");
    println!("user: {user_json}");

    println!();
    let user1 = &mut User {
        username: "".to_string(),
        email: "".to_string(),
        sign_in_count: 0,
        active: false,
    };
    user1.username = String::from("someusername123");
    user1.sign_in_count = 2;

    let user_json = serde_json::to_string_pretty(&user1).expect("serialize user to json");
    println!("user1: {user_json}");

    println!();
    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let user_json = serde_json::to_string_pretty(&user2).expect("serialize user to json");
    println!("user2: {user_json}");
}

fn demo_2() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // println!("user1.username: {}", user1.username); // error: value borrowed here after move 所有权已经转移到 user2
    println!("user2.username: {}", user2.username);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // println!("user1.username: {}", user1.username); // error: value borrowed here after move 所有权已经转移到 user2
    println!("user2.username: {}", user2.username);
}


#[derive(Serialize)]
struct Color(i32, i32, i32);
#[derive(Serialize)]
struct Point(i32, i32, i32);
#[derive(Serialize)]
struct Rectangle {
    color: Color,
    point: Point,
}

fn demo_3() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let rect = Rectangle {
        color: black,
        point: origin,
    };
    let rect_json = serde_json::to_string_pretty(&rect).expect("serialize rect to json");
    println!("rect: {rect_json}");
}

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    demo_1();
    print_line_separator();

    demo_2();
    print_line_separator();

    demo_3();
    print_line_separator();

    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);
    print_type_of(&subject, Some("subject"));
}
