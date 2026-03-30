use helloworld::print_line_separator;

fn demo_1() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        // `_` 是通配符模式：匹配所有未被前面分支匹配到的情况（这里就是 `None`）。
        // `=>` 读作“匹配后执行”，左边是模式，右边是该分支表达式。
        // `()` 是单元值（unit），表示“什么都不做”；因此这个分支仅用于占位兜底。
        _ => (),
    }

    println!();
    let config_max = Some(225u8);
    //将 if let 视为 match 的语法糖，它在值匹配一个模式时运行代码，然后忽略所有其他值。
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn demo_2() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    println!("There are {} coins remaining.", count);

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("There are {} coins remaining.", count);
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
