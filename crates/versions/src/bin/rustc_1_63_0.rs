// rustc 1.63.0 演示 —— thread::scope、array::from_fn、Mutex::new const
fn main() {
    println!("rustc 1.63.0 演示");

    // ============================================================
    println!("\n1. thread::scope —— 作用域线程可借栈数据");
    // 1.63 稳定 scoped threads：线程可借用 outer 栈帧（非 'static）数据。
    // 对比旧写法：std::thread::spawn 要求 'static，只能 Arc/Mutex 共享/克隆数据。
    let mut numbers = vec![1, 2, 3, 4, 5];
    std::thread::scope(|s| {
        // 两半分别用两个线程并发求和处理（借用同一 vec 的不同 parts）
        let mid = numbers.len() / 2; // 先取长度，避免与 split_at_mut 的可变借用纠缠
        let (left, right) = numbers.split_at_mut(mid);
        s.spawn(|| {
            left[0] += 10; // 直接借用 &mut 
        });
        s.spawn(|| {
            right[right.len() - 1] += 10;
        });
    }); // scope 块结束自动 join（不会悬空）
    println!("  作用域线程处理后 numbers = {:?}", numbers);

    // ============================================================
    println!("\n2. array::from_fn —— 按函数生成数组");
    // 1.63 稳定：usize 下标 -> 元素，生成 [T; N]。
    let squares: [u32; 6] = std::array::from_fn(|i| (i * i) as u32);
    println!("  from_fn(i -> i*i) = {:?}", squares);

    // ============================================================
    println!("\n3. Path::try_exists —— 与 exists 的区别");
    use std::path::Path;
    // try_exists：返回 Result<bool, io::Error>；能区分“可能有权限问题”与“确实不存在”。
    match Path::new("\\\\NONEXIST_PATH\\\\none").try_exists() {
        Ok(b) => println!("  try_exists 返回 = {}", b),
        Err(e) => println!("  try_exists 错误（对比 exists 只有布尔）= {:?}", e),
    }

    // ============================================================
    println!("\n4. Mutex::new / Condvar::new / RwLock::new 可 const");
    use std::sync::Mutex;
    // 1.63 起 Mutex::new 可以放在 const / static：
    static ANSWER: Mutex<i32> = Mutex::new(42);
    println!("  static Mutex 内值 = {}", *ANSWER.lock().unwrap());
    // 1.63 前：static 需要 lazy_static 或 once_cell 初始化。

    // ============================================================
    println!("\n5. VecDeque<u8> 实现 Write");
    use std::io::Write;
    let mut dq: std::collections::VecDeque<u8> = Default::default();
    // Write trait 直接写进 deque
    write!(dq, "hello").unwrap();
    let readable: Vec<u8> = dq.into_iter().collect();
    println!("  write!(VecDeque) 后内容 = {:?} ({})", readable, String::from_utf8_lossy(&readable));

    // ============================================================
    println!("\n6. 兼容性提醒");
    println!("  - #[link] 属性参数检查更严格；float→Duration 改四舍五入。");
    println!("  - cenum_impl_drop_cast lint 变 deny（cast 掉 Drop 枚举更严格）。");
}
