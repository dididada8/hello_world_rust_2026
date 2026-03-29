fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    let mut s = String::from("hello");

    change(&mut s);
    println!("The string is now: {}", s);

    let r1 = &s;
    // let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    let r2 = &s;
    
    println!("{r1}, {r2}");
}

fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
} // 这里，s 离开作用域。但因为 s 没有它所引用内容的所有权，
// 所以 String 不会被丢弃。

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
