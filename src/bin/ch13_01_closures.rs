use helloworld::print_line_separator;
use rand::{RngExt, rng};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Yellow,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Option::map 的用法：
        // - 如果 user_preference 是 Some(color)，则执行闭包 |color| self.check_stock(&color)
        //   将 Some(color) 转换为 Some(bool)
        // - 如果 user_preference 是 None，则直接返回 None，不执行闭包
        // - unwrap_or(false) 会在结果为 None 时返回 false，为 Some(bool) 时返回 bool 值
        if user_preference
            .map(|color| self.check_stock(&color))
            .unwrap_or(false)
        {
            return user_preference.unwrap();
        }
        self.most_stocked()
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_colors_map = std::collections::HashMap::new();
        for color in &self.shirts {
            *num_colors_map.entry(*color).or_insert(0) += 1;
        }
        let (color, _) = num_colors_map
            .iter()
            .max_by_key(|(_, count)| *count)
            .unwrap();
        *color
    }

    fn check_stock(&self, color: &ShirtColor) -> bool {
        for shirt in &self.shirts {
            if shirt == color {
                return true;
            }
        }
        println!("No more {:?} shirts!", color);
        false
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包的返回类型 -> u32 可以省略，Rust 编译器能够自动推断
    // 可以简写为: let expensive_closure = |num: u32| { ... };
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn demo_1() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    assert_eq!(inventory.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);

    let user_pref1 = Some(ShirtColor::Yellow);
    let giveaway1 = inventory.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = inventory.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn demo_2() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn demo_3() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x| x + 1;

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); // 错误：类型不匹配，因为 example_closure 已经被推断为接受 String 类型的参数

    println!(
        "s = {}, add_one_v1={}, add_one_v2={}, add_one_v3={},add_one_v4={}",
        s,
        add_one_v1(1),
        add_one_v2(2),
        add_one_v3(3),
        add_one_v4(4)
    );
}

fn demo_4() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    println!();

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || {
        let num = rng().random_range(1..=100);
        if list.contains(&num) {
            return;
        }
        list.push(num);
    };
    borrows_mutably();
    borrows_mutably();
    borrows_mutably();

    println!("After calling closure: {list:?}");
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
    print_line_separator();
    demo_4();
}
