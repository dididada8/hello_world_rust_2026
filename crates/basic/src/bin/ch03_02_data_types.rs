use std::io;
use helloworld::print_type_of;
fn main() {
    let a: i32 = 42; // 32-bit signed integer
    let b: f64 = 3.14; // 64-bit floating-point number
    let c: bool = true; // boolean value
    let d: char = 'R'; // Unicode scalar value (character)
    let e: &str = "Hello, Rust!"; // string slice (string literal)

    println!("Integer (i32): {a}");
    println!("Floating-point (f64): {b}");
    println!("Boolean (bool): {c}");
    println!("Character (char): {d}");
    println!("String slice (&str): {e}");
    println!();

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}");

    let n1 = 98_222;
    let n2 = 0xff;
    let n3 = 0o77;
    let n4 = 0b1111_0000;
    let n5 = b'A';
    print_type_of(&n1, Some(&format!("n1 ({n1}) type:")));
    print_type_of(&n2, Some(&format!("n2 ({n2}) type:")));
    print_type_of(&n3, Some(&format!("n3 ({n3}) type:")));
    print_type_of(&n4, Some(&format!("n4 ({n4}) type:")));
    print_type_of(&n5, Some(&format!("n5 ({n5}) type:")));

    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);


    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}


