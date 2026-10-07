// rustc 1.10.0 演示 —— panic hook / compare_exchange / CStr / 二分查找
// 附：docs/multy_thread.md 第三层 happens-before 与第七层 Lock-Free（compare_exchange 族恰在 1.10 稳定）
use std::ffi::CStr;
use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("rustc 1.10.0 演示");

    println!("\n1. panic::set_hook / take_hook / PanicInfo::location（1.10 稳定）");
    // 换上自定义 panic 钩子，捕获 PanicInfo 的位置信息，之后恢复默认钩子
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(|info| {
        if let Some(loc) = info.location() {
            println!("   [hook] panic 位于 {}:{}（payload 动态获取略）", loc.file(), loc.line());
        }
    }));
    let _ = panic::catch_unwind(|| panic!("hook 触发"));
    panic::set_hook(default_hook);
    println!("钩子已恢复默认");

    println!("\n2. AtomicUsize::compare_exchange（1.10 稳定）");
    let a = AtomicUsize::new(10);
    // 期望 10，成功换成 20；CompareAndSwap 的"带回旧值"版本
    let r = a.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst);
    println!("compare_exchange(10->20) = {:?}，现值 = {}", r, a.load(Ordering::SeqCst));
    let r2 = a.compare_exchange(10, 30, Ordering::SeqCst, Ordering::SeqCst);
    println!("compare_exchange(10->30) 失败返回旧值 {:?}", r2);

    println!("\n3. CStr::from_bytes_with_nul（1.10 稳定）");
    let cs = CStr::from_bytes_with_nul(b"safe\0");
    match cs {
        Ok(s) => println!("合法以 NUL 结尾的字节串 -> {:?}", s),
        Err(e) => println!("FromBytesWithNulError: {}", e),
    }
    println!("缺 NUL 会报错：{:?}", CStr::from_bytes_with_nul(b"bad").is_err());

    println!("\n4. binary_search_by_key");
    let v = [1, 3, 5, 7, 9];
    println!("按 x%%10 找 7 -> {:?}", v.binary_search_by_key(&7, |x| x % 10));

    println!("\n5. Weak::new（1.10 稳定）");
    let weak: std::sync::Weak<usize> = std::sync::Weak::new();
    println!("空 Weak upgrade = {:?}", weak.upgrade());

    println!("\n6. panic=abort / cdylib（println 讲解节）");
    // 当年形态：-C panic=abort 让 panic 直接中止进程（RFC 1513）；
    // 新 crate 类型 cdylib 用于生成给 C/其他语言加载的动态库（RFC 1510）。
    // 两者均为编译/构建配置，无法在单文件演示里直接体现。
    println!("-C panic=abort：panic 不再展开栈而是 abort；cdylib：跨语言动态库目标。");

    // ===== 以下对应 docs/multy_thread.md 第三层（内存模型）/ 第七层（Lock-Free）=====
    // compare_exchange / compare_exchange_weak 恰在本版本（1.10）稳定——无锁结构的语言级入口。

    println!("\n7. L07 无锁栈 Treiber Stack：CAS 循环替代互斥锁");
    struct Node {
        val: u64,
        next: *mut Node,
    }
    struct TreiberStack {
        head: std::sync::atomic::AtomicPtr<Node>,
    }
    impl TreiberStack {
        fn new() -> Self {
            TreiberStack {
                head: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()),
            }
        }
        fn push(&self, node: Box<Node>) {
            let ptr = Box::into_raw(node);
            let mut cur = self.head.load(Ordering::Acquire);
            loop {
                unsafe { (*ptr).next = cur };
                // 强 CAS：值等则交换，失败也绝不假摔（x86 上对应 LOCK CMPXCHG，第二层汇编）
                match self
                    .head
                    .compare_exchange(cur, ptr, Ordering::AcqRel, Ordering::Acquire)
                {
                    Ok(_) => return,
                    Err(actual) => cur = actual, // 失败时拿到新观测值，重试
                }
            }
        }
        fn pop(&self) -> Option<Box<Node>> {
            let mut cur = self.head.load(Ordering::Acquire);
            loop {
                if cur.is_null() {
                    return None;
                }
                let next = unsafe { (*cur).next };
                // 弱 CAS：允许伪失败（对应 ARM LL/SC 被打断），必须配循环使用
                match self
                    .head
                    .compare_exchange_weak(cur, next, Ordering::AcqRel, Ordering::Acquire)
                {
                    Ok(_) => return Some(unsafe { Box::from_raw(cur) }),
                    Err(actual) => cur = actual,
                }
            }
        }
    }
    impl Drop for TreiberStack {
        fn drop(&mut self) {
            while self.pop().is_some() {}
        }
    }
    let stack = std::sync::Arc::new(TreiberStack::new());
    let mut handles = Vec::new();
    for id in 0..2u64 {
        let s = std::sync::Arc::clone(&stack);
        handles.push(std::thread::spawn(move || {
            for i in 0..50u64 {
                s.push(Box::new(Node {
                    val: id * 1000 + i,
                    next: std::ptr::null_mut(),
                }));
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    let mut vals: Vec<u64> = Vec::new();
    while let Some(n) = stack.pop() {
        vals.push(n.val);
    }
    vals.sort();
    println!(
        "   2 线程无锁各 push 50 项，回收 {} 项，min = {} max = {}（无一把 mutex）",
        vals.len(),
        vals[0],
        vals[vals.len() - 1]
    );
    // 注：pop 全部发生在 join 之后。并发 push+pop 的真实 Treiber 栈存在经典 ABA 隐患，见下一节。

    println!("\n8. L07 ABA：CAS 只比值的盲区 + 版本号（tagged CAS）对策");
    // 数值版：1 -> 2 -> 1，CAS(期望1 -> 3) 依然成功——值回到原点，中间历史被抹掉。
    let x = AtomicUsize::new(1);
    let snapshot = x.load(Ordering::SeqCst);
    x.store(2, Ordering::SeqCst);
    x.store(1, Ordering::SeqCst);
    println!(
        "   无标记 CAS(1->3) 成功 = {}（ABA：预期值未变，但对象可能已被回收复用）",
        x.compare_exchange(snapshot, 3, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    );
    // 对策一：tagged pointer——(值, 版本) 打包进一个 64 位原子量，版本单调递增。
    let pack = |val: u32, tag: u32| -> u64 { ((val as u64) << 32) | tag as u64 };
    let cell = std::sync::atomic::AtomicU64::new(pack(10, 0));
    let seen = cell.load(Ordering::SeqCst); // 观测到 (10, tag=0)
    cell.store(pack(20, 1), Ordering::SeqCst);
    cell.store(pack(10, 2), Ordering::SeqCst); // 值回到 10（ABA），tag 已变
    let tagged = cell.compare_exchange(seen, pack(30, 99), Ordering::SeqCst, Ordering::SeqCst);
    println!(
        "   tagged CAS 识破 ABA：成功 = {}，旧值 = (val={}, tag={})",
        tagged.is_ok(),
        (tagged.unwrap_err() >> 32) as u32,
        (tagged.unwrap_err() & 0xffff_ffff) as u32
    );
    // 对策二（工业级）：Hazard Pointer / Epoch Based Reclamation / RCU——
    // 即 multy_thread.md 第七层的 memory reclamation 子树，核心是「何时能安全释放」。

    println!("\n9. L03 happens-before：Release 写 + Acquire 读 = 安全发布");
    // 经典双字发布协议：非原子的 DATA 先写，原子的 READY 后置位；
    // Release/Acquire 建立 synchronizes-with，消费端读 READY 成功后必能看到 DATA 的完整内容。
    // （若用 Relaxed，编译器/CPU 可把两次写重排 -> 消费端看到 READY=true 却读到半成品）
    static READY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    static mut DATA: *mut String = std::ptr::null_mut();
    let msg = Box::new("经 Release/Acquire 发布的消息".to_string());
    unsafe { DATA = Box::into_raw(msg) }; // 第一步：写数据（普通写，无原子）
    READY.store(true, Ordering::Release); // 第二步：置标志，RELEASE 屏障
    let consumer = std::thread::spawn(|| {
        while !READY.load(Ordering::Acquire) {
            // ACQUIRE 屏障：成功读到此标志后，之前 RELEASE 前的写入全部可见
            std::thread::yield_now();
        }
        let p = unsafe { DATA };
        println!("   消费线程安全读到: {}", unsafe { &*p });
        drop(unsafe { Box::from_raw(p) }); // 收回所有权，不泄漏
    });
    let _ = consumer.join();
    println!("   Relaxed 只保原子性不保顺序；AcqRel 兼具两侧屏障（多用于 RMW）");

    println!("\n10. L07 进展性层级：lock-free ≠ wait-free");
    // lock-free：CAS 循环——单个线程可能反复失败（饿死），但系统整体总有线程前进。
    let mut retries = 0u32;
    let y = AtomicUsize::new(0);
    loop {
        let cur = y.load(Ordering::SeqCst);
        if cur >= 5 {
            break;
        }
        match y.compare_exchange(cur, cur + 1, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {}
            Err(_) => retries += 1, // 本演示单线程必然 0 次重试
        }
    }
    println!("   CAS 循环加到 5：重试 {} 次（单线程无竞争；竞争越烈重试越多）", retries);
    // wait-free：任何操作有限步内完成——fetch_add 一条 LOCK XADD 搞定，无重试。
    let z = std::sync::Arc::new(AtomicUsize::new(0));
    let mut hs = Vec::new();
    for _ in 0..4 {
        let a = std::sync::Arc::clone(&z);
        hs.push(std::thread::spawn(move || {
            for _ in 0..100 {
                a.fetch_add(1, Ordering::SeqCst); // 硬件级 wait-free RMW
            }
        }));
    }
    for h in hs {
        let _ = h.join();
    }
    println!("   fetch_add(SEQ_CST) 4×100 = {}（wait-free：每步一条原子指令，无循环重试）", z.load(Ordering::SeqCst));
    println!("   另：obstruction-free 介于两者之间——单线程独占时能前进（详见第二十一层对比表）");
}
