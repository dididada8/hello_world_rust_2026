use helloworld::print_line_separator;
use std::fs::File;
use std::io::ErrorKind;

fn demo_1() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // error.kind() 返回 io::ErrorKind 枚举，匹配具体的错误类型
            // 如果文件不存在，尝试创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    println!("{greeting_file:?}");
    print_line_separator();

    // 闭包语法：|参数| { 闭包体 }，类似匿名函数
    // unwrap_or_else 接受一个闭包，当 Result 是 Err 时执行
    // |error| 是闭包参数，error 类型由编译器自动推断为 io::Error
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            // 嵌套闭包：内层闭包处理文件创建失败的情况
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    println!("{greeting_file:?}");

    print_line_separator();

    // 使用 match 处理错误而不是 panic，这样程序可以继续运行
    let greeting_file = match File::open("hello1.txt") {
        Ok(file) => {
            println!("{file:?}");
            Some(file)
        }
        Err(e) => {
            println!("File not found: {e}");
            None
        }
    };
    println!("{greeting_file:?}");
}

fn main() {
    demo_1();
    print_line_separator();


}
