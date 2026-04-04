/* ========== 生命周期（Lifetime）基础 ==========

生命周期：引用的有效作用域

为什么需要生命周期标注：
  - Rust 需要知道返回的引用是从哪个参数借用的
  - 防止悬垂引用（dangling reference）
  - 确保引用始终有效
*/

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
            x  // 返回 x 的引用
        } else {
            y  // 返回 y 的引用
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

fn main() {
    demo_1();
}