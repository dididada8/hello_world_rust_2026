use std::cmp::Ordering;
use std::net::IpAddr;
use std::{io, process};

// demo_1: 使用 match 表达式处理 Result，错误时提前返回
// - parse() 返回 Result<IpAddr, AddrParseError>
// - 使用 match 模式匹配处理成功和失败两种情况
// - Ok(ip) 分支：解析成功，将 IpAddr 赋值给 home
// - Err(e) 分支：解析失败，打印错误信息并 return 提前退出函数
// - 注意：由于 Err 分支使用了 return，后续的 println! 不会执行
fn demo_1() {
    let ip_str = "127.0.0.11x"; // 无效的 IP 地址字符串（多了字符 'x'）
    let home = match ip_str.parse::<IpAddr>() {
        Ok(ip) => ip, // 分支1: 返回 ip 给 home
        Err(e) => {
            eprintln!("解析 IP 地址失败: {}", e);
            return; // 分支2: 退出整个函数 提前退出函数，后续代码不执行
        }
    };
    println!("home: {}", home.is_ipv4()); // 只有解析成功才会执行
}

// demo_2: 使用 unwrap_or_else 提供默认值
// - unwrap_or_else: Result 的方法，失败时执行闭包并返回默认值
// - 与 demo_1 的区别：
//   * demo_1: 错误时提前 return，后续代码不执行
//   * demo_2: 错误时使用默认值，后续代码继续执行
// - 适用场景：需要提供合理的默认值，程序能继续运行
fn demo_2() {
    let ip_str = "127.0.0.11x"; // 无效的 IP 地址
    let home = ip_str.parse::<IpAddr>().unwrap_or_else(|e| {
        eprintln!("解析 IP 地址失败: {}", e);
        // unwrap() 详解：
        // - "127.0.0.1".parse() 返回 Result<IpAddr, AddrParseError>
        // - unwrap() 的作用：
        //   * 如果是 Ok(ip)  => 返回 ip（IpAddr 类型）
        //   * 如果是 Err(e)  => panic! 程序崩溃
        // - 这里可以安全使用 unwrap()，因为 "127.0.0.1" 是硬编码的有效 IP
        // - 如果这里失败了，说明程序逻辑有严重错误，应该崩溃
        "127.0.0.1".parse().unwrap() // 提供默认值：localhost
    });
    println!("home: {}", home.is_ipv4()); // 无论解析成败，都会执行
}

// demo_3: 先获取 Result，再使用 if let 处理错误
// - 与 demo_1 的区别：
//   * demo_1: 在 match 表达式中直接处理并赋值给 home
//   * demo_3: 先将 Result 存储在 result 变量中，单独处理错误，最后 unwrap 获取值
// - if let Err(e) = result: 只匹配错误情况
// - 如果是 Err，打印错误并 return
// - 如果是 Ok，继续执行，然后 unwrap 获取值
// - 这种方式更清晰地展示了 Result 的存在
fn demo_3() {
    let ip_str = "127.0.0.1x"; // 无效的 IP 地址
    let result = ip_str.parse::<IpAddr>(); // 先获取 Result

    if let Err(e) = result {
        eprintln!("解析 IP 地址失败: {}", e);
        return; // 错误时提前退出
    }
    let home = result.unwrap(); // 此时确定是 Ok，可以安全 unwrap
    println!("home: {}", home.is_ipv4());
}

// demo_4: 猜数字游戏 - 综合错误处理示例
// 展示了多种错误处理技巧：
// 1. 使用 match 处理用户输入的解析错误
// 2. 优化：先 trim() 一次，避免重复调用
// 3. 处理特殊输入："quit" 退出程序
// 4. 输入验证：检查数字范围
// 5. 使用 continue 跳过本次循环，继续下一次输入
fn demo_4() {
    println!("猜数字！");

    let secret_number = rand::random_range(1..=100); // 生成 1-100 的随机数

    loop {
        // --snip--

        println!("请输入你的猜测。");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        // 优化点：先 trim 一次，避免后续重复调用
        let guess = guess.trim();

        // 尝试将字符串解析为 i32
        let guess: i32 = match guess.parse() {
            Ok(num) => num, // 解析成功，返回数字
            Err(msg) => {
                // 解析失败，检查是否是退出指令
                if guess == "quit" {
                    process::exit(0); // 正常退出程序
                }
                println!("请输入一个数字，或者输入 'quit' 退出。");
                continue; // 跳过本次循环，继续下一次输入
            }
        };

        // 验证输入范围
        if guess < 1 || guess > 100 {
            println!("秘密数字将在 1 和 100 之间。");
            continue; // 范围错误，重新输入
        }

        // 比较猜测值与秘密数字
        match guess.cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break; // 猜对了，退出循环
            }
        }
    }
}

fn main() {
    demo_1();
    println!();
    demo_2();
    println!();
    demo_3();
    println!();
    demo_4();
}
