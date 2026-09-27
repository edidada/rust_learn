// rustc 1.34.0 演示 —— TryFrom/TryInto + iter::from_fn/successors + AtomicI8
// 该版本 introduces:
// 1) convert::TryFrom / TryInto / Infallible 稳定：可失败转换的标准 trait。
//    （f64::try_from(u32) 在 1.34/1.35 仍返回 Result，1.36 起标弃用并改 Infallible。）
// 2) iter::from_fn / successors：从闭包/前驱生成迭代器。
// 3) str::escape_* 三个转义 API 稳定。
// 4) 新原子类型 AtomicI8/I16/I32/I64/U8/U16/U32/U64。
// 5) ATOMIC_*_INIT 弃用：static 中可直接用 const fn 构造。

use std::convert::{Infallible, TryFrom};

fn main() {
    println!("rustc 1.34.0 演示");

    // 1. TryFrom / TryInto
    println!("\n1. TryFrom / TryInto / Infallible");
    let smaller: Result<u8, _> = u8::try_from(20u16);
    println!("u8::try_from(20u16) = {:?}", smaller);
    let big: Result<u8, _> = u8::try_from(300u16);
    println!("u8::try_from(300u16) = {:?}", big);
    let t: u64 = 5u8.try_into().unwrap();
    println!("u8 --TryInto--> u64 = {}", t);
    // Infallible：永远不发生的错误类型
    let infallible: Result<i32, Infallible> = Ok(1);
    println!("Infallible 示例: {:?}", infallible);

    // 2. iter::from_fn / successors
    println!("\n2. iter::from_fn / successors");
    let counter = std::iter::from_fn(|| {
        Some(1)
    });
    let _ = counter; // 无状态演示，直接讲用法
    let mut n = 1u32;
    let seq: Vec<u32> = std::iter::successors(Some(n), |&x| {
        if x < 20 { Some(x * 2) } else { None }
    })
    .collect();
    println!("successors 倍增序列 = {:?}", seq);

    // 3. str::escape_*
    println!("\n3. str::escape_*");
    println!("escape_default(\"a\\tb\") = {:?}", "a\tb".escape_default().to_string());
    println!("escape_unicode(\"R\") = {:?}", "R".escape_unicode().to_string());
    println!("escape_debug(\"q\") = {:?}", "q".escape_debug().to_string());

    // 4. 新原子类型
    println!("\n4. AtomicI8 / AtomicU64");
    let a = std::sync::atomic::AtomicI8::new(-1);
    a.fetch_add(3, std::sync::atomic::Ordering::SeqCst);
    println!("AtomicI8(-1) fetch_add(3) = {}", a.load(std::sync::atomic::Ordering::SeqCst));
    let b = std::sync::atomic::AtomicU64::new(1);
    b.fetch_or(0xFF, std::sync::atomic::Ordering::SeqCst);
    println!("AtomicU64 fetch_or(0xFF) = {}", b.load(std::sync::atomic::Ordering::SeqCst));

    // 5. slice::sort_by_cached_key（key 函数只算一次）
    println!("\n5. sort_by_cached_key");
    let mut words = ["ccc", "a", "bb"];
    words.sort_by_cached_key(|w| w.len());
    println!("{:?}", words);

    // 6. NonZeroI*（带符号非零类型）
    println!("\n6. NonZeroI32");
    let nz = std::num::NonZeroI32::new(-3);
    println!("NonZeroI32::new(-3) = {:?}", nz);

    // 7. 弃用讲解
    println!("\n7. ATOMIC_*_INIT 弃用");
    // 旧写法（1.34 起弃用，仅示意）：static X: AtomicBool = ATOMIC_BOOL_INIT;
    // 新写法：static X: AtomicBool = AtomicBool::new(false); —— 1.24 const fn 后即可用
    println!("static 初始化原子请用 AtomicBool::new(false)（const fn），不要用 ATOMIC_BOOL_INIT");
}
