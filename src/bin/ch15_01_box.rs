use helloworld::print_type_of;

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    print_type_of(&b,Some("box demo"));
}