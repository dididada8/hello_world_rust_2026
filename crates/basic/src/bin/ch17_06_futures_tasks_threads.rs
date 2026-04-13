use std::thread;
use std::time::Duration;

fn demo_1() {
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


fn main() {
    demo_1();
}
