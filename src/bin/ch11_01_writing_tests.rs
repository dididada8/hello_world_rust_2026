use std::fmt::format;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    String::from(format!("Hello!  {name}"))
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/* ========== Rust 测试的两种运行方式 ==========

方式1：使用 cargo test（标准方式，推荐）
   命令：cargo test
   特点：
     - 自动发现并运行所有带 #[test] 的函数
     - 并行运行测试
     - 捕获输出
     - 提供详细的测试报告

方式2：在 main() 中手动调用测试函数
   特点：
     - 使用 cargo run 运行
     - 串行执行
     - 直接看到输出
     - 适合调试和演示
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_positive() {
        assert_eq!(add(10, 20), 30);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(5, 0), 5);
    }

    #[test]
    fn test_add_no() {
        assert_ne!(add(5, 0), 1);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}

/* ========== 在 main() 中手动运行测试 ========== */
fn main() {
    println!("========== 手动运行测试 ==========\n");

    /* 方法1：直接调用测试逻辑（复制测试代码）
       注意：不能直接调用 tests::it_works()，因为：
         - #[cfg(test)] 使测试模块只在测试模式下编译
         - #[test] 标记的函数在非测试模式下不可用
    */

    println!("测试 1: it_works");
    let result = add(2, 2);
    assert_eq!(result, 4);
    println!("✅ 通过: add(2, 2) = {}", result);
    println!();

    println!("测试 2: test_add_positive");
    let result = add(10, 20);
    assert_eq!(result, 30);
    println!("✅ 通过: add(10, 20) = {}", result);
    println!();

    println!("测试 3: test_add_zero");
    let result = add(5, 0);
    assert_eq!(result, 5);
    println!("✅ 通过: add(5, 0) = {}", result);
    println!();

    /* 方法2：创建测试运行器函数 */
    run_tests();

    println!("\n========== 所有测试通过！ ==========");
    println!("\n提示：");
    println!("- 运行此程序：cargo run --bin ch11_01_writing_tests");
    println!("- 运行标准测试：cargo test --bin ch11_01_writing_tests");
}

/* 测试运行器函数：封装测试逻辑 */
fn run_tests() {
    println!("--- 运行自定义测试套件 ---");

    // 定义测试用例
    let test_cases = vec![
        (1, 1, 2, "add(1, 1)"),
        (100, 200, 300, "add(100, 200)"),
        (0, 0, 0, "add(0, 0)"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (left, right, expected, name) in test_cases {
        let result = add(left, right);
        if result == expected {
            println!("✅ 通过: {} = {}", name, result);
            passed += 1;
        } else {
            println!("❌ 失败: {} = {}, 期望 {}", name, result, expected);
            failed += 1;
        }
    }

    println!("\n测试统计: {} 通过, {} 失败", passed, failed);
}

/* ========== 对比两种运行方式 ==========

1. cargo test（标准方式）：
   $ cargo test --bin ch11_01_writing_tests

   优点：
     ✅ 自动发现测试
     ✅ 并行执行（速度快）
     ✅ 详细的测试报告
     ✅ 失败时显示详细信息
     ✅ 支持过滤测试

   示例输出：
     running 3 tests
     test tests::it_works ... ok
     test tests::test_add_positive ... ok
     test tests::test_add_zero ... ok

2. cargo run（手动方式）：
   $ cargo run --bin ch11_01_writing_tests

   优点：
     ✅ 适合调试
     ✅ 可以自定义输出
     ✅ 可以看到中间过程
     ✅ 串行执行（便于追踪）

   缺点：
     ❌ 需要手动编写测试逻辑
     ❌ 不会自动发现新测试
     ❌ 没有测试框架的高级功能

最佳实践：
  - 开发时：使用 cargo test
  - 演示时：使用 main() 手动运行
  - 调试时：使用 main() + println!
*/
