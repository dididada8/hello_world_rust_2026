fn demo_1() {
    #[derive(Copy, Clone)]
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
    println!();
    let coin = Coin::Dime;
    println!("coin.value_in_cents() = {}", coin.value_in_cents());
    println!();
    let three_quarters = Coin::Quarter;
    println!(
        "three_quarters.value_in_cents() = {}",
        three_quarters.value_in_cents()
    );
    println!();
    let nickel = Coin::Nickel;
    println!("nickel.value_in_cents() = {}", nickel.value_in_cents());
}

fn main() {
    demo_1();
}
