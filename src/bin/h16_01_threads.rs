use helloworld::print_line_separator;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn demo_1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned_thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("HI number {i} from the main=thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn demo_2() {
    let v = vec![1, 2, 3];
    //使用 move 关键字强制闭包取得它使用的值的所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

fn demo_3() {
    // mpsc::channel() 创建一个信道，返回 (发送端 tx, 接收端 rx)
    // mpsc = multiple producer single consumer（多生产者单消费者）
    let (tx, rx) = mpsc::channel();

    // move 将 tx 的所有权转移进闭包，子线程独占发送端
    thread::spawn(move || {
        let val = String::from("hi");

        // tx.send(val) 按值接收 val，val 的所有权转移给信道
        // 签名：fn send(&self, t: T) -> Result<(), SendError<T>>
        //                      ^^^  按值接收，消费所有权
        //
        // 为什么必须转移所有权而不是发送引用？
        //   若发送 &val，子线程结束后 val 销毁，接收端持有悬空引用 → 内存不安全
        //   Rust 在编译期禁止跨线程传递引用（除非满足 'static 生命周期）
        //   发送所有权则保证：数据的唯一持有者从子线程转移到信道再到主线程，全程安全
        //
        // send 之后 val 绑定失效，不可再使用（已 move 进信道）
        // 所有权流向：val → send() → [信道队列] → rx.recv() → received
        tx.send(val).unwrap();
        // unwrap()：send 返回 Result，若接收端已关闭则返回 Err，此处直接 panic
    });

    // rx.recv() 阻塞主线程，直到信道中有数据
    // 返回 Result<String, RecvError>，unwrap 取出 String
    // received 获得 String "hi" 的完整所有权（零拷贝，堆数据只有一份）
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn demo_4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn demo_5() {
    let (tx, rx) = mpsc::channel();

    // 错误1：原代码 `let txs = &vec![tx.clone(), tx.clone(), tx]`
    //   - vec![...] 是临时值，&vec![...] 是临时引用，离开语句后临时值销毁
    //   - txs[i] 试图从 &Vec 中 move 出 Sender<String>
    //   - Sender<String> 不是 Copy 类型，不能从引用中 move → 编译报错
    //
    // 修复：在循环内直接 clone()，每次迭代产生一个独立的发送端
    for i in 0..3 {
        // 每次循环 clone 一份 tx，move 进闭包，各线程持有独立的发送端
        // Sender::clone() 不复制信道数据，只增加发送端计数
        let t = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from(format!("more      <---{i}")),
                String::from(format!("messages  <---{i}")),
                String::from(format!("for       <---{i}")),
                String::from(format!("you       <---{i}")),
            ];

            for val in vals {
                t.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
            // t 离开作用域，clone 出来的这份发送端被 drop，信道发送端计数 -1
        });
    }

    // 错误2：原代码没有 drop(tx)
    //   mpsc 信道在所有发送端都 drop 后才关闭
    //   若不 drop 原始 tx，即使 3 个子线程都结束，信道仍未关闭（原始 tx 还活着）
    //   `for received in rx` 会永远阻塞，程序不会退出
    //
    // 修复：手动 drop 原始 tx，使信道在所有 clone 端结束后正常关闭
    drop(tx);

    // rx 作为迭代器：每次 recv 一条消息，所有发送端都 drop 后迭代结束
    for received in rx {
        println!("Got: {received}");
    }
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
    print_line_separator();
    demo_4();
    print_line_separator();
    demo_5();
}
