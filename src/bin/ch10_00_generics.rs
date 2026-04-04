fn demo_1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("最大的数字是 {largest}");
    // {:?} 是 Debug 格式化输出
    // - {} 使用 Display trait（用户友好的显示格式）
    // - {:?} 使用 Debug trait（开发者友好的调试格式）
    // - {:#?} 使用美化的 Debug 格式（多行缩进显示）
    // Vec<i32> 自动实现了 Debug trait，所以可以用 {:?} 打印
    // 输出示例：[34, 50, 25, 100, 65]
    println!("{:?}", number_list);
}

fn main() {
    demo_1();
}
