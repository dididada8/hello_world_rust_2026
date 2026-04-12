use helloworld::print_line_separator;
use std::sync::{Arc, Mutex};
use std::thread;

fn demo_1() {
    // Mutex::new(5) 创建一个互斥锁，将数据 5 包裹在锁内
    // Mutex<T> 的特点：
    //   - 任何时刻只有一个线程能持有锁并访问内部数据
    //   - 访问数据前必须先获取锁（lock），访问结束后必须释放锁（unlock）
    //   - Rust 通过 MutexGuard 的 Drop 机制自动释放锁，不会忘记解锁
    let m = Mutex::new(5);

    {
        // 内层作用域：用于控制锁的持有范围
        // m.lock() 尝试获取互斥锁：
        //   - 若锁当前被其他线程持有 → 阻塞，直到锁被释放
        //   - 若锁可用 → 立即获取，返回 LockResult<MutexGuard<i32>>
        //
        // .unwrap()：
        //   - lock() 返回 Result，因为若持有锁的线程 panic，锁会进入"中毒"状态
        //   - unwrap() 在锁中毒时 panic，正常情况下取出 MutexGuard<i32>
        //
        // MutexGuard<i32> 是一个智能指针：
        //   - 实现了 Deref  → 可以像 &i32 一样读取内部数据
        //   - 实现了 DerefMut → 可以像 &mut i32 一样修改内部数据
        //   - 实现了 Drop   → 离开作用域时自动释放锁（无需手动 unlock）
        let mut num = m.lock().unwrap();
        //  ^^^  必须声明为 mut，因为下一行要通过 DerefMut 修改内部值

        // *num = 6：解引用 MutexGuard，修改锁内的数据
        // num 的类型是 MutexGuard<i32>，*num 通过 DerefMut 得到 &mut i32
        // 将内部值从 5 改为 6
        *num = 6;
    } // num（MutexGuard）在此离开作用域，自动调用 Drop，锁被释放
    // 释放后其他线程或后续代码才能再次获取锁

    // 此时锁已释放，m 可以被正常访问（Debug 格式打印）
    // 输出：m = Mutex { data: 6, poisoned: false, .. }
    //   - data: 6        → 锁内数据已被修改
    //   - poisoned: false → 锁未中毒（持有锁的代码没有 panic）
    println!("m = {m:?}");
}

// demo_2：多线程共享同一个计数器，演示 Arc<Mutex<T>> 的经典用法
//
// ── Arc vs Rc 的区别 ──────────────────────────────────────────────────
// Rc<T>（Reference Counted）：
//   - 引用计数不使用原子操作，性能高，但线程不安全
//   - 未实现 Send / Sync，编译器禁止跨线程传递
//   - 只能用于单线程场景
//
// Arc<T>（Atomically Reference Counted）：
//   - 引用计数使用 CPU 原子指令（AtomicUsize），线程安全
//   - 实现了 Send + Sync，可以跨线程安全共享
//   - 代价：原子操作比普通整数操作略慢
//
// 使用场景对比：
//   单线程共享所有权  → Rc<T>         （如链表、树的多引用节点）
//   多线程共享所有权  → Arc<T>        （如线程池共享配置、共享计数器）
//   单线程内部可变性  → Rc<RefCell<T>>（运行时借用检查）
//   多线程共享可变数据 → Arc<Mutex<T>> （本例，加锁保证互斥访问）
// ──────────────────────────────────────────────────────────────────────
fn demo_2() {
    // Arc::new(Mutex::new(0))：
    //   内层 Mutex::new(0)  → 将计数器 0 用互斥锁保护，保证同一时刻只有一个线程修改
    //   外层 Arc::new(...)  → 用原子引用计数包裹，允许多个线程持有指向同一 Mutex 的指针
    //
    // 为什么不能只用 Mutex？
    //   Mutex 本身是单一所有权，无法同时被多个线程持有
    //   Arc 提供多所有权 + 线程安全，是 Mutex 在多线程中的"载体"
    let counter = Arc::new(Mutex::new(0));

    // 存放各线程的 JoinHandle，用于后续等待线程结束
    let mut handles = vec![];

    for _ in 0..10 {
        // Arc::clone(&counter)：
        //   不复制堆上的 Mutex 数据，只克隆 Arc 指针（原子地将引用计数 +1）
        //   此时 counter 和 这个新 clone 都指向同一个 Mutex<i32>
        //   每次循环引用计数 +1，循环结束后共 11 个 Arc（1个主线程 + 10个子线程）
        //
        // 为什么不直接 move counter 进闭包？
        //   move 只能转移一次，第一次循环后 counter 就失效了
        //   Arc::clone 让每个线程各持一份 Arc，共享同一 Mutex
        let counter = Arc::clone(&counter);
        //  ^^^^^^^ 变量遮蔽：新的 counter 是 Arc 的一份 clone，
        //          后面的 move 闭包会 move 这份 clone，不影响外层的 counter

        let handle = thread::spawn(move || {
            // counter（clone 的那份 Arc）被 move 进闭包，子线程持有所有权
            //
            // counter.lock()：通过 Arc 的 Deref 透明访问内部 Mutex，申请锁
            //   - 若其他线程持有锁 → 阻塞等待
            //   - 获取成功 → 返回 MutexGuard<i32>
            let mut num = counter.lock().unwrap();

            // *num += 1：通过 DerefMut 修改锁内的计数器值
            // 由于 Mutex 保证互斥，10 个线程的 += 1 不会产生数据竞争
            *num += 1;

            // num（MutexGuard）在此 drop，锁自动释放，下一个线程可以获取锁
            // counter（Arc clone）在此 drop，引用计数 -1
        });

        handles.push(handle); // 保存 handle，防止子线程被立即 detach
    }

    // join() 等待每个子线程执行完毕，保证所有 += 1 都已完成
    // 若不 join，主线程可能在子线程结束前就打印结果
    for handle in handles {
        handle.join().unwrap();
    }
    // 此时 10 个子线程的 Arc clone 都已 drop，引用计数回到 1（只剩主线程的 counter）
    //
    // 为什么到这里 10 个 clone 都已 drop？
    //   每个子线程的 Arc clone 是通过 move 闭包进入子线程的，
    //   闭包本身就是子线程的"栈帧"，子线程函数体执行完毕时栈帧销毁，
    //   闭包捕获的所有变量（包括 counter 这份 Arc clone）都会被 drop。
    //
    //   而 handle.join().unwrap() 会阻塞主线程，直到对应子线程完全结束。
    //   上面的 for 循环对全部 10 个 handle 都调用了 join()，
    //   所以执行到这一行时，10 个子线程都已经结束 → 10 份 Arc clone 都已 drop。
    //
    //   时间线：
    //     子线程结束（闭包退出）→ counter clone drop → 引用计数 -1
    //     join() 返回           → 确认该线程已结束
    //     10 次 join() 全部返回 → 10 份 clone 全部 drop → 引用计数从 11 降回 1

    // counter.lock().unwrap()：最后一次获取锁，读取最终计数值
    // *(...) 解引用 MutexGuard 得到 i32
    // 输出：Result: 10
    println!("Result: {}", *counter.lock().unwrap());
    // counter 在此 drop，引用计数降为 0，Mutex 和堆上数据被释放
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
