use helloworld::print_type_of;

fn demo_1() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // 遍历链表并打印每个节点的值，同时使用了 i32 和 Box<List> 字段
    fn print_list(list: &List) {
        match list {
            List::Cons(val, next) => {
                print!("{} -> ", val); // 使用 i32 字段
                print_list(next);      // 使用 Box<List> 字段（递归）
            }
            List::Nil => println!("Nil"),
        }
    }

    use List::{Cons, Nil};
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // error: value used here after move
    // a 的所有权已转移给 b，所以 c 无法再使用 a

    print_list(&b); // 输出：3 -> 5 -> 10 -> Nil
    print_type_of(&b, Some("demo_1:List"));
}

fn main() {
    demo_1();
}