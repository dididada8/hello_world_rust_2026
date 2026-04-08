use helloworld::print_line_separator;
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
}
fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
