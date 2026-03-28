fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    println!("1 + 2 = {}", add(1, 2));
    println!("2 - 1 = {}", sub(2, 1));

    print_labeled_measurement(5, 'h');

    println!();
    
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
