use std::rc;
use helloworld::{print_line_separator, print_type_of};

// demo_1：用 Box<List> 构建链表，演示单一所有权的限制
fn demo_1() {
    // Box<List> 是独占所有权的智能指针，将数据分配到堆上
    // 每个 Box 只能有一个所有者，数据随所有者销毁而释放
    enum List {
        Cons(i32, Box<List>), // 节点：i32 值 + 指向下一节点的 Box 指针（独占所有权）
        Nil,                   // 链表终止符
    }

    // 递归遍历链表并打印每个节点的值
    // 参数是 &List（借用），不转移所有权，避免递归中所有权丢失
    fn print_list(list: &List) {
        match list {
            // 匹配 Cons 节点：val 是 &i32，next 是 &Box<List>（自动解引用可当 &List 用）
            List::Cons(val, next) => {
                print!("{} -> ", val); // 打印当前节点值
                print_list(next);      // 递归处理下一节点（next: &Box<List> 自动 deref 为 &List）
            }
            // 匹配终止符，打印 "Nil" 并换行
            List::Nil => println!("Nil"),
        }
    }

    use List::{Cons, Nil};

    // 构建链表 a：5 -> 10 -> Nil
    // Box::new() 将 Cons(10, ...) 分配到堆上，a 持有整个链表的所有权
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    // Box::new(a) 将 a 移入堆上，a 的所有权转移给 b.next
    // 此后 a 这个绑定失效，无法再使用
    let b = Cons(3, Box::new(a));

    // 以下代码会编译报错：error[E0382]: use of moved value: `a`
    // let c = Cons(4, Box::new(a));
    // 原因：Box 是独占所有权，a 已经 move 进 b，不能再 move 进 c
    // 这正是 Box 与 Rc 的核心区别：Box 不允许共享，Rc 允许共享

    print_list(&b); // 输出：3 -> 5 -> 10 -> Nil
    print_type_of(&b, Some("demo_1:List"));
    // demo_1 结束，b 离开作用域，Box 递归释放整个链表（3 -> 5 -> 10 -> Nil）
}

fn demo_2() {
    // 用 Rc<List> 替代 Box<List>，允许多个所有者共享同一份堆数据
    // Rc = Reference Counted，通过引用计数实现共享所有权（仅限单线程）
    enum List {
        Cons(i32, Rc<List>), // 节点：i32 值 + 指向下一节点的 Rc 智能指针
        Nil,                  // 链表终止符
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    // 构建链表 a：5 -> 10 -> Nil
    // Rc::new() 将数据分配到堆上，同时创建第一个 Rc 指针
    // 此时堆上数据的引用计数 = 1（只有 a 持有）
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // Rc::clone(&a) 不复制堆上的数据，只克隆 Rc 指针本身（增加引用计数）
    // b 的 next 字段持有一个新的 Rc 指针，指向同一块堆数据
    // 此时 a 指向的堆数据引用计数 = 2（a 本身 + b.next）
    let b = Cons(3, Rc::clone(&a));

    // 同理，c 的 next 也克隆了 a 的 Rc 指针
    // 此时 a 指向的堆数据引用计数 = 3（a + b.next + c.next）
    let c = Cons(4, Rc::clone(&a));

    // 此时引用计数为 3
    print_type_of(&a, Some("a typeDef:"));
    print_type_of(&b, Some("b typeDef:"));
    print_type_of(&c, Some("c typeDef:"));
    println!();
    // ---- match 详解 ----
    //
    // 为什么不能直接 `match a`？
    //   a 的类型是 Rc<List>，Rc<T> 本身没有实现 List 的模式，
    //   不能对 Rc<T> 直接做枚举模式匹配，必须先拿到内部的 &List。
    //
    // a.as_ref() 的作用：
    //   将 Rc<List> 转换为 &List（借用内部值），不转移所有权，引用计数不变。
    //   Rc<T> 实现了 AsRef<T>，所以 a.as_ref() 返回 &List。
    //
    // 匹配 &List 的两个分支（必须穷举，否则编译报错）：
    //   List::Cons(val, next) => val: &i32, next: &Rc<List>
    //   _ => 通配符，覆盖 List::Nil 的情况
    //
    // 整个 match 块结束后，借用结束，引用计数依然是 3。
    match a.as_ref() {
        List::Cons(val, next) => {
            println!("a head = {val}"); // 输出：5（a 的头节点值）
            // next 是 &Rc<List>，指向堆上的 Cons(10, Rc::new(Nil))
            // 因为只是借用，引用计数仍为 3
            print_type_of(&next, Some("a next typeDef==>"));
        }
        _ => {
            // 若 a 指向 Nil 则走这里（本例不会触发）
            println!("a is empty");
        }
    } // 借用结束，引用计数仍为 3


    // b 是 List（不是 Rc<List>），可以直接解构，转移 b 的所有权
    // 解构后 next 绑定了 b.next（即 Rc::clone(&a) 得到的那个 Rc 指针）
    if let Cons(val, next) = b {
        println!("b head = {val}"); // 输出：3
        print_type_of(&next, Some("b next typeDef"));
    } // next 离开作用域，b 持有的那份 Rc 被 drop，引用计数从 3 降为 2


    // 同理，c 解构后 next 持有 c.next 那份 Rc
    if let Cons(val, next) = c {
        println!("c head = {val}"); // 输出：4
        print_type_of(&next, Some("c next typeDef"));
    } // next 离开作用域，c 持有的那份 Rc 被 drop，引用计数从 2 降为 1


    // demo_2 结束，a 离开作用域，引用计数从 1 降为 0，堆上数据被释放
    //
    // 总结：
    //   b.next 和 c.next 都共享 a 指向的堆数据（5 -> 10 -> Nil）
    //   Rc 保证只要引用计数 > 0，数据就不会被释放
    //   与 Box 不同，Rc 允许多个所有者；与 Arc 不同，Rc 不支持多线程
}
#[allow(unused_variables)]
fn demo_3() {
    #[allow(unused_variables, dead_code)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    use std::rc::Rc;
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    let d = Rc::new(Cons(4, Rc::new(b)));
    println!("count after creating c = {}", Rc::strong_count(&a));

}
fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
