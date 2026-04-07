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
            //判断
            return;
        }
        list.push(num);
    };

    /*
    注意，在 borrows_mutably 闭包的定义和调用之间不再有 println!：
    当 borrows_mutably 被定义时，它捕获了对 list 的可变引用。我们在闭包被调用后不再使用闭包，
    所以可变借用结束。在闭包定义和闭包调用之间，不允许不可变借用来打印，因为当有可变借用时，
    不允许其他借用。
     */
    //println!("Before calling closure: {list:?}");
    borrows_mutably();
    borrows_mutably();
    borrows_mutably();

    println!("After calling closure: {list:?}");
    list.sort();
    for num in &mut list {
        //&mut list 是一个可变引用，允许我们修改 list 中的元素
        *num += 1; // 注意：这里需要解引用 num，因为 num 是 &mut i32 类型
    }
    println!("After modifying list: {list:?}");
}
//让我们简要探讨使用需要 move 关键字的闭包生成一个新线程
fn demo_5() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || {
        println!("From thread: {list:?}");
    })
    .join()
    .unwrap();

    println!();

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("Sorted by width: {list:?}");
    list.sort_by_key(|r| r.height);
    println!("Sorted by height: {list:?}");

    let find_one = list.iter().find(|r| r.width == 11).unwrap_or_else(|| {
        println!("演示FnOnce  Option<T> 上 unwrap_or_else 方法的定义 适用于可以被调用一次的闭包。所有闭包至少实现这个 trait，\
        因为所有闭包都可以被调用。将捕获的值移出其主体的闭包将只实现 FnOnce，而不实现其他 Fn trait，\
        因为它只能被调用一次。");
        &Rectangle {
            width: 0,
            height: 0,
        }
    });
    println!("Find Rectangle with width 10: {:?}", find_one);

    let value = String::from("closure called");

    /*
    let mut sort_operations = vec![];
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
        这段代码会导致编译错误，因为闭包在 sort_by_key 中被多次调用，
        每次调用都会尝试将 value 移动到 sort_operations 中。
     */
    println!("{list:#?}");
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
    print_line_separator();
    demo_4();
    print_line_separator();
    demo_5();
}
