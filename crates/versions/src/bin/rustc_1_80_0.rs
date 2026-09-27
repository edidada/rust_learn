// rustc 1.80.0 演示 —— LazyLock/LazyCell、take_if、split_at_checked、Box<[T]> into_iter、prelude size_of
// 注意：本仓库以 rustc 1.98 / edition 2024 运行；1.80 稳定的 API 在 1.98 均可用。
use std::cell::LazyCell;
use std::collections::HashMap;
use std::sync::LazyLock;

// 1.80 稳定 LazyLock：全局静态的线程安全惰性初始化。
// 旧行为：要么手写 once_cell::sync::Lazy（外部 crate），要么 OnceLock + 手写闭包。
// 现在 static 一行搞定，首次解引用时执行初始化、之后直接读缓存的值。
static CONFIG: LazyLock<HashMap<&'static str, i32>> = LazyLock::new(|| {
    println!("  （CONFIG 真正初始化——只发生一次）");
    HashMap::from([("threads", 4), ("cache", 128)])
});

fn lookup(key: &str) -> Option<i32> {
    CONFIG.get(key).copied() // 解引用 LazyLock，触发惰性初始化
}

fn main() {
    println!("rustc 1.80.0 演示");

    println!("\n1. LazyLock —— 线程安全静态惰性初始化（1.80 重点）");
    println!("第一次 lookup(\"threads\") …");
    assert_eq!(lookup("threads"), Some(4)); // 这里才初始化
    println!("threads = {}", lookup("threads").unwrap());
    println!("第二次 lookup(\"cache\")（不再打印初始化信息）…");
    println!("cache = {}", lookup("cache").unwrap());

    println!("\n2. LazyCell —— 单线程惰性初始化（无同步开销，!Sync）");
    let cell: LazyCell<Vec<i32>, _> = LazyCell::new(|| vec![10, 20]);
    // 首次 Deref 时初始化：
    println!("LazyCell 首次访问 = {:?}", &*cell);
    let _ = cell;
    let c2 = LazyCell::new(|| "lazy-value".to_string());
    println!("LazyCell(单线程) = {}", &*c2); // Deref 触发初始化
    println!("LazyCell 再次访问 = {:?}", *c2);

    println!("\n3. Option::take_if（1.80 稳定）");
    let mut opt = Some(26);
    // 满足条件才取走：
    let taken = opt.take_if(|v| *v > 25);
    println!("take_if(>25) 取走 = {taken:?}, 剩余 = {opt:?}");
    let mut opt2 = Some(10);
    let taken2 = opt2.take_if(|v| *v > 25);
    println!("take_if(>25) 取走 = {taken2:?}, 剩余 = {opt2:?}");

    println!("\n4. <[T]>::split_at_checked —— 越界返回 None 而不是 panic");
    let arr = [1, 2, 3, 4, 5];
    let (a, b) = arr.split_at_checked(2).unwrap();
    println!("split(2) 前 = {a:?}, 后 = {b:?}");
    println!("split(9) 越界 = {:?}", arr.split_at_checked(9));

    println!("\n5. impl IntoIterator for Box<[T]>（1.80 稳定）");
    let boxed: Box<[i32]> = vec![7, 8, 9].into();
    let mut sum = 0;
    for x in boxed {
        sum += x; // Box<[T]> 直接 into_iter，无需 &* 或 .iter()
    }
    println!("Box<[T]> 求和 = {sum}");

    println!("\n6. prelude 中的 size_of / align_of（无需 std::mem:: 前缀）");
    println!("size_of::<u64>() = {}; align_of::<u64>() = {}", size_of::<u64>(), align_of::<u64>());
}
