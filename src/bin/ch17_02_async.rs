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

fn main() {
    demo_1();
}
