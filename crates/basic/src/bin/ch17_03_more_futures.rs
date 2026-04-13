use helloworld::print_line_separator;

fn slow(name: &str, duration: std::time::Duration) {
    println!("  '{name}' is slowing down for {}ms", duration.as_millis());
    std::thread::sleep(duration);
}

fn demo_1() {
    trpl::block_on(async {
        let fut1 = async {
            println!("'a' started.");
            slow("a", std::time::Duration::from_millis(30));
            println!("'a' ran for 30ms");
            slow("a", std::time::Duration::from_millis(10));
            println!("'a' ran for 10ms");
            slow("a", std::time::Duration::from_millis(20));
            println!("'a' ran for 20ms");
            println!("'a' finished.");
        };

        let fut2 = async {
            println!("'b' started.");
            slow("b", std::time::Duration::from_millis(75));
            println!("'b' ran for 75ms");
            slow("b", std::time::Duration::from_millis(10));
            println!("'b' ran for 10ms");
            slow("b", std::time::Duration::from_millis(15));
            println!("'b' ran for 15ms");
            slow("b", std::time::Duration::from_millis(350));
            println!("'b' ran for 350ms");
        };

        trpl::select(fut1, fut2).await;
    });
}

fn demo_2() {
    trpl::block_on(async {
        let fut1 = async {
            println!("'a' started.");
            trpl::sleep(std::time::Duration::from_millis(30)).await;
            println!("'a' ran for 30ms");
            trpl::sleep(std::time::Duration::from_millis(10)).await;
            println!("'a' ran for 10ms");
            trpl::sleep(std::time::Duration::from_millis(20)).await;
            println!("'a' ran for 20ms");
            println!("'a' finished.");
        };

        let fut2 = async {
            println!("'b' started.");
            trpl::sleep(std::time::Duration::from_millis(75)).await;
            println!("'b' ran for 75ms");
            trpl::sleep(std::time::Duration::from_millis(10)).await;
            println!("'b' ran for 10ms");
            trpl::sleep(std::time::Duration::from_millis(15)).await;
            println!("'b' ran for 15ms");
            trpl::sleep(std::time::Duration::from_millis(350)).await;
            println!("'b' ran for 350ms");
        };

        trpl::select(fut1, fut2).await;
    });
}

// demo_3：在必须使用阻塞操作时，通过 yield_now() 手动让出控制权，
// 实现多个 future 的协作式并发。
// 这是 demo_1（纯阻塞，无并发）和 demo_2（异步 sleep，自动让出）的折中方案。
fn demo_3() {
    trpl::block_on(async {
        // ──────────────────────────────────────────────
        // yield_now() 的原理：
        //   - 第一次被 poll 时，立即返回 Poll::Pending，
        //     同时把自己注册为"立刻就绪"，通知运行时马上再次 poll 自己。
        //   - 运行时因为收到通知，会在下次调度时立即再次 poll 它，
        //     第二次 poll 时返回 Poll::Ready(())，执行继续。
        //   - 关键效果：在两次 poll 之间，运行时有机会先去 poll 其他 future，
        //     从而实现协作式切换（cooperative multitasking）。
        //
        // 与 sleep(Duration).await 的区别：
        //   - sleep：让出控制权并等待指定时间，期间 future 不会被 poll。
        //   - yield_now：让出控制权但"立刻"重新就绪，时间代价几乎为零，
        //     仅仅是给运行时一个插入其他任务的机会。
        // ──────────────────────────────────────────────

        let fut1 = async {
            println!("'c', started.");

            // slow() 是阻塞调用，会占用线程 30ms，期间运行时完全无法调度其他 future
            slow("'c'", std::time::Duration::from_millis(30));
            println!("'a' ran for 30ms");

            // ★ yield_now()：主动让出控制权给运行时
            //   fut1 在此暂停（Poll::Pending），运行时转去 poll fut2，
            //   fut2 执行直到它的下一个 yield_now() 或完成，
            //   之后运行时再回来继续 fut1。
            trpl::yield_now().await;

            slow("'c'", std::time::Duration::from_millis(10));
            println!("'c', ran for 10ms");

            // ★ 再次让出，给 fut2 一次运行机会
            trpl::yield_now().await;

            slow("'c'", std::time::Duration::from_millis(20));
            println!("'c' ran for 20ms");
            println!("'c' finished.");
            // fut1 结束后，trpl::select 检测到第一个完成的 future，立即返回，
            // fut2 剩余部分被丢弃（不再执行）。
        };

        let fut2 = async {
            // fut2 在 fut1 第一次 yield_now() 之后才有机会开始运行
            println!("'d' started.");

            slow("'d'", std::time::Duration::from_millis(75));
            println!("'d' ran for 75ms");

            // ★ fut2 也在此让出，运行时转回 fut1 继续执行
            trpl::yield_now().await;

            slow("'d'", std::time::Duration::from_millis(10));
            println!("'d' ran for 10ms");

            // ★ 再次让出
            trpl::yield_now().await;

            slow("'d'", std::time::Duration::from_millis(15));
            println!("'d' ran for 15ms");

            // ★ 最后一次让出（此时 fut1 可能已经完成并触发 select 返回，
            //   导致 fut2 的这行 yield_now 之后的代码不会被执行）
            trpl::yield_now().await;
        };

        // select：同时 poll fut1 和 fut2，哪个先完成就返回，另一个被丢弃。
        // 由于 yield_now 提供了切换点，两个 future 得以交替推进，
        // 而不像 demo_1 那样 fut1 独占线程直到结束。
        trpl::select(fut1, fut2).await;
    });
}


