// demo_1: 没有使用泛型的版本 - 代码重复问题
// 问题：largest_i32 和 largest_char 的逻辑完全相同，只是类型不同
// 这违反了 DRY（Don't Repeat Yourself）原则
fn demo_1() {
    // 查找 i32 切片中的最大值
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 查找 char 切片中的最大值
    // 注意：这个函数和 largest_i32 的逻辑完全相同！
    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("{:?} 最大的数字是 {result}", number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("{:?} 最大的字符是 {result}", char_list);
}

// demo_2: 使用泛型解决 demo_1 的代码重复问题
// 优势：一个函数可以处理多种类型（i32, char, f64 等）
fn demo_2() {
    // 错误示范（已注释）：没有 trait 约束的泛型函数
    // fn largest<T>(list: &[T]) -> &T {
    //     // 编译错误！不是所有 T 都支持 > 运算符
    // }

    // ========== 泛型函数签名详解 ==========
    // fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T
    //
    // 【1】<T: std::cmp::PartialOrd> - 泛型类型参数 + trait 约束
    //      - T: 泛型类型参数（Type parameter），代表任意类型
    //      - : std::cmp::PartialOrd - trait 约束（trait bound）
    //      - 含义：T 必须实现 PartialOrd trait（支持偏序比较）
    //      - 为什么需要：因为函数体中使用了 > 运算符（第 67 行）
    //
    // 【2】(list: &[T]) - 参数
    //      - &[T]: T 类型元素的切片引用
    //      - 可以接受 &Vec<i32>、&[char]、&[f64] 等任何实现了 PartialOrd 的类型
    //
    // 【3】-> &T - 返回类型
    //      - 返回 T 类型的引用
    //      - 避免所有权转移和数据复制
    //
    // 【PartialOrd trait 说明】
    //      - 提供偏序比较：<, >, <=, >=
    //      - 基本类型（i32, f64, char 等）都自动实现了
    //      - 与 Ord 的区别：PartialOrd 允许不可比较的情况（如 f64::NAN）
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {  // 需要 PartialOrd trait 才能使用 >
                largest = item;
            }
        }

        largest
    }

    // 测试 i32 类型
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);  // T 推断为 i32
    println!("最大的数字是 {result}");

    // 测试 char 类型 - 同一个函数！
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);  // T 推断为 char
    println!("最大的字符是 {result}");
}

// demo_3: 泛型结构体和方法
fn demo_3() {
    // 泛型结构体定义
    // struct Point<T> 表示：Point 是一个泛型结构体，T 是类型参数
    // 注意：x 和 y 必须是同一类型 T
    struct Point<T> {
        x: T,
        y: T,
    }

    // ========== 为泛型结构体实现方法 ==========
    // impl<T: std::fmt::Debug> Point<T>
    //
    // 【1】impl<T: std::fmt::Debug> - 泛型实现块
    //      - T: 类型参数
    //      - : std::fmt::Debug - trait 约束
    //      - 含义：只有当 T 实现了 Debug trait 时，才能使用这个 impl 块中的方法
    //
    // 【2】Point<T> - 为哪个类型实现方法
    //      - 为 Point<T> 结构体实现方法
    //      - T 必须和 impl 中声明的 T 一致
    impl<T: std::fmt::Debug> Point<T> {
        pub fn print(&self) {
            // 使用 {:?} Debug 格式化，因为不是所有 T 都实现了 Display
            // 由于 impl 有 Debug 约束，这里可以安全使用 {:?}
            println!("x: {:?}, y: {:?}", self.x, self.y);
        }
    }

    // 创建不同类型的 Point 实例
    let integer = Point { x: 5, y: 10 };        // Point<i32>
    let float = Point { x: 1.0, y: 4.0 };       // Point<f64>
    let char_point = Point { x: 'a', y: 'b' };  // Point<char>

    // ========== 重要概念：泛型单态化（Monomorphization）==========
    // Point<i32>、Point<f64>、Point<char> 是三个完全不同的类型！
    // 编译时，Rust 会为每个具体类型生成独立的代码
    //
    // 因此，以下代码会报错：
    // let all = vec![integer, float, char_point];     // 错误！类型不一致
    // let all = [integer, float, char_point];         // 改成数组也会错误！
    //
    // 解决方案：
    // 1. 只存储同一类型：vec![integer]
    // 2. 使用 trait object：Vec<Box<dyn SomeTrait>>
    // 3. 使用 enum 包装不同类型

    // 调用方法
    integer.print();      // 调用 Point<i32> 的 print 方法
    float.print();        // 调用 Point<f64> 的 print 方法
    char_point.print();   // 调用 Point<char> 的 print 方法
}

fn main() {
    demo_1();
    println!();
    demo_2();
    println!();
    demo_3();
}
