use chrono::Local;
use helloworld::print_line_separator;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn demo_1() {
    // 这个示例刻意“线程 + Future”混合使用，目的是体现两种并发模型的互补：
    // 1) 线程适合承载阻塞或独立 CPU 工作：
    //    例如 thread::sleep、阻塞 I/O、或必须跑在单独 OS 线程的任务。
    // 2) Future/async 适合高并发等待：
    //    recv().await 在等待消息时不会占住线程，可让执行器调度其他任务。
    // 3) 用 channel 连接两者可实现解耦：
    //    生产者只管发送，消费者只管异步处理，生命周期通过“关闭通道”自然收尾。
    // 4) 工程实践收益：
    //    遗留阻塞代码可先放在线程里，外围用 async 组织流程，便于渐进式迁移。
    //
    // 简单说：线程负责“做事/阻塞”，Future 负责“等待/编排”，组合后吞吐和可维护性更好。

    // 创建一个异步通道：
    // - tx: 发送端（producer）
    // - rx: 接收端（consumer）
    //
    // 这里的 recv 是 async API，因此“接收方”天然适合放到 async 上下文里等待消息。
    let (tx, mut rx) = trpl::channel();

    // 启动一个“操作系统线程”（OS thread）作为生产者。
    // move 关键字把 tx 的所有权移动进线程闭包，确保线程生命周期内 tx 一直有效。
    thread::spawn(move || {
        // 每秒发送一个数字，总共发送 1..=10。
        for i in 1..=10 {
            // send 失败通常表示接收端已经被丢弃（没有人再接收消息）。
            // 这里用 unwrap 简化示例：如果失败就直接 panic。
            tx.send(i).unwrap();

            // 用阻塞式睡眠模拟“慢速生产者”。
            // 注意：这是在线程中阻塞，不会阻塞 async runtime 的执行器线程。
            thread::sleep(Duration::from_secs(1));
        }
        // 循环结束后，tx 被 drop，通道关闭。
        // 这会让接收端最终得到 None，从而退出消费循环。
    });

    // 启动一个最小 async 执行器并运行消费逻辑，直到 future 完成。
    trpl::block_on(async {
        // recv().await 的语义：
        // - 有消息时返回 Some(value)
        // - 所有发送端都关闭后返回 None
        //
        // while let Some(...) 这种写法可持续消费，直到通道自然结束。
        while let Some(value) = rx.recv().await {
            println!("received: {}", value);
        }

        // 走到这里说明：
        // 1) 生产者线程已经结束（或至少所有 tx 已被 drop）
        // 2) 通道中的消息已被全部取完
    });
}

fn demo_2() {
    // demo_2: 多个线程生产者 -> 一个 async 消费者
    // 目标：演示多个 tx clone 可以并发发送，而 rx 在单个 Future 中统一接收。
    let (tx, mut rx) = trpl::channel();

    let producer_count = 3;
    let messages_per_producer = 4;
    let mut handles = Vec::with_capacity(producer_count);

    for producer_id in 1..=producer_count {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            for n in 1..=messages_per_producer {
                let thread_id = thread::current().id();
                let now = Local::now();
                let msg = format!("P{producer_id} -> message {n}   {:?} {now}", thread_id);
                tx.send(msg).unwrap();

                // 每个生产者使用不同节奏，模拟真实场景中的并发交错。
                let delay_ms = 150 * producer_id as u64;
                thread::sleep(Duration::from_millis(delay_ms));
            }
            // 线程结束时，其 tx clone 被 drop。
        });
        handles.push(handle);
    }

    // 丢弃主线程手中的 tx，避免额外发送端让通道无法结束。
    drop(tx);

    trpl::block_on(async {
        // 单个 async 接收者持续消费，直到所有发送端关闭后收到 None。
        while let Some(msg) = rx.recv().await {
            println!("received: {msg}");
        }
        println!("receiver done: all producers finished and channel closed");
    });

    // join 线程，确保没有静默 panic。
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("===demo 1====");
    demo_1();
    print_line_separator();
    println!("===demo 2====");
    demo_2();
    print_line_separator();
}
