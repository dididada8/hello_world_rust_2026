use helloworld::print_line_separator;
use std::fmt::{Debug, Display};

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
        String::from("(Read more...)") /* 无分号 = 表达式 = 返回值 */
    }
}

/* ========== derive 宏的使用说明 ==========

语法：#[derive(Trait1, Trait2, ...)]

✅ 可以自动派生的 trait（标准库内置）：
   - Debug：调试输出（{:?}）
   - Clone：克隆
   - Copy：复制语义
   - PartialEq, Eq：相等比较
   - PartialOrd, Ord：顺序比较
   - Hash：哈希

示例：
   #[derive(Debug, Clone, PartialEq)]
   struct Point { x: i32, y: i32 }

❌ 不能自动派生的 trait（需要手动实现）：
   - Display：用户友好的显示格式（{}）
   - Default：默认值
   - 自定义 trait
正确方式：手动实现 Display trait
*/

#[derive(Debug)] /* 自动实现 Debug trait，可以用 {:?} 打印 */
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

/* 手动实现 Display trait
   Display 不能通过 derive 自动实现，需要手动编写
   实现后可以使用 {} 格式化打印
*/
impl std::fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )
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

fn demo_1() {
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    let (a, post) = sample_data();
    println!("1 new post: {}", post.summarize());
    notify(&a);
}

fn demo_2() {
    //Trait 约束：告诉编译器 T 必须实现 Summary trait，才能调用 summarize 方法
    fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    let (a, _) = sample_data();
    notify(&a);
}

fn demo_3() {
    //使用多个 Trait 约束：告诉编译器 T 必须同时实现 Summary 和 Display trait，才能调用 summarize 和 to_string 方法
    fn notify<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
        println!("Notify via {}", item);
    }
    let (a, _) = sample_data();
    notify(&a);
}

fn demo_4() {
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        // 函数体
        println!("pass parameter: t = {}, u = {:?}", t, u);
        0
    }
    let i = 3;
    let s = "hello";
    println!("The value of i is: {}", some_function(&i, &s));
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

fn sample_data() -> (NewsArticle, SocialPost) {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    (article, post)
}
