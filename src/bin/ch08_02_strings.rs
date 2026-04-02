use helloworld::print_type_of;
use unicode_segmentation::UnicodeSegmentation;

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

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str 接受 &str 参数，只是借用 s2，不会获取所有权
    // 因为 push_str 的签名是：fn push_str(&mut self, string: &str)
    // 参数是 &str（借用），所以 s2 在调用后仍然有效
    s1.push_str(s2);
    println!("s2 is {s2}");
    print_type_of(&s1, Some("s1"));

    println!();

    let mut s = String::from("lo");
    s.push('l');
    print_type_of(&s, Some(&format!("s:{}", s)));

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 已被移动到这里，不能再使用
    println!("s2 is {s2}, s3 is {s3}");
    println!();

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    print_type_of(&s1, Some("s1"));
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");
    println!();

    let hellos = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    print_type_of(&hellos, Some("hellos"));
    let hello_vec = hellos.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    print_type_of(&hello_vec, Some("hello_vec"));

    let mut hello_vec: Vec<String> = Vec::new();
    for s in hellos.iter().cloned() {
        // 方法：先借用iter()，再复制cloned()，不改变所有权
        hello_vec.push(s);
    }

    // 方法：使用 &hellos 借用，s 类型是 &String，调用 clone() 复制，不改变所有权
    for s in &hellos {
        hello_vec.push(s.clone());
    }
    print_type_of(
        &hello_vec,
        Some(&format!(
            "hello_vec with cloned: {} count ! ",
            hello_vec.len().to_string()
        )),
    );
    let hello_vec = Vec::from(hellos);
    print_type_of(&hello_vec, Some("hello_vec"));

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str 签名：fn push_str(&mut self, string: &str)，只借用 s2，不改变所有权
    println!("s2 is {s2}");

    let hello = String::from("Здравствуйте");
    println!(
        "hello is {hello},len {},count {}",
        hello.len(),
        hello.chars().count()
    );
    // 西里尔字母每个占2字节，[0..4]取4字节=2个字符"Зд"
    println!("{}", &hello[0..4]);
    println!();

    /*
       按字符处理用 .chars()
       需要切片用 .get() 返回 Option
       避免直接用 [start..end] 硬切片
    */
    let hello_cn = String::from("张三是一个中国人的名字，你知道吗？");
    println!(
        "hello is {hello_cn},len {},count {}",
        hello_cn.len(),
        hello_cn.chars().count()
    );
    // 中文汉字每个占3字节，[0..3]取3字节=1个字符"张"
    println!("{}", &hello_cn[0..3]);

    let first_char: String = hello_cn.chars().take(1).collect(); // "张"
    let first_two: String = hello_cn.chars().take(2).collect(); // "张三"
    println!("first_char is {first_char},first_two is {first_two}");

    if let Some((idx, _)) = hello_cn.char_indices().nth(1) {
        println!("{}", &hello_cn[..idx]); // "张"，idx=3 是安全边界
    }

    match hello_cn.get(0..3) {
        Some(slice) => println!("{}", slice), // "张"
        None => println!("Invalid byte index"),
    }

    let graphemes: Vec<&str> = hello_cn.graphemes(true).collect();
    println!("{}", graphemes[2]); // "是"，graphemes[2] 是第3个字符"是"
}
