use helloworld::{print_line_separator, print_type_of};
use std::collections::HashMap;

fn main() {
    // Rust 类型推断是双向的：编译器会分析整个作用域来推断类型
    // 虽然这里 HashMap::new() 没有指定泛型参数，但编译器会向后查看
    // 根据第12行的 insert 调用推断出 HashMap<String, i32> 类型
    // 类型在编译时完全确定，不是运行时动态类型
    let mut scores = HashMap::new();
    print_type_of(&scores, Some("scores map"));

    scores.insert(String::from("Blue"), 10); //调用推断出 HashMap<String, i32> 类型
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    print_line_separator();

    let team_name = String::from("Blue");
    // get 方法返回 Option<&V>，这里是 Option<&i32>
    // 因为 HashMap 的值类型是 i32，所以 get 返回 Option<&i32>
    // 需要使用 match 或 if let 来处理 Option
    match scores.get(&team_name) {
        Some(score) => println!("Score for {}: {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }

    let value = scores.get(&team_name).copied().unwrap_or(-1); // 复制值并提供默认值
    if value < 0 {
        println!("No score found for {}", team_name);
    } else {
        println!("Score for {}: {}", team_name, value);
    }


}
