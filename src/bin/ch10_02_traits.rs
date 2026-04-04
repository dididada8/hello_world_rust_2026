pub trait Summary {
    /* trait 的默认实现方法

    ========== 为什么第 67 行不能加分号 ==========

    【表达式 vs 语句】
      第 67 行：String::from("(Read more...)")  ← 这是表达式

      ✅ 正确（无分号）：
         fn summarize(&self) -> String {
             String::from("(Read more...)")    /* 表达式，作为返回值 */
         }

      ❌ 错误（有分号）：
         fn summarize(&self) -> String {
             String::from("(Read more...)");   /* 语句，不返回值 */
         }
         编译错误：expected `String`, found `()`

    【Rust 的核心规则】
      1. 表达式（expression）：有返回值
         - 没有分号
         - 例：5 + 3, if true { 1 } else { 2 }, String::from("hi")

      2. 语句（statement）：没有返回值（返回 unit type `()`）
         - 有分号
         - 例：let x = 5;  println!("hi");  String::from("hi");

    【函数返回值的两种写法】
      方式1（表达式返回，推荐）：
         fn get_number() -> i32 {
             42              /* 无分号，表达式作为返回值 */
         }

      方式2（显式 return）：
         fn get_number() -> i32 {
             return 42;      /* 有分号也可以，因为 return 是语句 */
         }

    【对比 Java】
      Java 必须显式写 return：
         String summarize() {
             return "Read more...";   /* 必须有 return */
         }

      Rust 可以省略 return（更简洁）：
         fn summarize(&self) -> String {
             String::from("(Read more...)")   /* 隐式返回 */
         }

    【常见错误示例】
      错误写法：
         fn summarize(&self) -> String {
             String::from("(Read more...)");   /* 加了分号！ */
         }

      错误信息：
         error[E0308]: mismatched types
          --> expected `String`, found `()`

      原因：
         - String::from(...); 变成了语句
         - 语句返回 ()（unit type）
         - 函数期望返回 String，但实际返回了 ()
         - 类型不匹配！
    */
    fn summarize(&self) -> String {
        String::from("(Read more...)")  /* 无分号 = 表达式 = 返回值 */
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}