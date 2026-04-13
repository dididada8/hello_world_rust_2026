use chrono::Local;
use helloworld::print_line_separator;
use std::thread;
use std::time::Duration;
use tokio::runtime::Builder;

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

fn demo_3() {
    // demo_3: 使用线程池（tokio blocking pool）实现“多生产者 + 单 async 消费者”。
    //
    // 为什么这里要用 blocking pool：
    // 1) 生产者逻辑里包含 thread::sleep(...) 这类“阻塞调用”。
    // 2) 若阻塞调用运行在 async worker 线程上，会占住 worker，降低 runtime 调度能力。
    // 3) spawn_blocking 会把这类任务转移到专用阻塞线程池，让 async worker 继续处理 await 任务。
    // 4) 结果是：阻塞生产者与异步消费者互不拖累，吞吐和响应性更稳定。
    let (tx, mut rx) = trpl::channel();

    let producer_count = 3;
    let messages_per_producer = 4;

    // 创建多线程 runtime；spawn_blocking 的任务会进入专用阻塞线程池执行。
    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let mut producer_handles = Vec::with_capacity(producer_count);

        for producer_id in 1..=producer_count {
            let tx = tx.clone();
            // 生产者是“阻塞风格”代码（sleep + 普通循环），放进 blocking pool 更合适。
            let handle = tokio::task::spawn_blocking(move || {
                for n in 1..=messages_per_producer {
                    let thread_id = thread::current().id();
                    let now = Local::now();
                    let msg = format!("P{producer_id} -> message {n}   {:?} {now}", thread_id);

                    if tx.send(msg).is_err() {
                        break;
                    }

                    let delay_ms = 150 * producer_id as u64;
                    thread::sleep(Duration::from_millis(delay_ms));
                }
            });
            producer_handles.push(handle);
        }

        // 丢弃主任务持有的 tx，确保全部生产者结束后通道能关闭。
        drop(tx);

        // 单个 async 接收者持续消费，直到通道关闭。
        let receiver = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                println!("received: {msg}");
            }
            println!("receiver done: all producers finished and channel closed");
        });

        for handle in producer_handles {
            handle.await.unwrap();
        }
        receiver.await.unwrap();
    });
}

#[allow(dead_code)]
fn demo_4() {
    // demo_4: 阻塞日志源(thread) + async 批量写入(模拟)。
    //
    // 生产环境映射：
    // - 线程侧：对接只能阻塞读取的日志源（文件 tail、串口、遗留 SDK）。
    // - async 侧：按“条数/超时”批处理后写数据库或消息队列。
    let (tx, mut rx) = trpl::channel();

    let producer = thread::spawn(move || {
        for i in 1..=10 {
            thread::sleep(Duration::from_millis(120)); // 模拟阻塞读取
            let line = format!("log-{i} at {}", Local::now().format("%H:%M:%S%.3f"));
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let mut batch = Vec::new();
        let mut batch_id = 1usize;

        loop {
            // 250ms 内没新日志就触发一次“超时刷盘”。
            match tokio::time::timeout(Duration::from_millis(250), rx.recv()).await {
                Ok(Some(line)) => {
                    batch.push(line);
                    if batch.len() >= 3 {
                        println!("flush batch-{batch_id}: {:?}", batch);
                        batch.clear();
                        batch_id += 1;
                    }
                }
                Ok(None) => {
                    if !batch.is_empty() {
                        println!("flush final batch-{batch_id}: {:?}", batch);
                    }
                    break;
                }
                Err(_) => {
                    if !batch.is_empty() {
                        println!("flush timeout batch-{batch_id}: {:?}", batch);
                        batch.clear();
                        batch_id += 1;
                    }
                }
            }
        }
    });

    producer.join().unwrap();
}

#[allow(dead_code)]
fn demo_5() {
    // demo_5: async 请求编排 + blocking pool 做 CPU 密集计算。
    //
    // 生产环境映射：
    // - async 侧负责接请求、并发编排、聚合结果。
    // - 线程池侧负责签名/压缩/加解密/图像处理等 CPU 重任务。
    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let jobs = vec![101u64, 102, 103, 104];
        let mut handles = Vec::new();

        for job_id in jobs {
            let handle = tokio::task::spawn_blocking(move || {
                let mut acc = job_id;
                for i in 0..2_000_000u64 {
                    // 人工构造 CPU 计算，模拟签名/压缩等开销。
                    acc = acc.wrapping_mul(1_664_525).wrapping_add(i ^ 1_013_904_223);
                }
                let thread_id = thread::current().id();
                (job_id, acc, thread_id)
            });
            handles.push(handle);
        }

        let results = futures::future::join_all(handles).await;
        for result in results {
            let (job_id, checksum, thread_id) = result.unwrap();
            println!("job {job_id} done on {:?}, checksum={checksum}", thread_id);
        }
    });
}

#[allow(dead_code)]
fn demo_6() {
    // demo_6: 回调线程推送事件 + async 并发消费并限流。
    //
    // 生产环境映射：
    // - 线程侧：第三方 SDK 通过阻塞回调不断推送事件。
    // - async 侧：将事件 fan-out 到异步 I/O（HTTP/DB），并控制 in-flight 上限防止打爆下游。
    let (tx, mut rx) = trpl::channel();

    let callback_thread = thread::spawn(move || {
        for event_id in 1..=8 {
            thread::sleep(Duration::from_millis(80)); // 模拟阻塞回调间隔
            if tx
                .send(format!("event-{event_id} from callback thread"))
                .is_err()
            {
                break;
            }
        }
    });

    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        let mut in_flight = tokio::task::JoinSet::new();
        let max_in_flight = 3usize;

        while let Some(event) = rx.recv().await {
            in_flight.spawn(async move {
                // 模拟异步 I/O 处理（如调用下游服务）。
                tokio::time::sleep(Duration::from_millis(220)).await;
                format!("processed: {event}")
            });

            // 并发上限：达到阈值就先等待一个任务完成，形成背压。
            if in_flight.len() >= max_in_flight {
                let done = in_flight.join_next().await.unwrap().unwrap();
                println!("{done}");
            }
        }

        // 通道关闭后，等待剩余异步任务结束。
        while let Some(done) = in_flight.join_next().await {
            println!("{}", done.unwrap());
        }
    });

    callback_thread.join().unwrap();
}

fn main() {
    println!("===demo 1====");
    demo_1();
    print_line_separator();
    println!("===demo 2====");
    demo_2();
    print_line_separator();
    println!("===demo 3====");
    demo_3();
    print_line_separator();

    // 下面是更贴近生产的混合模式示例，按需开启：
    // demo_4();
    // print_line_separator();
    // demo_5();
    // print_line_separator();
    // demo_6();
    // print_line_separator();
}
