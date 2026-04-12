use helloworld::print_line_separator;

// 演示：单个 async 块内，发送与接收是顺序执行的，不存在并发。
//
// 【核心原因】
// 整个逻辑只有 **一个 Future（async 块）**，在同一个任务里顺序运行：
//   1. 先完整跑完"发送循环"（含多次 sleep）
//   2. 再从头开始跑"接收循环"
//
// 发送循环结束时，4 条消息已全部堆积在 channel 的缓冲区里，
// 接收循环一启动，4 次 recv().await 全部立刻就绪，因此 4 行输出同时打印，
// 看起来像"瞬间全部收到"——这正是没有并发的体现。
fn demo_1() {
    trpl::block_on(async {
        // trpl::channel() 返回一个带缓冲的异步 channel（类似 tokio::sync::mpsc）。
        // tx：发送端（Sender）；rx：接收端（Receiver，需要 mut 因为 recv() 需要 &mut self）。
        let (tx, mut rx) = trpl::channel();

        let vals = vec!["hi", "from", "the", "future"];

        // ── 阶段一：发送循环 ──────────────────────────────────────────────
        // 整个 for 循环在同一个 async 块里顺序执行，rx.recv() 此时根本没有机会运行。
        for val in vals {
            // send() 是同步的，立即把消息放入 channel 缓冲区，不阻塞。
            tx.send(val).unwrap();

            // .await 把控制权还给运行时，500ms 后再恢复。
            // 但运行时里只有这一个任务，sleep 到期后仍然回到这同一个 async 块。
            // 注意：sleep 期间 rx 端没有任何东西在跑，消息只是静静待在缓冲区里。
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }

        // 显式 drop tx，让 channel 关闭。
        // rx.recv() 在 channel 关闭且缓冲区清空后会返回 None，从而结束 while 循环。
        drop(tx);

        // ── 阶段二：接收循环 ──────────────────────────────────────────────
        // 执行到这里时，4 条消息已经在缓冲区里等待了（发送循环跑了约 2 秒才到这里）。
        // 每次 recv().await 发现缓冲区非空，立即返回，不需要等待任何时间。
        // 结果：4 行输出几乎同时打印，而不是每隔 500ms 打印一行。
        //
        // 若要实现"每隔 500ms 收到一条"的效果，需要把发送和接收放到两个并发的 Future 中
        // （例如用 trpl::join! 或 tokio::spawn），让它们真正同时运行。
        while let Some(value) = rx.recv().await {
            println!("received: {}", value);
        }
    });
}

fn demo_2() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec!["hi", "from", "the", "future"];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received: {}", value);
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

// 演示：两个发送端 + 一个接收端，用 trpl::join! 并发运行。
//
// 【为什么不需要手动 drop(tx) 程序也能正常结束？】
//
// channel 的关闭条件：**所有 Sender（tx 及其 clone）全部被 drop**，rx.recv() 才会返回 None。
//
// 这里 tx 和 tx1 都通过 `async move` 被"移动"进各自的 Future：
//   - tx1  →  tx1_fut（async move）
//   - tx   →  tx_fut （async move）
//
// 移动之后，外层 async 块里**不再持有任何 tx / tx1 的所有权**。
// 当 tx1_fut / tx_fut 各自跑完（async 块执行到末尾），Rust 自动 drop
// 其局部变量——和普通函数返回时 drop 局部变量完全一样。
//
// 时序：
//   tx1_fut 跑完 → tx1 被 drop
//   tx_fut  跑完 → tx  被 drop   ← 此时两个 Sender 均已 drop，channel 关闭
//   rx_fut 的 rx.recv().await 返回 None → while 循环结束 → rx_fut 完成
//   trpl::join! 三个 Future 全部完成，返回
//
// 对比 demo_1：tx 留在外层 async 块的作用域里（未 move），与 rx 的 while
// 循环处于同一作用域，必须显式 drop(tx) 才能关闭 channel；否则 while 循环
// 永远等不到 None，程序会一直挂着。
fn demo_3() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // tx1 是 tx 的 clone，代表第二个发送端（每 500ms 发一条）。
        // async move 把 tx1 的所有权转移进这个 Future；
        // tx1_fut 跑完时，tx1 自动被 drop。
        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec!["hi", "from", "the", "future"];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
            // 此处 async 块结束，tx1 离开作用域，自动 drop
        };

        // tx 是原始发送端（每 1000ms 发一条）。
        // async move 把 tx 的所有权转移进这个 Future；
        // tx_fut 跑完时，tx 自动被 drop。
        // 两个 Sender 都 drop 后，channel 关闭。
        let tx_fut = async move {
            let vals = vec!["more", "messages", "for", "you"];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(std::time::Duration::from_millis(1000)).await;
            }
            // 此处 async 块结束，tx 离开作用域，自动 drop
            // → 两个 Sender 均已 drop → channel 关闭
        };

        // rx.recv().await 在 channel 关闭且缓冲区清空后返回 None，
        // while 循环自然结束，无需外部干预。
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received: {}", value);
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
