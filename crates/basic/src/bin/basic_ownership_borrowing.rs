use helloworld::print_type_of;
fn main() {
    {
        let s = String::from("hello"); // s 是所有者
        let len = s.len(); // 通过 s 的引用获取长度
        println!("The length of '{}' is {}.", s, len);
    } // s 离开作用域，内存被释放

    // String: 可增长、可变的字符串（堆上）
    let mut s = String::from("hello");
    s.push_str(", world!");

    // &str: 字符串切片（通常是栈上的引用）
    let literal: &str = "hello";
    let slice: &str = &s[0..5];

    println!("s = {}", s);
    println!("literal = {}", literal);
    println!("slice = {}，&slice = {}", slice, &slice);
    println!(
        "slice ptr = {:p}, &slice ptr = {:p}",
        slice.as_ptr(),
        &slice
    );
    print_type_of(&slice, Some("slice"));
}
