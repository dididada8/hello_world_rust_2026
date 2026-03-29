use helloworld::print_line_separator;
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
fn main() {
    demo_1();
    print_line_separator();
}
