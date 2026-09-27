// rustc 1.73.0 演示 —— div_ceil、LocalKey 便捷方法、Bound 切片、Arc<File>
fn main() {
    println!("rustc 1.73.0 演示");

    // ============================================================
    println!("\n1. 无符号整数 div_ceil / next_multiple_of");
    println!("  7u32.div_ceil(2) = {}", 7u32.div_ceil(2)); // 4
    println!("  10u64.div_ceil(3) = {}", 10u64.div_ceil(3)); // 4
    println!("  10u32.next_multiple_of(7) = {}", 10u32.next_multiple_of(7)); // 14
    println!("  14u32.next_multiple_of(7) = {}", 14u32.next_multiple_of(7)); // 14
    println!("  u32::MAX.checked_next_multiple_of(2) = {:?}", u32::MAX.checked_next_multiple_of(2)); // None

    // ============================================================
    println!("\n2. LocalKey::<Cell<T>> / LocalKey::<RefCell<T>> 便捷方法");
    thread_local! {
        static COUNT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
        static NAME: std::cell::RefCell<String> = std::cell::RefCell::new(String::from("初始"));
    }
    // 1.73 起无需 .with(|c| ...) 包一层
    COUNT.set(COUNT.get() + 5);
    println!("  Cell::get/set: COUNT = {}", COUNT.get());
    COUNT.replace(COUNT.get() * 2);
    println!("  Cell::replace 后 = {}", COUNT.take());
    COUNT.set(1); // take 清零后重置
    NAME.with_borrow_mut(|n| n.push_str(" + 追加"));
    println!("  RefCell::with_borrow: {}", NAME.with_borrow(|n| n.clone()));

    // ============================================================
    println!("\n3. 用 Bound 做切片索引");
    use std::ops::Bound::{Excluded, Included};
    let s = "hello 1.73";
    // 1.73 起 (Bound<usize>, Bound<usize>) 实现 SliceIndex<str>
    let sub = &s[(Included(1), Excluded(5))];
    println!("  s[(Included(1), Excluded(5))] = {:?}", sub); // "ello"

    // ============================================================
    println!("\n4. const Weak::new 与 panic 消息说明");
    use std::rc::Weak;
    const W: Weak<String> = Weak::new(); // 1.73 起 const 可用
    println!("  const Weak::new() -> ptr = {:?}", W.as_ptr());
    println!("  - 默认 panic 消息格式更新：如 'panic occurred' 类消息更简洁");
    println!("  - assert_eq! 的 panic 消息更干净（带颜色的引导行）");

    // ============================================================
    println!("\n5. Arc<File> 的 Read/Write/Seek");
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    let f = File::open("Cargo.toml").expect("Cargo.toml 应存在（cwd=仓库根）");
    let af = std::sync::Arc::new(f);
    let mut buf = String::new();
    let mut af2 = af.clone();
    af2.seek(SeekFrom::Start(0)).ok();
    let n = af2.read_to_string(&mut buf).unwrap_or(0);
    println!("  Arc<File> 读取 {} 字节；克隆后共享同一句柄", n);

    // ============================================================
    println!("\n6. 文字说明");
    println!("  - extern \"thiscall\" ABI 稳定（x86 Windows 手动调用约定）");
    println!("  - io::Sink 功能并入 io::Empty；ExitStatus 实现 Default");
    println!("  - invalid_reference_casting 现在默认 deny");
}
