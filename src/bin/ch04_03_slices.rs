use helloworld::print_line_separator;

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
     
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
}
