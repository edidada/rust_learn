// rustc 1.70.0 演示 —— IsTerminal、OnceCell/OnceLock、NonZero MIN/MAX、into_inner 等
fn main() {
    println!("rustc 1.70.0 演示");

    // ============================================================
    println!("\n1. std::io::IsTerminal（本版本亮点）");
    use std::io::IsTerminal;
    // 判断标准输出/错误是否连接到终端（TTY），替代了大量第三方atty crate 的场景
    println!("  stdout().is_terminal() = {}", std::io::stdout().is_terminal());
    println!("  stderr().is_terminal() = {}", std::io::stderr().is_terminal());

    // ============================================================
    println!("\n2. OnceCell / OnceLock 稳定");
    use std::cell::OnceCell;
    use std::sync::OnceLock;
    // OnceCell：单线程，无锁开销
    let cell: OnceCell<String> = OnceCell::new();
    assert!(cell.get().is_none());
    cell.set("第一次设置".to_owned()).unwrap();
    assert!(cell.set("再设无效".to_owned()).is_err());
    println!("  OnceCell::set 后 get = {:?}", cell.get());
    // OnceLock：跨线程版本
    static CONFIG: OnceLock<u32> = OnceLock::new();
    let v1 = CONFIG.get_or_init(|| 42);
    let v2 = CONFIG.get_or_init(|| 999); // 不会再执行
    println!("  OnceLock get_or_init: {} / {}（同值）", v1, v2);

    // ============================================================
    println!("\n3. NonZero*::MIN/MAX 与 is_some_and / is_ok_and");
    println!("  NonZeroU32::MIN = {}, MAX = {}", std::num::NonZeroU32::MIN, std::num::NonZeroU32::MAX);
    println!("  NonZeroI32::MIN = {}", std::num::NonZeroI32::MIN);
    println!("  Some(5).is_some_and(|x| x > 3) = {}", Some(5).is_some_and(|x| x > 3));
    println!("  Ok::<i32, &str>(2).is_ok_and(|x| x > 3) = {}", Ok::<i32, &str>(2).is_ok_and(|x| x > 3));
    println!("  Err::<i32, &str>(\"boom\").is_err_and(|e| e == \"boom\") = {}",
        Err::<i32, &str>("boom").is_err_and(|e| e == "boom"));

    // ============================================================
    println!("\n4. Rc::into_inner / Arc::into_inner");
    use std::rc::Rc;
    use std::sync::Arc;
    let only: Rc<String> = Rc::new("独占".to_owned());
    println!("  Rc 引用计数为 1 -> into_inner = {:?}", Rc::into_inner(only));
    let shared1: Rc<String> = Rc::new("共享".to_owned());
    let shared2 = shared1.clone();
    println!("  Rc 引用计数为 2 -> into_inner = {:?}", Rc::into_inner(shared1));
    drop(shared2);
    let arc = Arc::new(1);
    println!("  Arc::into_inner(独占) = {:?}", Arc::into_inner(arc));

    // ============================================================
    println!("\n5. BinaryHeap::retain");
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(vec![1, 4, 2, 9, 7]);
    heap.retain(|&x| x % 2 == 1); // 保留奇数
    println!("  retain 奇数后 = {:?}", heap.into_sorted_vec()); // [1, 7, 9]

    // ============================================================
    println!("\n6. 迭代器 Default 与 concat! 负数字面量");
    // 1.70 起大量迭代器实现 Default：default() 即空迭代器
    let it: std::vec::IntoIter<i32> = Default::default();
    println!("  std::vec::IntoIter::default() = {:?}", it.collect::<Vec<i32>>());
    let si: std::slice::Iter<'static, i32> = Default::default();
    println!("  std::slice::Iter::default() = {:?}", si.count());
    // concat! 现在接受负数字面量
    let s = concat!(-1, " 和 ", -2);
    println!("  concat!(-1, ...) = {:?}", s);

    // ============================================================
    println!("\n7. Atomic*::as_ptr");
    use std::sync::atomic::{AtomicU32, Ordering};
    let a = AtomicU32::new(7);
    let p = a.as_ptr(); // 稳定：取内部裸指针
    println!("  as_ptr 读取 = {}", unsafe { *p });
    a.store(8, Ordering::Relaxed);
    println!("  store(8) 后经指针读取 = {}", unsafe { *p });

    // ============================================================
    println!("\n8. 文字说明（Cargo/编译器）");
    println!("  - crates.io 默认 sparse 协议（1.68 稳定，1.70 默认启用）");
    println!("  - 新增 CARGO_PKG_README；稳定 cargo logout");
    println!("  - debug 构建中对指针解引用插入对齐检查，运行时暴露 UB");
}
