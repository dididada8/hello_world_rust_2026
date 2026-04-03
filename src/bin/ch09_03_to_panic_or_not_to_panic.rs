use std::cmp::Ordering;
use std::io;
use std::net::IpAddr;

fn demo_1() {
    let ip_str = "127.0.0.11x";
    let home = match ip_str.parse::<IpAddr>() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("解析 IP 地址失败: {}", e);
            return;
        }
    };
    println!("home: {}", home.is_ipv4());
}

fn demo_2() {
    let ip_str = "127.0.0.11x";
    let home = ip_str.parse::<IpAddr>().unwrap_or_else(|e| {
        eprintln!("解析 IP 地址失败: {}", e);
        "127.0.0.1".parse().unwrap()
    });
    println!("home: {}", home.is_ipv4());
}

fn demo_3() {
    let ip_str = "127.0.0.1x";
    let result = ip_str.parse::<IpAddr>();

    if let Err(e) = result {
        eprintln!("解析 IP 地址失败: {}", e);
        return;
    }
    let home = result.unwrap();
    println!("home: {}", home.is_ipv4());
}

fn demo_4() {
    println!("猜数字！");

    let secret_number = rand::random_range(1..=100);

    loop {
        // --snip--

        println!("请输入你的猜测。");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("秘密数字将在 1 和 100 之间。");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
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
