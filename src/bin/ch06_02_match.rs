use strum::IntoEnumIterator; // 提供 `Coin::iter()` 这个关联函数（来自 trait 扩展）
use strum_macros::EnumIter;
use helloworld::print_line_separator;
// 派生宏：为 enum 生成“遍历所有变体”的实现代码

fn demo_1() {
    #[derive(Copy, Clone, EnumIter)] // 加新变体后无需改遍历函数，`Coin::iter()` 会自动包含新变体
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => {
                println!("Lucky nickel!");
                5
            }
            Coin::Dime => {
                println!("Lucky dime!");
                10
            }
            Coin::Quarter => {
                println!("Lucky quarter!");
                25
            }
        }
    }

    impl Coin {
        fn value_in_cents(&self) -> u8 {
            // `self` 的类型是 `&Coin`（不可变借用），`*self` 是解引用，把 `&Coin` 取成 `Coin`。
            // 这不是显式 `clone()`：因为 `Coin` 实现了 `Copy`，这里会发生一次隐式按位复制。
            // 因此可以把复制出的 `Coin` 传给按值参数：`fn value_in_cents(coin: Coin) -> u8`。
            value_in_cents(*self)
        }
    }

    println!(
        "value_in_cents(Coin::Quarter) = {}",
        value_in_cents(Coin::Quarter)
    );
    let penny = Coin::Penny;
    println!("penny.value_in_cents() = {}", penny.value_in_cents());

    let coin = Coin::Dime;
    println!("coin.value_in_cents() = {}", coin.value_in_cents());

    let three_quarters = Coin::Quarter;
    println!(
        "three_quarters.value_in_cents() = {}",
        three_quarters.value_in_cents()
    );

    let nickel = Coin::Nickel;
    println!("nickel.value_in_cents() = {}", nickel.value_in_cents());
    println!();
    // 自动迭代：这里不是手写数组，而是使用 `EnumIter` 生成的迭代器按声明顺序产出每个变体。
    // 因此当你在 `Coin` 中新增变体时，这个循环会自动遍历到，不需要再维护 `iter_coins()`。
    for coin in Coin::iter() {
        println!("iter coin value_in_cents() = {}", coin.value_in_cents());
    }
}
fn demo_2(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // `{:?}` 是 Debug 格式化占位符：要求对应参数实现 `std::fmt::Debug`。
    // `Option<i32>` 已实现 Debug，所以会打印成 `Some(5)`、`Some(6)`、`None` 这种开发者可读形式。
    // 对比 `{}`（Display）：`{}` 面向用户展示，很多类型（如 `Option<i32>`）默认并不实现 Display。
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}

fn demo_3(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // `_` 是通配符模式（wildcard pattern），表示“匹配所有前面分支没匹配到的值”。
        // 这里等价于“除了 3 和 7 以外的任何点数都执行 reroll()”。
        // 它不会把值绑定到变量，因此不能在分支体里再读取具体点数；好处是可明确表达“其余情况忽略具体值”。
        // 在 `match` 里常用 `_` 做兜底分支，保证模式匹配是穷尽的（exhaustive）。
        _ => reroll(),
    }

    fn add_fancy_hat() {
        println!("Adding a fancy hat!");
    }
    fn remove_fancy_hat() {
        println!("Removing the fancy hat!");
    }
    fn reroll() {
        println!("Rerolling the dice!");
    }


}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
