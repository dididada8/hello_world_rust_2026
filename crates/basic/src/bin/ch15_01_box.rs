use helloworld::{print_line_separator, print_type_of};

fn demo_1(){
    #[derive(Debug)]
    enum List{
        Cons(i32, Box<List>),
        Nil
    }
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print_type_of(&list,Some("box list demo"));

    // 方式1：match —— 完整模式匹配，必须覆盖所有分支
    // list 的所有权在此转移（move），因为 List 未实现 Copy trait
    // - Cons(head, tail)：匹配非空节点，head 绑定 i32 值，tail 绑定 Box<List>
    //   head 类型：i32（实现了 Copy，直接复制）
    //   tail 类型：Box<List>（获得 Box 的所有权）
    // - Nil：匹配空节点
    // {:?} 打印 tail 需要 List 实现 Debug trait（已通过 #[derive(Debug)] 自动派生）
    match list {
        Cons(head, tail) => println!("[match]    head = {}, tail = {:?}", head, tail),
        Nil => println!("[match]    list is empty"),
    }

    // 方式2：if let —— 只关心一种模式，忽略其他情况，代码更简洁
    // 与 match 等价，但不需要写 Nil 分支
    let list2 = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    if let Cons(head, tail) = list2 {
        println!("[if let]   head = {}, tail = {:?}", head, tail);
    }

    // 方式3：let else（Rust 1.65+）—— 匹配失败时执行 else 块（必须发散：return/panic/break）
    // 适合"匹配失败就提前退出"的场景，成功后 head/tail 直接在外部作用域可用
    let list3 = Cons(42, Box::new(Nil));
    let Cons(head, tail) = list3 else {
        println!("[let else] list is empty");
        return;
    };
    println!("[let else] head = {}, tail = {:?}", head, tail);

    // 方式4：为枚举实现方法 —— 封装访问逻辑，调用方不需要关心内部结构
    impl List {
        // 返回头部值的 Option，不转移所有权（借用 &self）
        fn head(&self) -> Option<i32> {
            match self {
                Cons(val, _) => Some(*val),
                Nil => None,
            }
        }
        // 返回尾部的引用 Option，不转移所有权
        fn tail(&self) -> Option<&List> {
            match self {
                Cons(_, next) => Some(next), // Box<T> 实现了 Deref，可直接得到 &List
                Nil => None,
            }
        }
    }
    let list4 = Cons(7, Box::new(Cons(8, Box::new(Nil))));
    println!("[method]   head = {:?}, tail = {:?}", list4.head(), list4.tail());
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    print_type_of(&b,Some("box demo"));

    print_line_separator();
    demo_1();
}