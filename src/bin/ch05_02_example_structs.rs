use helloworld::print_line_separator;

fn demo_1() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("area is {}", rect1.width * rect1.height);
}


fn main() {
    demo_1();
    print_line_separator();
    another_demo();
}


fn another_demo() {
    let base = 10;
    // 1) 内部函数：不能捕获外部变量 `base`
    fn add_fn(x: i32) -> i32 {
        // x + base // 这里会报错：cannot find value `base` in this scope
        x + 1
    }
    // 2) 闭包：可以捕获外部变量 `base`
    let add_closure = |x: i32| x + base;

    println!("add_fn(5) = {}", add_fn(5)); // 6
    println!("add_closure(5) = {}", add_closure(5)); // 15
}
