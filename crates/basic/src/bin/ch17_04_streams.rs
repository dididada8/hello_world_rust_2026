use futures::StreamExt;
use helloworld::print_line_separator;
use std::time::Duration;

// demo_1: 最基础的 Stream 消费模型（完整注释版）
//
// 这个例子演示了 3 个核心点：
// 1) 如何把一组普通数据变成 Stream
// 2) 如何用 next().await 按需拉取（pull）每一项
// 3) Stream 结束时为什么会得到 None
fn demo_1() {
    // block_on 会创建并驱动一个异步运行上下文，直到 async 块执行完成。
    // 在小 demo 里很常见；在生产服务里通常由 tokio runtime 驱动整个进程。
    trpl::block_on(async {
        // 普通同步数据源（数组）
        let values = [1, 2, 3, 4, 5];

        // 把迭代器/集合包装成 Stream。
        // 此时 stream 还没有被真正“消费”，只是一个可被轮询的数据流对象。
        let mut stream = trpl::stream_from_iter(values);

        // next().await:
        // - 有数据时：返回 Some(item)
        // - 数据耗尽时：返回 None（循环结束）
        //
        // 这种写法是最常见的“消费者循环”模板。
        while let Some(value) = stream.next().await {
            println!("demo_1 received: {}", value);
        }

        // 当 values 的所有元素都被消费后，next().await 返回 None，while 退出。
        // 这里不需要手动 close，数据源是有限集合，生命周期自然结束。
    });
}

// demo_2: 生产常见场景——数据清洗/校验（filter_map）
//
// 场景：消息队列里混入了空值、非法值，需要在消费前做清洗。
fn demo_2_filter_map_validation() {
    trpl::block_on(async {
        let raw_events = ["42", "  7", "", "bad", "100", " -3 "];

        let cleaned = trpl::stream_from_iter(raw_events)
            .filter_map(|raw| async move {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    return None;
                }
                trimmed.parse::<i32>().ok()
            })
            .filter(|n| futures::future::ready(*n >= 0));

        futures::pin_mut!(cleaned);
        while let Some(id) = cleaned.next().await {
            println!("demo_2 valid id: {}", id);
        }
    });
}

// demo_3: 生产常见场景——限并发处理（buffer_unordered）
//
// 场景：对外部服务发请求时，既要并发提速，也要限制并发度避免压垮下游。
fn demo_3_bounded_concurrency() {
    trpl::block_on(async {
        let jobs = 1..=8;

        let mut results = trpl::stream_from_iter(jobs)
            .map(|job_id| async move {
                // 模拟 I/O 调用（如 HTTP、DB、RPC）
                trpl::sleep(Duration::from_millis(120)).await;
                format!("job-{job_id} done")
            })
            // 同时最多跑 3 个任务；谁先完成就先产出谁。
            .buffer_unordered(3);

        while let Some(msg) = results.next().await {
            println!("demo_3 {}", msg);
        }
    });
}

// demo_4: 生产常见场景——批处理写入（chunks）
//
// 场景：日志/埋点/订单事件常以“批量写库”降低 IOPS 与事务开销。
fn demo_4_batching_chunks() {
    trpl::block_on(async {
        let events = 1..=10;

        let mut batched = trpl::stream_from_iter(events).chunks(4);

        while let Some(batch) = batched.next().await {
            // 真实场景里这里通常是一次批量 SQL / 批量 API 调用
            println!("demo_4 persist batch: {:?}", batch);
            trpl::sleep(Duration::from_millis(80)).await;
        }
    });
}

fn main() {
    println!("===demo 1====");
    demo_1();
    print_line_separator();
    println!("===demo 2====");
    demo_2_filter_map_validation();
    print_line_separator();
    println!("===demo 3====");
    demo_3_bounded_concurrency();
    print_line_separator();
    println!("===demo 4====");
    demo_4_batching_chunks();
}
