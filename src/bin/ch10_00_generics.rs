use helloworld::print_line_separator;

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

// demo_2: 对 demo_1 的升级 - 提取函数消除重复代码
// 改进点：
// 1. 将查找最大值的逻辑封装到 largest 函数中
// 2. 函数签名：largest(list: &[i32]) -> &i32
//    - 参数：&[i32] 是切片引用，可以接受 Vec、数组等
//    - 返回：&i32 返回引用，避免复制数据
// 3. 复用：可以对多个列表调用同一个函数
//
// 对比 demo_1:
// - demo_1: 代码写在主函数中，不可复用
// - demo_2: 提取成函数，可以多次调用，避免重复代码
fn demo_2() {
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 第一个列表
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("{:?} 最大的数字是 {},", number_list, result);

    // 第二个列表 - 复用同一个函数
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("{:?} 最大的数字是 {},", number_list, result);
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