fn demo_4() {
    async fn timeout<F, T>(future_to_try: F, max_time: std::time::Duration) -> Result<T, std::time::Duration>
    where
        F: std::future::Future<Output = T>,
    {
        let timer = trpl::sleep(max_time);
        match trpl::select(future_to_try, timer).await {
            trpl::Either::Left(output) => Ok(output),
            trpl::Either::Right(_) => Err(max_time),
        }
    }

    trpl::block_on(async {
        let slow = async {
            trpl::sleep(std::time::Duration::from_secs(5)).await;
            "Finally finished!"
        };

        match timeout(slow, std::time::Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with: {}", message),
            Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
        }
    });
}

// ══════════════════════════════════════════════════════════════════
// 三种并发方式对比
// ══════════════════════════════════════════════════════════════════
//
//  ┌─────────┬──────────────────────────┬───────────┬───────────────────────────────────────┐
//  │         │ sleep 方式               │ fut2 启动 │ 适用场景                              │
//  ├─────────┼──────────────────────────┼───────────┼───────────────────────────────────────┤
//  │ demo_1  │ std::thread::sleep       │     ❌    │ 无并发需求的纯顺序阻塞代码            │
//  │         │ （阻塞线程，无让出点）   │           │                                       │
//  ├─────────┼──────────────────────────┼───────────┼───────────────────────────────────────┤
//  │ demo_2  │ trpl::sleep().await      │     ✅    │ 纯异步场景（首选方案）                │
//  │         │ （异步，自动让出控制权） │           │ sleep 期间自动切换到其他 future       │
//  ├─────────┼──────────────────────────┼───────────┼───────────────────────────────────────┤
//  │ demo_3  │ std::thread::sleep       │     ✅    │ 必须保留阻塞 API，但仍需协作并发      │
//  │         │ + yield_now().await      │           │ 手动在每段阻塞操作后插入切换点        │
//  │         │ （阻塞后手动让出）       │           │                                       │
//  └─────────┴──────────────────────────┴───────────┴───────────────────────────────────────┘
//
// 核心结论：
//   • 阻塞调用（thread::sleep / 文件 IO / CPU 密集计算）会独占整个线程，
//     运行时在阻塞期间无法调度任何其他 future。
//   • 解决方案一（推荐）：将阻塞操作替换为对应的异步版本，用 .await 自动让出。
//   • 解决方案二（兜底）：保留阻塞操作，但在每段阻塞之后手动插入 yield_now().await，
//     为运行时制造协作切换点（缺点：需要开发者在每处阻塞后手动管理切换位置）。
// ══════════════════════════════════════════════════════════════════

fn main() {
    println!("=== demo_1: 使用阻塞 sleep，fut2 无法启动 ===");
    let t = std::time::Instant::now();
    demo_1();
    println!("demo_1 耗时: {:?}", t.elapsed());
    print_line_separator();

    println!("=== demo_2: 使用异步 sleep，fut1 和 fut2 交替执行 ===");
    let t = std::time::Instant::now();
    demo_2();
    println!("demo_2 耗时: {:?}", t.elapsed());
    print_line_separator();

    println!("=== demo_3: 使用 yield_now，fut1 和 fut2 交替执行 ===");
    let t = std::time::Instant::now();
    demo_3();
    println!("demo_3 耗时: {:?}", t.elapsed());

    print_line_separator();
    println!("=== demo_4:");
    let t = std::time::Instant::now();
    demo_4();
    println!("demo_4 耗时: {:?}", t.elapsed());
}
