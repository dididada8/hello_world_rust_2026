/*
与 Rc<T> 不同，RefCell<T> 类型表示对其持有的数据的单一所有权。那么，
是什么让 RefCell<T> 与 Box<T> 这样的类型不同呢？：
在任何给定时间，你 要么 有一个可变引用， 要么 有任意数量的不可变引用（但不能两者都有）。
引用必须始终有效。
 */
use helloworld::print_line_separator;

fn demo_1() {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    /*
       // ============================================================
       // 无法编译的版本：违反了 Rust 借用规则
       // ============================================================
       struct MockMessenger {
            sent_messages: Vec<String>,  // 普通 Vec，没有内部可变性
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: vec![],
                }
            }
        }

        impl Messenger for MockMessenger {
            // 问题根源在这里：
            //
            // Messenger trait 要求 send 的签名是 fn send(&self, msg: &str)
            // 即接收者是 &self（不可变借用），这是 trait 的约定，无法改变。
            //
            // 但在方法体内，self.sent_messages.push(...) 需要修改 Vec，
            // 这要求 self 是 &mut self（可变借用）。
            //
            // 矛盾：
            //   - trait 约定：&self        → 不可变借用
            //   - push 需要：&mut self.sent_messages → 可变借用
            //
            // Rust 的借用规则：不能通过不可变引用修改数据。
            // 编译器报错：error[E0596]: cannot borrow `self.sent_messages`
            //             as mutable, as it is behind a `&` reference
            fn send(&self, message: &str) {
                self.sent_messages.push(String::from(message)); // ← 编译报错
                //  ^^^^^^^^^^^^^^^^^ 通过 &self 试图可变地访问字段，违反借用规则
            }
        }
    */

    use std::cell::RefCell;

    // RefCell<T> 提供"内部可变性"：
    //   即使持有的是 &self（不可变引用），也可以在运行时修改内部数据。
    //   代价是：借用规则的检查从编译期推迟到运行时（违反则 panic）。
    struct MockMessenger {
        // RefCell<Vec<String>> 而非 Vec<String>：
        //   外层 RefCell 是包装器，对编译器来说整个字段是"不可变的"，
        //   但 RefCell 在运行时维护一个借用状态计数器，
        //   允许通过 borrow() / borrow_mut() 动态地借出内部 Vec。
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // RefCell::new(vec![]) 创建一个 RefCell，
                // 内部存放一个空 Vec<String>，初始借用状态：0 个不可变借用，0 个可变借用
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // &self：编译器看到的是不可变借用，满足 trait 的签名要求
        fn send(&self, message: &str) {
            // borrow_mut() 的作用：
            //   在运行时向 RefCell 申请一个可变借用（类似 &mut Vec<String>）。
            //   RefCell 内部计数器检查：
            //     - 若当前已有不可变借用（borrow() 持有中）→ panic!
            //     - 若当前已有可变借用（borrow_mut() 持有中）→ panic!
            //     - 否则 → 成功，返回 RefMut<Vec<String>> 智能指针
            //
            // RefMut<T> 实现了 DerefMut，所以可以直接调用 Vec 的方法（.push）
            // RefMut 离开作用域时，自动释放可变借用（计数器归零）
            self.sent_messages.borrow_mut().push(String::from(message));
            //                 ^^^^^^^^^^^^
            //   运行时申请可变借用 → 返回 RefMut<Vec<String>>
            //   .push() 通过 DerefMut 直接操作内部 Vec
            //   本行结束，RefMut 临时值被 drop，可变借用释放
        }
    }

    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // set_value(80)：80/100 = 80%，触发 send("Warning: ...")
        // send 内部调用 borrow_mut().push(...)，将消息存入 RefCell 内的 Vec
        limit_tracker.set_value(80);

        // borrow() 的作用：
        //   在运行时申请一个不可变借用，返回 Ref<Vec<String>>。
        //   RefCell 内部检查：
        //     - 若当前已有可变借用（borrow_mut() 持有中）→ panic!
        //     - 否则 → 成功，不可变借用计数 +1
        //
        // .len() 通过 Deref 直接调用 Vec 的方法
        // Ref 离开作用域时，不可变借用计数 -1
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        //                                      ^^^^^^^^
        //   运行时申请不可变借用 → 返回 Ref<Vec<String>>
        //   .len() 通过 Deref 读取 Vec 长度
        //   断言结束，Ref 被 drop，不可变借用释放

        // 再次 borrow() 读取并打印，与上面同理，此时上一个 Ref 已释放，可以正常借出
        println!("{:?}", mock_messenger.sent_messages.borrow());
    }

    it_sends_an_over_75_percent_warning_message();
}

fn main() {
    demo_1();
    print_line_separator();
}
