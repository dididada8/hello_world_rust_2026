fn demo_1() {
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

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

fn demo_2() {
    //fn largest<T>(list: &[T]) -> &T {

    // 泛型函数签名详解：fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T
    //
    // 1. <T: std::cmp::PartialOrd> - 泛型类型参数 + trait 约束
    //    - T: 泛型类型参数（Type parameter），代表任意类型
    //    - : std::cmp::PartialOrd - trait 约束（trait bound）
    //    - 含义：T 必须实现 PartialOrd trait（支持偏序比较）
    //    - 为什么需要：因为函数体中使用了 > 运算符（第 36 行）
    //
    // 2. (list: &[T]) - 参数
    //    - &[T]: T 类型元素的切片引用
    //    - 可以接受 &Vec<i32>、&[char]、&[f64] 等任何实现了 PartialOrd 的类型
    //
    // 3. -> &T - 返回类型
    //    - 返回 T 类型的引用
    //    - 避免所有权转移和数据复制
    //
    // 对比第 31 行注释掉的版本：
    //    - fn largest<T>(list: &[T]) -> &T
    //    - 没有 trait 约束，编译失败！
    //    - 错误原因：不是所有 T 都支持 > 比较（如自定义结构体）
    //
    // PartialOrd trait 说明：
    //    - 提供偏序比较：<, >, <=, >=
    //    - 基本类型（i32, f64, char 等）都自动实现了
    //    - 与 Ord 的区别：PartialOrd 允许不可比较的情况（如 NaN）
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("最大的数字是 {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("最大的字符是 {result}");
}

fn main() {
    demo_1();
    println!();
    demo_2();
}
