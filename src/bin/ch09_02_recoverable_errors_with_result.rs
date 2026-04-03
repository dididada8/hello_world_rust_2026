use helloworld::{print_line_separator, process_result};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::panic;
use std::{fs, io};

// 基础错误处理：使用嵌套 match 处理多种错误情况
// 展示了如何根据错误类型进行不同的处理逻辑
fn demo_1() {
    // 示例1：嵌套 match 模式 - 处理不同类型的错误
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file, // 文件打开成功，直接使用
        Err(error) => match error.kind() {
            // 失败时，进一步匹配错误类型
            // error.kind() 返回 io::ErrorKind 枚举，匹配具体的错误类型
            // 如果文件不存在，尝试创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,                                         // 文件创建成功
                Err(e) => panic!("Problem creating the file: {e:?}"), // 创建失败则 panic
            },
            _ => {
                // 其他类型的错误（如权限不足、磁盘满等）
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    println!("{greeting_file:?}");
    print_line_separator();

    // 示例2：使用闭包简化嵌套 match - 功能与示例1相同，代码更简洁
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

    // 示例3：优雅的错误处理 - 不使用 panic，返回 Option 类型
    // 使用 match 处理错误而不是 panic，这样程序可以继续运行
    // 成功返回 Some(file)，失败返回 None
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

// 错误传播模式：使用 match 手动传播错误
// 这是 ? 运算符出现之前的传统写法，代码冗长但逻辑清晰
fn demo_2() {
    fn read_username_from_file() -> Result<String, io::Error> {
        // 第一步：尝试打开文件
        let username_file_result = File::open("hello2.txt");

        // 使用 match 处理打开文件的结果
        let mut username_file = match username_file_result {
            Ok(file) => file,        // 成功：解包得到文件句柄
            Err(e) => return Err(e), // 失败：提前返回错误，将错误向上传播
        };

        let mut username = String::new();

        // 第二步：尝试读取文件内容到字符串
        match username_file.read_to_string(&mut username) {
            Ok(_) => {
                // 成功：Ok 包含读取的字节数，这里用 _ 忽略
                println!("inner fn -> {username:?}");
                Ok(username) // 返回成功结果，包含读取的用户名
            }
            Err(e) => Err(e), // 失败：返回错误，将错误向上传播
        }
    }
    let username = read_username_from_file().expect("Unable to get username");
    println!("{username:?}");
}

// ? 运算符：错误传播的简写语法
// 作用：如果 Result 是 Err，则提前返回该 Err；如果是 Ok(value)，则解包得到 value
// 语法：expression? 等价于 match expression { Ok(val) => val, Err(e) => return Err(e) }
// 要求：只能用在返回 Result 或 Option 类型的函数中
// 优势：简化错误处理代码，避免大量嵌套 match
fn demo_3() {
    fn read_username_from_file() -> Result<String, io::Error> {
        // File::open("hello2.txt")?
        // - 如果成功，解包 Ok(file) 得到 file，赋值给 username_file
        // - 如果失败，立即 return Err(e)，后续代码不执行
        let mut username_file = File::open("hello2.txt")?;
        let mut username = String::new();
        // read_to_string(&mut username)?
        // - 如果成功，返回读取的字节数（被忽略）
        // - 如果失败，立即 return Err(e)
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    let username = read_username_from_file().expect("Unable to get username");
    println!("{username:?}");
}

// 链式调用：? 运算符的高级用法
// 对比 demo_3：demo_3 需要中间变量 username_file，demo_4 直接链式调用
// 关键优化：File::open("hello2.txt")?.read_to_string(&mut username)?
// 链式调用原理：
// 1. File::open("hello2.txt")? 返回 File 对象（如果成功）
// 2. 直接在返回的 File 对象上调用 .read_to_string(&mut username)?
// 3. 省略了中间变量，代码更简洁
// 为什么可以链式调用？
// - File::open() 返回 Result<File, Error>
// - ? 运算符解包得到 File
// - File 实现了 Read trait，有 read_to_string 方法
// - 方法可以直接在表达式结果上调用
fn demo_4() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        // 链式调用：省略中间变量 username_file
        // 等价于 demo_3 的两行代码：
        // let mut username_file = File::open("hello2.txt")?;
        // username_file.read_to_string(&mut username)?;
        File::open("hello2.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
    let username = read_username_from_file().expect("Unable to get username");
    println!("{username:?}");
}


// 最简洁的写法：使用标准库的便捷函数
// 对比前面所有 demo：
// - demo_2: 手动 match 传播错误（冗长）
// - demo_3: 使用 ? 运算符，但需要打开文件和读取两步
// - demo_4: 链式调用，仍需两步操作
// - demo_5: 一行搞定！使用 fs::read_to_string 直接读取文件内容
//
// fs::read_to_string 内部实现：
// 1. 打开文件
// 2. 创建 String
// 3. 读取所有内容到 String
// 4. 返回 Result<String, io::Error>
//
// 优势：
// - 代码最简洁（只需1行）
// - 自动处理所有细节（打开、读取、关闭）
// - 标准库提供，无需自己实现
// - 适用于小文件的一次性读取
//
// 注意：对于大文件，应使用 BufReader 流式读取以节省内存
fn demo_5() {
    fn read_username_from_file() -> Result<String, io::Error> {
        // 一行代码完成所有操作：打开文件 + 读取内容 + 返回 Result
        // 完全等价于 demo_3/demo_4 的所有逻辑，但更简洁
        fs::read_to_string("hello.txt")
    }
    let username = read_username_from_file().expect("Unable to get username");
    println!("{username:?}");
}

fn main() {
    println!("=== demo_1 ===");
    demo_1();

    print_line_separator();
    println!();

    println!("=== demo_2 ===");
    // 使用 catch_unwind 捕获 panic，让程序可以继续执行
    // catch_unwind 返回 Result：Ok(返回值) 或 Err(panic 信息)
    let result = panic::catch_unwind(|| {
        demo_2();
    });

    process_result(result, Some("demo_2"));

    print_line_separator();
    println!();

    println!("=== demo_3 ===");
    // 使用 catch_unwind 捕获 panic，让程序可以继续执行
    // catch_unwind 返回 Result：Ok(返回值) 或 Err(panic 信息)
    let result = panic::catch_unwind(|| {
        demo_3();
    });
    process_result(result, Some("demo_3"));

    print_line_separator();
    println!();

    println!("=== demo_4 ===");
    // 使用 catch_unwind 捕获 panic，让程序可以继续执行
    // catch_unwind 返回 Result：Ok(返回值) 或 Err(panic 信息)
    let result = panic::catch_unwind(|| {
        demo_4();
    });
    process_result(result, Some("demo_4"));

    print_line_separator();
    println!();

    println!("=== demo_5 ===");
    let result = panic::catch_unwind(|| {
        demo_5();
    });
    process_result(result, Some("demo_5"));
    print_line_separator();
    println!();
}
