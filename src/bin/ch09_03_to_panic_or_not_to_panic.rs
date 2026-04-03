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
fn main() {
    demo_1();
    println!();
    demo_2();
}
