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

fn main() {
    demo_1();
    print_line_separator();
}
