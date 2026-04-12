use helloworld::print_line_separator;

fn slow(duration: std::time::Duration) {
    std::thread::sleep(duration);
}

fn demo_1() {
    trpl::block_on(async {
        let fut1 = async {
            println!("'a' started.");
            slow(std::time::Duration::from_millis(30));
            println!("'a' ran for 30ms");
            slow(std::time::Duration::from_millis(10));
            println!("'a' ran for 10ms");
            slow(std::time::Duration::from_millis(20));
            println!("'a' ran for 20ms");
            println!("'a' finished.");
        };

        let fut2 = async {
            println!("'b' started.");
            slow(std::time::Duration::from_millis(75));
            println!("'b' ran for 75ms");
            slow(std::time::Duration::from_millis(10));
            println!("'b' ran for 10ms");
            slow(std::time::Duration::from_millis(15));
            println!("'b' ran for 15ms");
            slow(std::time::Duration::from_millis(350));
            println!("'b' ran for 350ms");
        };

        trpl::select(fut1, fut2).await;
    });
}

fn main() {
    demo_1();
    print_line_separator();
}
