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
    #[derive(Debug)]
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

fn demo_3() {
    #[derive(Debug, Copy, Clone)] // 这样我们可以在一会儿检查状态
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                // -- snip --
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn describe_state_quarter(coin: Coin) -> Option<String> {
        if let Coin::Quarter(state) = coin {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old, for America!"))
            } else {
                Some(format!("{state:?} is relatively new."))
            }
        } else {
            None
        }
    }

    // `if let PATTERN = EXPR { ... }` 是“只关心一个匹配分支”的简写语法。
    // 这里的 `PATTERN` 是 `Some(desc)`，表示只有当右侧表达式结果是 `Option::Some(...)` 时才进入代码块。
    // `desc` 是模式绑定变量：会把 `Some` 里面的字符串解包并绑定到 `desc`，作用域仅在 `{ ... }` 内。
    // 若结果是 `None`，条件不成立，整个 `if let` 直接跳过（可选地再写 `else` 处理其他情况）。
    // 右侧 `describe_state_quarter(Coin::Quarter(UsState::Alaska))` 会先构造参数再调用函数：
    // `Coin::Quarter(UsState::Alaska)` 是“带关联数据的枚举变体构造”，把州信息装进 `Coin::Quarter`。
    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }

    println!();

    let all_coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ];
    for coin in all_coins {
        if let Some(desc) = describe_state_quarter(coin) {
            println!("{desc}");
        } else {
            println!("NONE {coin:?}");
        }
    }
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
