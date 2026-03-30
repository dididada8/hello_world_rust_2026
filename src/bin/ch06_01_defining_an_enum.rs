use helloworld::print_line_separator;

fn demo_1() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home IP: {} ({:?})", home.address, home.kind);
    println!("Loopback IP: {} ({:?})", loopback.address, loopback.kind);
}

fn demo_2() {
    #[allow(dead_code)]//
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to ({}, {})", x, y),
                Message::Write(text) => println!("Write: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to ({}, {}, {})", r, g, b)
                }
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    let move_message = Message::Move { x: 1, y: 2 };
    move_message.call();
    let quit = Message::Quit;
    quit.call();
}

    fn main() { // demo_2 内部的局部函数，仅用于演示 enum 和方法调用
    demo_1();
    print_line_separator();
    demo_2();
}
