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

fn demo_2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
