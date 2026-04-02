use std::collections::HashMap;
use helloworld::print_type_of;

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

}