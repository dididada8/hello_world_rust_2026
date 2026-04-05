/* ========== 生命周期（Lifetime）基础 ==========

生命周期：引用的有效作用域

为什么需要生命周期标注：
  - Rust 需要知道返回的引用是从哪个参数借用的
  - 防止悬垂引用（dangling reference）
  - 确保引用始终有效
*/
use helloworld::print_line_separator;
use std::fmt::Display;

fn demo_1() {
    /* ❌ 错误版本：缺少生命周期标注

    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x  // 返回 x 还是 y？
        } else {
            y  // 编译器无法确定！
        }
    }

    编译错误：
      error[E0106]: missing lifetime specifier
      --> expected named lifetime parameter
      = help: this function's return type contains a borrowed value,
              but the signature does not say whether it is borrowed from `x` or `y`
    */

    /* ✅ 正确版本：添加生命周期标注

    语法：fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

    【生命周期标注详解】
      <'a>           - 声明生命周期参数 'a（可以是任意名字，通常用 'a, 'b, 'c）
      x: &'a str     - x 的生命周期是 'a
      y: &'a str     - y 的生命周期是 'a
      -> &'a str     - 返回值的生命周期是 'a

    【含义】
      - 返回值的生命周期与 x 和 y 中较短的那个相同
      - 'a 是 x 和 y 生命周期的重叠部分
      - 保证返回的引用在 x 和 y 都有效时才有效

    【生命周期不是】
      ❌ 改变引用的实际生命周期
      ❌ 创建新的生命周期
      ✅ 只是描述多个引用之间的生命周期关系

    【对比 Java】
      Java 不需要生命周期标注：
        - Java 使用垃圾回收（GC）
        - Rust 使用借用检查器（borrow checker）
        - Rust 在编译时就保证内存安全
    */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x // 返回 x 的引用
        } else {
            y // 返回 y 的引用
        }
    }

    // 测试用例
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: {}", result);

    /* 生命周期示例：理解作用域

    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            // ❌ 编译错误！result 的生命周期不能超过 string2
        }
        // println!("最长的字符串是: {}", result);  // string2 已失效
    }

    解释：
      - result 的生命周期 'a 是 string1 和 string2 的交集
      - string2 在内层作用域结束时被销毁
      - result 不能在 string2 之后使用
    */
}

/* ========== demo_2: 不使用生命周期语法的替代方案 ========== */
fn demo_2() {
    /* 方案1：返回 String（拥有所有权）而不是引用

    优点：
      - 不需要生命周期标注
      - 调用者拥有返回值，可以随意使用
      - 简单直观

    缺点：
      - 需要分配堆内存（性能开销）
      - 克隆数据（如果数据很大会很慢）
    */
    fn longest_owned(x: &str, y: &str) -> String {
        if x.len() > y.len() {
            x.to_string() // 转换为 String，拥有所有权
        } else {
            y.to_string()
        }
    }

    let s1 = "hello world";
    let s2 = "hi";
    let result = longest_owned(s1, s2);
    println!("方案1 - 最长的字符串: {}", result);

    /* 方案2：返回索引而不是引用

    优点：
      - 不需要生命周期标注
      - 避免借用问题
      - 索引是 Copy 类型

    缺点：
      - 调用者需要保留原始数据
      - 需要额外的逻辑来获取实际值
    */
    fn longest_index(x: &str, y: &str) -> bool {
        x.len() > y.len() // 返回 true 表示 x 更长，false 表示 y 更长
    }

    let s1 = "hello world";
    let s2 = "hi";
    let is_first_longer = longest_index(s1, s2);
    let result = if is_first_longer { s1 } else { s2 };
    println!("方案2 - 最长的字符串: {}", result);

    /* 方案3：使用静态生命周期 'static

    适用场景：
      - 字符串字面量（存储在程序二进制文件中）
      - 静态变量
      - 整个程序运行期间都有效的数据

    限制：
      - 只能用于 'static 数据
      - 不适用于临时创建的 String
    */
    fn longest_static(x: &'static str, y: &'static str) -> &'static str {
        if x.len() > y.len() { x } else { y }
    }

    let s1 = "hello world"; // 字符串字面量是 'static
    let s2 = "hi";
    let result = longest_static(s1, s2);
    println!("方案3 - 最长的字符串: {}", result);

    /* 方案4：使用元组返回多个值

    优点：
      - 不需要生命周期标注
      - 灵活，可以返回额外信息

    缺点：
      - 调用者需要处理元组
      - 语法稍微复杂
    */
    fn longest_with_length(x: &str, y: &str) -> (String, usize) {
        if x.len() > y.len() {
            (x.to_string(), x.len())
        } else {
            (y.to_string(), y.len())
        }
    }

    let s1 = "hello world";
    let s2 = "hi";
    let (result, length) = longest_with_length(s1, s2);
    println!("方案4 - 最长的字符串: {}, 长度: {}", result, length);

    /* ========== 对比总结 ==========

    | 方案              | 是否需要生命周期 | 性能 | 灵活性 | 适用场景 |
    |-------------------|------------------|------|--------|----------|
    | 返回引用 + 生命周期 | ✅ 需要         | 高   | 高     | 零拷贝，高性能场景 |
    | 返回 String       | ❌ 不需要       | 中   | 高     | 数据较小，简单场景 |
    | 返回索引/bool     | ❌ 不需要       | 高   | 低     | 简单判断 |
    | 'static 生命周期  | ❌ 不需要       | 高   | 低     | 只用于静态数据 |
    | 返回元组          | ❌ 不需要       | 中   | 高     | 需要多个返回值 |

    最佳实践：
      - 如果性能关键且数据量大 → 使用生命周期标注 + 返回引用
      - 如果简单场景且数据量小 → 返回 String（clone）
      - 如果是静态字符串 → 使用 'static
      - 如果只是比较 → 返回 bool 或索引
    */
}
fn demo_3() {
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("公告！{ann}");
        if x.len() > y.len() { x } else { y }
    }
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    // 第一次调用：使用字符串字面量（&str）
    let x = "hello world, let us learn Rust!!!";
    let result = longest_with_an_announcement(x, "world", "这是一个公告");
    println!("最长的字符串: {}", result);

    let x_owned = format!("{}{}", x, "!!!");  // x_owned 的类型是 String
    let result = longest(x_owned.as_str(), "world");  // 使用 .as_str() 转换为 &str
    println!("最长的字符串: {}", result);

    /* 类型对比说明：
    &str（字符串切片）：
        - 不可变引用
        - 不拥有数据
        - 固定大小（在栈上）
        - 例：let s = "hello";

    String（字符串类型）：
        - 可变
        - 拥有数据
        - 大小可变（在堆上）
        - 例：let s = String::from("hello");

    转换：
        - String -> &str：使用 .as_str() 或 &s 或 &s[..]
        - &str -> String：使用 .to_string() 或 String::from()
    */
}

fn main() {
    demo_1();
    println!("\n========== 不使用生命周期的替代方案 ==========\n");
    demo_2();
    print_line_separator();
    demo_3();
}
