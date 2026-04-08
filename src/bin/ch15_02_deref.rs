use helloworld::{print_line_separator, print_type_of};
use std::ops::Deref;

fn demo_1() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/*
Rust 将 * 运算符替换为对 deref 方法的调用，然后进行一次普通的解引用，这样我们就不必考虑是否需要调用 deref 方法。
这个 Rust 特性让我们编写无论我们有常规引用还是实现
Deref 的类型都能相同工作的代码。
deref 方法返回对值的引用，并且 *(y.deref()) 中括号外的普通解引用仍然是必要的，这与所有权系统有关。
如果 deref 方法直接返回值而不是对值的引用，该值将从 self 中移出。在这种情况下或在大多数情况下使用解引用运算符时，
我们不想取得 MyBox<T> 内部值的所有权。
 */
fn demo_2() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 为 MyBox<T> 实现 Deref trait
    // impl<T>: 这是泛型参数声明，表示这个实现适用于任何类型 T
    // Deref for MyBox<T>: 为 MyBox<T> 类型实现 Deref trait
    impl<T> Deref for MyBox<T> {
        // type Target = T: 这是 Deref trait 的关联类型（associated type）
        // 关联类型定义了解引用操作返回的目标类型
        // 这里指定解引用 MyBox<T> 会得到类型 T 的引用
        // 例如：MyBox<i32> 解引用后得到 &i32
        type Target = T;

        // deref 方法：定义解引用的具体行为
        // &self: 接收 MyBox 的不可变引用
        // -> &Self::Target: 返回关联类型 Target 的引用，即 &T
        // Self::Target 就是上面定义的 type Target = T
        fn deref(&self) -> &Self::Target {
            // &self.0: 返回元组结构体第一个字段的引用
            // MyBox(T) 是元组结构体，self.0 访问其内部存储的 T 类型值
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    print_type_of(&y, Some("demo_2:myBox"));
}
fn demo_3() {
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    println!("{}", *m);
    print_type_of(&m, Some("demo_3:myBox"));
}
fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
