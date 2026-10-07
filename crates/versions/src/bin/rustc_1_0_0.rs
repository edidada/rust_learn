// rustc 1.0.0 演示 —— 1.0 基线：所有权 / 借用 / trait / 索引 / 拷贝
// 附：docs/multy_thread.md 并发知识图谱之 1.0 即稳定的基线层（第 10~21 节）
fn main() {
    println!("rustc 1.0.0 演示");

    println!("\n1. 所有权与移动（move 语义，1.0 即有）");
    // vec 拥有堆上数据；赋值给 moved 后所有权转移，原变量不可再用
    let vec = vec![10, 20, 30];
    let moved = vec; // 移动
    println!("moved = {}（原 vec 所有权已转移）", moved[0]);

    println!("\n2. 借用：不可变借用 vs 可变借用");
    let mut s = String::from("hello");
    let len = s.len(); // &self 只读借用
    s.push_str(" world"); // &mut self 可变借用
    println!("s = \"{}\"（len 读取借用在先，随后可变借用）", s);

    println!("\n3. 生命周期标注基线");
    let a = String::from("abc");
    let b = String::from("def");
    let longer = longest(&a, &b);
    println!("longest = {}", longer);

    println!("\n4. trait + 默认方法（1.0 起扩展 trait 已并入核心 trait）");
    // 1.0 把 IteratorExt 的方法并入 Iterator trait 本身，迭代器链从此开箱即用
    let sum: i32 = [1, 2, 3, 4].iter().filter(|x| **x % 2 == 0).sum();
    println!("偶数之和 = {}", sum);

    println!("\n5. UFCS：不带 trait 的关联路径 MyType::default()");
    struct Counter;
    impl Default for Counter {
        fn default() -> Self {
            Counter
        }
    }
    // 1.0 起可以直接写 Counter::default()
    let _c = Counter::default();
    println!("Counter::default() 通过 UFCS 调用成功");

    println!("\n6. Index/IndexMut 按值索引");
    let mut map = std::collections::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    // 1.0 起 Index 以值方式取索引，"string" 字面量可直接索引
    println!("map[\"one\"] = {}", map["one"]);

    println!("\n7. Copy 继承 Clone");
    let n = 7;
    let m = n.clone(); // Copy 类型必然实现 Clone
    println!("n = {}, m = {}（Copy 类型 clone 后原值仍可用）", n, m);

    println!("\n8. match / Result / Option 基线");
    let r: Result<i32, String> = Ok(40);
    match r {
        Ok(v) => println!("match Ok({})", v),
        Err(_) => println!("Err 分支未进入"),
    }
    let o = Some("value");
    println!("option 匹配 = {}", o.unwrap_or("fallback"));

    println!("\n9. 当年老 API 与现行等价物的对照");
    // 当年形态：`std::thread::sleep_ms(1)`；毫秒计时 API 后来被 Duration 版取代
    // 现行等价物：`std::thread::sleep(std::time::Duration::from_millis(1))`
    std::thread::sleep(std::time::Duration::from_millis(1));
    println!("sleep 一毫秒：当年 thread::sleep_ms(1) -> 现行 thread::sleep(Duration)");

    // ================= 以下对应 docs/multy_thread.md 的 Rust 并发基线（1.0 即稳定） =================

    println!("\n10. L04 OS 线程模型：spawn / join / 命名线程（1.0 即有）");
    // 主线 B：Thread -> join -> 获得结果，这是 Future 出现前的传统形态
    let h = std::thread::spawn(|| 6 * 7);
    println!("   join() = {}（main 阻塞等待工作线程结束）", h.join().unwrap());
    let h2 = std::thread::Builder::new()
        .name("worker-1".to_string())
        .spawn(|| {
            println!("   命名线程运行中，current name = {:?}", std::thread::current().name());
        })
        .unwrap();
    let _ = h2.join();

    println!("\n11. L16 Arc<Mutex<T>>：类型系统把 data race 推到编译期");
    // multy_thread.md 第十六节：Rust 的特色是 Ownership+Send+Sync 让非法共享编译失败，
    // 而不是运行出 data race 后凌晨三点 production 崩溃。
    use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(0usize));
    let mut hs = Vec::new();
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        hs.push(std::thread::spawn(move || {
            // move 闭包捕获的是 Arc 的所有权克隆，不是引用——线程要求 'static
            for _ in 0..10_000 {
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in hs {
        let _ = h.join();
    }
    println!(
        "   4 线程各加 10000，结果 = {}（无锁裸 ++ 就是第二层讲的 lost update）",
        *counter.lock().unwrap()
    );

    println!("\n12. L06 Data Race ≠ Race Condition：check-then-act 原子性违规");
    // 每一步都在锁里（语言意义上没有 data race），但「查余额」与「扣款」是两个独立临界区。
    // 用 Barrier 强制两线程都先完成 check -> act 必然交错，覆写结果可复现。
    use std::sync::Barrier;
    let balance = Arc::new(Mutex::new(100i32));
    let gate = Arc::new(Barrier::new(2));
    let mut hs = Vec::new();
    for _ in 0..2 {
        let bal = Arc::clone(&balance);
        let g = Arc::clone(&gate);
        hs.push(std::thread::spawn(move || {
            let enough = *bal.lock().unwrap() >= 60; // check
            g.wait(); // 会合：保证两个线程都已 check 完（此时余额还是 100）
            if enough {
                *bal.lock().unwrap() -= 60; // act：两个都通过检查 -> 双双扣款
            }
        }));
    }
    for h in hs {
        let _ = h.join();
    }
    println!(
        "   余额 100，两笔 60 提现后 = {}（透支即 TOCTOU/atomicity violation，但全程无 data race）",
        *balance.lock().unwrap()
    );

    println!("\n13. L05 条件变量 Condvar：生产者/消费者的等待与唤醒");
    use std::sync::Condvar;
    let queue: Arc<(Mutex<Vec<i32>>, Condvar)> = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    {
        let q = Arc::clone(&queue);
        let producer = std::thread::spawn(move || {
            for i in 1..=3 {
                q.0.lock().unwrap().push(i);
                q.1.notify_one(); // 唤醒一个等待者（第五层 monitor 的底层件）
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            q.0.lock().unwrap().push(-1); // 哨兵：告知结束
            q.1.notify_one();
        });
        let mut consumed = Vec::new();
        let mut guard = queue.0.lock().unwrap();
        loop {
            while guard.is_empty() {
                // wait：原子地「释放锁 + 挂起」，被唤醒后重新持锁返回新 guard（循环防虚假唤醒）
                guard = queue.1.wait(guard).unwrap();
            }
            let v = guard.pop().unwrap();
            if v == -1 {
                break;
            }
            consumed.push(v);
        }
        let _ = producer.join();
        consumed.sort();
        println!("   消费线程收到 = {:?}", consumed);
    }

    println!("\n14. L05 RwLock：多读单写");
    use std::sync::RwLock;
    let shared = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut hs = Vec::new();
    for _ in 0..3 {
        let s = Arc::clone(&shared);
        hs.push(std::thread::spawn(move || {
            let r = s.read().unwrap(); // 读锁可并发持有
            println!("   读线程求和 = {}", r.iter().sum::<i32>());
        }));
    }
    let writer = {
        let s = Arc::clone(&shared);
        std::thread::spawn(move || {
            let mut w = s.write().unwrap(); // 写锁独占
            w.push(4);
            println!("   写线程追加了 4");
        })
    };
    for h in hs {
        let _ = h.join();
    }
    let _ = writer.join();
    println!("   最终数据 = {:?}", shared.read().unwrap());

    println!("\n15. L05 Barrier：N 线程对齐同一阶段再继续（可复用，类 Java CyclicBarrier）");
    let barrier = Arc::new(Barrier::new(3));
    let mut hs = Vec::new();
    for id in 0..3 {
        let b = Arc::clone(&barrier);
        hs.push(std::thread::spawn(move || {
            b.wait(); // 阶段 1 会合
            b.wait(); // 阶段 2 会合：Barrier 自动重置
            println!("   worker {} 完成两个阶段同步", id);
        }));
    }
    for h in hs {
        let _ = h.join();
    }

    println!("\n16. L03/L07 原子量与内存序（AtomicUsize::fetch_add / load / store，1.0 即有）");
    // 第三层三问：原子性、可见性、顺序性。Ordering 全家桶
    // Relaxed/Acquire/Release/AcqRel/SeqCst 自 1.0 可用；
    // 带回旧值的 compare_exchange（第七层 CAS）1.10 才稳定，见 rustc/1.10.0 分支演示。
    use std::sync::atomic::{AtomicUsize, Ordering};
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut hs = Vec::new();
    for _ in 0..4 {
        let a = Arc::clone(&atomic_counter);
        hs.push(std::thread::spawn(move || {
            for _ in 0..10_000 {
                // 原子 RMW：读-改-写不可分割，对应 x86 的 LOCK XADD（第二层 ISA）
                a.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in hs {
        let _ = h.join();
    }
    println!(
        "   4×10000 fetch_add(SeqCst) = {}，不加锁也绝不丢更新",
        atomic_counter.load(Ordering::SeqCst)
    );

    println!("\n17. L12 消息传递 mpsc::channel（不共享内存来通信 vs Go 的 chan 同主线 D）");
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel::<String>();
    let mut hs = Vec::new();
    for id in 0..3 {
        let tx = tx.clone();
        hs.push(std::thread::spawn(move || {
            tx.send(format!("task-{}", id)).unwrap();
        }));
    }
    drop(tx); // 最后一个发送端释放后，recv 返回 Err，循环自然结束
    let mut msgs: Vec<String> = Vec::new();
    while let Ok(m) = rx.recv() {
        msgs.push(m);
    }
    for h in hs {
        let _ = h.join();
    }
    msgs.sort();
    println!("   主线程经 channel 收到 = {:?}", msgs);

    println!("\n18. thread_local!：每线程一份私有数据，无共享即无竞争");
    use std::cell::RefCell;
    thread_local! {
        static LOCAL_COUNT: RefCell<u32> = RefCell::new(0);
    }
    let mut hs = Vec::new();
    for _ in 0..3 {
        hs.push(std::thread::spawn(|| {
            LOCAL_COUNT.with(|c| *c.borrow_mut() += 10);
            LOCAL_COUNT.with(|c| *c.borrow_mut() += 1);
            LOCAL_COUNT.with(|c| println!("   本线程私有计数 = {}（线程间互不影响）", c.borrow()));
        }));
    }
    for h in hs {
        let _ = h.join();
    }

    println!("\n19. L05 Once：一次性全局初始化（lazy init 原语；call_once 语义 1.0 即有）");
    use std::sync::Once;
    static INIT: Once = Once::new();
    static mut CONFIG_VALUE: u32 = 0;
    INIT.call_once(|| unsafe { CONFIG_VALUE = 42 });
    INIT.call_once(|| unsafe { CONFIG_VALUE = 999 }); // 第二次不会执行
    println!("   配置值 = {}（仅首次 call_once 生效）", unsafe { CONFIG_VALUE });

    println!("\n20. L16 Send / Sync：并发正确性前移到类型系统");
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<String>(); // String 能整体搬到另一线程
    assert_sync::<Mutex<i32>>(); // &Mutex<i32> 能跨线程共享
    assert_send::<mpsc::Sender<String>>();
    // 下面两行取消注释即编译失败——这正是 Rust「非法共享 -> 编译期拒绝」：
    // assert_send::<std::rc::Rc<i32>>(); // Rc 非 Send：引用计数不是原子加减
    // assert_sync::<RefCell<i32>>();     // RefCell 非 Sync：借用计数不受锁保护
    println!("   Send/Sync 断言通过；Rc/RefCell 若跨线程共享会被编译器直接拒绝");

    println!("\n21. L06 deadlock 之一：lock-order inversion 的修复（统一按地址序拿锁）");
    let acct_a = Arc::new(Mutex::new(100i32));
    let acct_b = Arc::new(Mutex::new(0i32));
    let (ta, tb) = (Arc::clone(&acct_a), Arc::clone(&acct_b));
    let t1 = std::thread::spawn(move || transfer(&ta, &tb, 30)); // A -> B 转 30
    let (tb2, ta2) = (Arc::clone(&acct_b), Arc::clone(&acct_a));
    let t2 = std::thread::spawn(move || transfer(&tb2, &ta2, 20)); // B -> A 转 20
    let _ = t1.join();
    let _ = t2.join();
    // 注意：println! 参数里的 MutexGuard 临时量会活到整条语句结束，
    // 同一把锁在一条语句里锁两次会自锁死（std Mutex 不可重入）——先取值再打印。
    let bal_a = *acct_a.lock().unwrap();
    let bal_b = *acct_b.lock().unwrap();
    println!("   双向转账后余额 = {} / {}，总额 {} 不变且未死锁", bal_a, bal_b, bal_a + bal_b);
}

// 转账：两个线程即使方向相反，也按同一全局顺序（Mutex 地址升序）拿锁，破坏循环等待条件。
// 若各自先锁 from 再锁 to，两线程互等对方持有的锁 -> deadlock（第六层 lock-order inversion）。
fn transfer(from: &std::sync::Mutex<i32>, to: &std::sync::Mutex<i32>, amount: i32) {
    let from_addr = from as *const std::sync::Mutex<i32> as usize;
    let to_addr = to as *const std::sync::Mutex<i32> as usize;
    if from_addr < to_addr {
        let mut f = from.lock().unwrap();
        let mut t = to.lock().unwrap();
        *f -= amount;
        *t += amount;
    } else {
        let mut t = to.lock().unwrap();
        let mut f = from.lock().unwrap();
        *f -= amount;
        *t += amount;
    }
}

// 泛型生命周期：'a 表示两个参数中被持有较短者的寿命
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}
