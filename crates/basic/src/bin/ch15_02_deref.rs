use helloworld::{print_line_separator, print_type_of};
use std::ops::{Deref, DerefMut};

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
    // 为 MyBox<T> 实现 Deref trait（注意：定义在 struct 之前，Rust 允许这样写）
    impl<T> Deref for MyBox<T> {
        type Target = T; // 解引用的目标类型是 T
        fn deref(&self) -> &T {
            // 返回元组结构体第一个字段的引用
            &self.0
        }
    }

    // 元组结构体，包裹一个 T 类型的值
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // m 的类型是 MyBox<String>
    let m = MyBox::new(String::from("Rust"));

    // Deref Coercion（自动解引用强制转换）：
    // 编译器看到 hello 需要 &str，但收到 &MyBox<String>，于是自动调用两步：
    //   1. MyBox<String> 的 deref() → &String
    //   2. String 的 deref() → &str
    hello(&m);

    // 手动解引用：*m 等价于 *(m.deref())，只解一层，得到内部的 String 值
    // 与 hello(&m) 的区别：这里只解引用一次，停在 String，不会继续到 &str
    println!("{}", *m);

    print_type_of(&m, Some("demo_3:myBox"));
}
/*
处理可变引用的解引用强制转换：
类似于用 Deref trait 覆盖不可变引用上的 * 运算符，
可以使用 DerefMut trait 覆盖可变引用上的 * 运算符。

Rust 在找到类型和 trait 实现的三种情况下进行解引用强制转换：
  1. 当 T: Deref<Target=U> 时，从 &T 到 &U（不可变 → 不可变）
  2. 当 T: DerefMut<Target=U> 时，从 &mut T 到 &mut U（可变 → 可变）
  3. 当 T: Deref<Target=U> 时，从 &mut T 到 &U（可变 → 不可变）

第三种情况：可变引用可以强制转换为不可变引用，但反过来不行。
原因：将不可变引用转为可变引用要求该不可变引用是数据的唯一引用，
借用规则无法保证这一点，所以 Rust 禁止此操作。
 */
fn demo_4() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 实现 Deref（不可变解引用），供情况1和情况3使用
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    // 实现 DerefMut（可变解引用），供情况2使用
    // 前提：必须已实现 Deref trait
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    // --- 情况1：&T → &U（不可变 → 不可变）---
    // hello 需要 &str，传入 &MyBox<String>
    // 自动强制转换：&MyBox<String> → &String → &str
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> 自动 deref 到 &str

    // --- 情况2：&mut T → &mut U（可变 → 可变）---
    // modify 需要 &mut String，传入 &mut MyBox<String>
    // 自动强制转换：&mut MyBox<String> → &mut String
    fn modify(s: &mut String) {
        s.push_str(" is awesome");
    }
    let mut m2 = MyBox::new(String::from("Rust"));
    modify(&mut m2); // &mut MyBox<String> 自动 deref_mut 到 &mut String
    println!("{}", *m2); // 输出：Rust is awesome

    // --- 情况3：&mut T → &U（可变 → 不可变）---
    // print_len 需要 &str，传入 &mut MyBox<String>
    // 可变引用可以强制转换为不可变引用，反之不行（借用规则限制）
    fn print_len(s: &str) {
        println!("length: {}", s.len());
    }
    let mut m3 = MyBox::new(String::from("Rust"));
    print_len(&mut m3); // &mut MyBox<String> → &String → &str（可变转不可变）

    print_type_of(&m, Some("demo_4:myBox"));
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
