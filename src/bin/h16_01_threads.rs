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

    for i in 0..5 {
        let thread_tx = tx.clone(); // 克隆发送端，允许多个生产者
        thread::spawn(move || {
            let val = format!("hi from thread {i}");
            thread_tx.send(val).unwrap();
        });
    }

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
