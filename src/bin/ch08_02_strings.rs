use helloworld::print_type_of;

fn main() {
    // data 是 &str 类型（字符串切片）
    // &str 是对字符串数据的引用/借用，没有所有权
    // 字符串字面量存储在程序的二进制文件中（静态内存）
    // &str 不可变且固定大小
    let data = "initial contents";
    print_type_of(&data, Some("data"));

    // s 是 String 类型（拥有所有权的字符串）
    // String 是堆分配的、可增长的字符串类型
    // to_string() 方法将 &str 转换为 String
    let s = data.to_string();
    print_type_of(&s, Some("s"));

    // 该方法也可以直接在字面量上使用：
    let s = "initial contents".to_string();
    print_type_of(&s, Some("s"));

    let mut s = String::from(data);
    s.push_str(", but now also");
    print_type_of(&s, Some("s"));


}
