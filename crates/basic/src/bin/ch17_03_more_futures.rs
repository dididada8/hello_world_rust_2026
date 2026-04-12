use helloworld::print_line_separator;

fn slow(name: &str, duration: std::time::Duration) {
    println!("  '{name}' is slowing down for {}ms", duration.as_millis());
    std::thread::sleep(duration);
}

fn demo_1() {
    trpl::block_on(async {
        let fut1 = async {
            println!("'a' started.");
            slow("a", std::time::Duration::from_millis(30));
            println!("'a' ran for 30ms");
            slow("a", std::time::Duration::from_millis(10));
            println!("'a' ran for 10ms");
            slow("a", std::time::Duration::from_millis(20));
            println!("'a' ran for 20ms");
            println!("'a' finished.");
        };

        let fut2 = async {
            println!("'b' started.");
            slow("b", std::time::Duration::from_millis(75));
            println!("'b' ran for 75ms");
            slow("b", std::time::Duration::from_millis(10));
            println!("'b' ran for 10ms");
            slow("b", std::time::Duration::from_millis(15));
            println!("'b' ran for 15ms");
            slow("b", std::time::Duration::from_millis(350));
            println!("'b' ran for 350ms");
        };

        trpl::select(fut1, fut2).await;
    });
}

fn demo_2() {
    trpl::block_on(async {
        let fut1 = async {
            println!("'a' started.");
            trpl::sleep(std::time::Duration::from_millis(30)).await;
            println!("'a' ran for 30ms");
            trpl::sleep(std::time::Duration::from_millis(10)).await;
            println!("'a' ran for 10ms");
            trpl::sleep(std::time::Duration::from_millis(20)).await;
            println!("'a' ran for 20ms");
            println!("'a' finished.");
        };

        let fut2 = async {
            println!("'b' started.");
            trpl::sleep(std::time::Duration::from_millis(75)).await;
            println!("'b' ran for 75ms");
            trpl::sleep(std::time::Duration::from_millis(10)).await;
            println!("'b' ran for 10ms");
            trpl::sleep(std::time::Duration::from_millis(15)).await;
            println!("'b' ran for 15ms");
            trpl::sleep(std::time::Duration::from_millis(350)).await;
            println!("'b' ran for 350ms");
        };

        trpl::select(fut1, fut2).await;
    });
}

fn main() {
    println!("=== demo_1: 使用阻塞 sleep，fut2 无法启动 ===");
    demo_1();
    print_line_separator();

    println!("=== demo_2: 使用异步 sleep，fut1 和 fut2 交替执行 ===");
    demo_2();
    print_line_separator();
}
