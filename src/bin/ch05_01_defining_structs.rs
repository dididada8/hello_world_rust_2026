use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user_json = serde_json::to_string_pretty(&user).expect("serialize user to json");
    println!("{user_json}");
}
