use helloworld::print_line_separator;

fn demo_1() {
    /*
    `first_word` 是「函数变量」：变量里放的是一段可调用逻辑。
    左边类型 `fn(&str) -> usize` 的意思是：
    1) 接收一个 `&str`
    2) 返回一个 `usize`

    右边使用的是闭包语法：`|s: &str| { ... }`
    `|...|` 在 Rust 中专门用于声明闭包参数，作用类似函数定义里的 `( ... )`。
    对比：
    - 普通函数: `fn f(s: &str) -> usize { ... }`
    - 闭包写法: `|s: &str| -> usize { ... }`（返回类型通常可省略）

    为什么这里可以赋给 `fn` 类型？
    因为这个闭包没有捕获外部变量（non-capturing closure），
    Rust 可以把它自动转换为函数指针 `fn(&str) -> usize`。
    */
    let first_word: fn(&str) -> usize = |s: &str| {
        // 先转成字节数组，方便按索引逐个检查字符。
        let bytes = s.as_bytes();
        // 找到第一个空格就返回它的位置索引。
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        // 如果没有空格，整个字符串都是第一个单词，返回总长度。
        s.len()
    };

    let mut s = String::from("hello world");

    let word = first_word(&s); // word 将得到值 5

    s.clear(); // 这将清空 String，使其等于 ""

    // word 仍然有值 5 在这里，但 s 不再有我们可以用值 5 有意义地使用的任何内容，
    // 所以 word 现在完全无效了！
    println!("The first word  is: {word}");

    print_line_separator();

    if s.is_empty() {
        s.push_str("hello world");
    }

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("The first word is: {hello}, the second word is: {world}");

    print_line_separator();

    let s = String::from("hello");

    let len = s.len();

    let slice1 = &s[0..len];
    let slice2 = &s[..];
    if slice1 == slice2 {
        println!("slice1 == slice2, the word is : {}", slice1);
    } else {
        println!("slice1 != slice2 . {}!={}", slice1, slice2);
    }
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn demo_2() {

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String` 的切片，无论是部分还是全部。
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于对 `String` 的引用，这等价于
    // `String` 的全部切片。
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面量的切片，无论是部分还是全部。
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面量*就是*字符串切片，
    // 这也适用，不需要切片语法！
    let word = first_word(my_string_literal);
    println!("The first word is: {}", word);
}
fn main() {
    demo_1();
    print_line_separator();
    println!();
    demo_2();
}
