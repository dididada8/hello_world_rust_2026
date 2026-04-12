use helloworld::{print_line_separator, print_type_of};
use std::thread;

fn demo_1() {
    // ─────────────────────────────────────────────────────────────────
    // thread::spawn vs trpl::spawn_task 对比说明
    //
    // ┌─────────────────┬──────────────────────────┬──────────────────────────────┐
    // │                 │    thread::spawn          │    trpl::spawn_task          │
    // ├─────────────────┼──────────────────────────┼──────────────────────────────┤
    // │ 执行单元        │ OS 线程（真正的并行）     │ 异步任务（协作式并发）       │
    // │ 调度者          │ 操作系统内核              │ 异步运行时（tokio executor） │
    // │ 阻塞方式        │ thread::sleep（阻塞线程） │ trpl::sleep（让出控制权）    │
    // │ 栈开销          │ ~MB 级默认栈              │ 极小（仅 Future 状态机大小） │
    // │ 使用场景        │ CPU 密集 / 阻塞 I/O       │ 大量并发 I/O / 网络请求      │
    // │ 必须在 async 中 │ 否                        │ 是（需在 async 块/fn 内调用）│
    // └─────────────────┴──────────────────────────┴──────────────────────────────┘
    // ─────────────────────────────────────────────────────────────────

    // ① thread::spawn：创建一个全新的 OS 线程，与主线程真正并行运行。
    //   闭包在独立线程中执行，thread::sleep 会阻塞该线程 500ms，
    //   但不影响其他线程（主线程、异步运行时线程仍可继续）。
    //   返回 JoinHandle<T>，稍后可调用 .join() 等待其结束。
    let channel = thread::spawn(|| {
        for i in 1..=5 {
            println!("      c_hi number {i} from the channel thread!");
            // 阻塞当前 OS 线程 500ms（只阻塞这个线程，不影响其他线程）
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // ② trpl::block_on：在当前同步上下文中启动异步运行时，
    //   驱动传入的 async 块直到完成。此时 ① 的线程与运行时并行跑。
    trpl::block_on(async {
        // ③ trpl::spawn_task：在当前异步运行时上创建一个新的异步任务。
        //   与 thread::spawn 不同，它不创建新 OS 线程，而是把 Future 交给
        //   同一个运行时的调度器，由调度器在合适的时机 poll 它。
        //   任务与 main task（下方循环）交替执行：当 spawn_task 的 Future
        //   执行到 .await 暂停时，运行时切换去执行 main task，反之亦然。
        //   这是"协作式并发"：任务主动在 .await 处让出 CPU，而非被抢占。
        let handle = trpl::spawn_task(async {
            for i in 1..=10 {
                println!("hi number {i} from the spawned task!");
                // trpl::sleep 是异步版 sleep：挂起当前任务并让出运行时，
                // 让其他任务（main task）得以执行，500ms 后再恢复此任务。
                // 若改用 thread::sleep，会阻塞整个运行时线程，main task 也无法运行。
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        // ④ main task：与 spawn_task 并发执行。
        //   每次 .await 时让出控制权，运行时可调度 spawned task 继续执行。
        //   因此两个任务的输出会交错出现（而非顺序执行）。
        for i in 1..=5 {
            println!("hi number {i} from the main task!");
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }

        // main task 循环结束后，block_on 即将退出。
        // 注意：若此时 spawned task 还未完成，它会被直接丢弃（不等待）。
        // 如需等待，应在此处调用 handle.await。
        print_type_of(&handle, Some("demo_1:JoinHandle<()>"));
        handle.await.unwrap();//
    });
    // block_on 返回后，异步运行时已结束。

    // ⑤ 等待 OS 线程 ① 结束，防止主线程提前退出导致子线程被强制终止。
    //   .unwrap() 处理线程 panic 的情况。
    channel.join().unwrap();
}

fn demo_2() {
    // demo_2 与 demo_1 的关键区别：
    // demo_1 用 spawn_task 创建独立任务（需要 handle.await 才能等待），
    // demo_2 直接用 trpl::join 把两个 Future 组合在一起同时驱动，
    // 不需要单独的 JoinHandle，更适合"我有几个 Future，想同时等它们都完成"的场景。
    trpl::block_on(async {
        // fut1/fut2 只是普通变量，存储的是尚未执行的 Future（惰性）。
        // 此时没有任何代码运行，只是描述了"将来要做什么"。
        let fut1 = async {
            for i in 1..=5 {
                // ── 为什么 fut1、fut2 会交替打印？──────────────────────────────
                // 执行到这里打印后，紧接着遇到下一行的 .await（trpl::sleep）。
                // .await 会挂起 fut1，把控制权交还给运行时（tokio executor）。
                // 运行时发现 fut2 可以继续运行（它也在等 sleep 计时），
                // 于是切换去 poll fut2，fut2 打印自己的那行，再次 .await 挂起。
                // 500ms 后两个 sleep 先后到期，运行时依次唤醒它们继续执行。
                // 这就形成了"fut1 打印 → fut2 打印 → fut1 打印 → ..."的交替输出。
                // 本质：每个 .await 都是一个"让出点"，运行时在让出点之间切换任务。
                println!("hi number {i} from the first task!");
                // sleep 500ms 并让出控制权，运行时趁机去推进 fut2
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
            // fut1 共循环 5 次后结束，但 trpl::join 会继续等待 fut2 跑完
        };

        let fut2 = async {
            for i in 1..=10 {
                println!(" hi number {i} from the second task!");
                // 同上，sleep 500ms 并让出控制权，运行时趁机去推进 fut1
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }
            // fut2 共循环 10 次后结束
        };

        // trpl::join(fut1, fut2)：同时驱动两个 Future，直到【两个都完成】才返回。
        // 与 trpl::select 的区别：
        //   - select：谁先完成返回谁，另一个被丢弃（竞速）
        //   - join：  必须等两个都完成才返回（汇合）
        // 因为 fut2 循环 10 次而 fut1 只有 5 次，所以 fut1 结束后
        // join 会继续驱动 fut2 跑完剩余的 5 次，总耗时约 10 × 500ms = 5s。
        trpl::join(fut1, fut2).await;
    });
}

fn demo_3(){
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec!["hi", "from", "the", "future"];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received: {}", value);
        }
    });
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
