// rustc 1.45.0 演示 —— as 饱和转换 / char 范围迭代 / strip_prefix / 原子 fetch API / remove_entry
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    println!("rustc 1.45.0 演示");

    // 1.45.0 定义：越界的浮点→整数 `as` 是饱和转换（此前是 UB）。
    // "200.0 as u8" 在 1.45 之后是 255，而不再是任意垃圾值！
    println!("\n1. 浮点→整数 as 饱和转换");
    println!("300.5f32 as u8 = {}（饱和到 u8::MAX）", 300.5f32 as u8);
    println!("-1.0f32 as u8  = {}（饱和到下界 0）", (-1.0f32) as u8);
    println!("3.9f32 as u8   = {}（范围内，向零舍入）", 3.9f32 as u8);
    let n: i32 = unsafe { 3.9f64.to_int_unchecked() }; // 确认不越界时可继续用 unchecked 抢性能
    println!("3.9f64.to_int_unchecked::<i32>() = {}（性能路径，调用方保证不越界）", n);

    // 1.45.0 引入：char 可直接用于 Range 迭代（Range/RangeFrom/RangeFull/RangeInclusive/RangeTo）。
    println!("\n2. char 迭代范围");
    let mut s = String::new();
    for ch in 'a'..='z' {
        s.push(ch);
    }
    println!("'a'..='z' 迭代拼接 = {}", s);
    let rev: String = ('z'..='a').rev().collect(); // 范围反向
    println!("字符范围反向也 OK：前 3 个 = {:?}", rev.chars().take(3).collect::<Vec<_>>());

    // 1.45.0 稳定：str::strip_prefix / strip_suffix。
    println!("\n3. strip_prefix / strip_suffix");
    let line = "rust: hello";
    println!("strip_prefix(\"rust:\") = {:?}", line.strip_prefix("rust:").map(str::trim));
    println!("strip_prefix(\"rust:\") 不含时 = {:?}", "cargo: bye".strip_prefix("rust:"));
    println!("strip_suffix(\"!\") = {:?}", "value!".strip_suffix('!'));

    // 1.45.0 引入：原子整数 fetch_min/fetch_max/fetch_update。
    println!("\n4. 原子 fetch_min/fetch_max/fetch_update");
    let a = AtomicI32::new(10);
    a.fetch_min(3, Ordering::SeqCst); // 取 10 与 3 的最小值
    println!("fetch_min(3) 后 = {}", a.load(Ordering::SeqCst));
    a.fetch_max(20, Ordering::SeqCst);
    println!("fetch_max(10) 后 = {}", a.load(Ordering::SeqCst));
    let old = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| Some(v * 2));
    println!("fetch_update(×2) 返回旧值 = {:?}，现值 = {}",
        old, a.load(Ordering::SeqCst));
    // fetch_update 闭包返回 None 则不更新（如要求偶数）:
    let odd = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v % 2 == 0 { None } else { Some(v) });
    println!("对偶数返回 None：fetch_update 结果 = {:?}（不更新）", odd);

    // 1.45.0 稳定：BTreeMap::remove_entry、Rc::as_ptr。
    println!("\n5. BTreeMap::remove_entry / Rc::as_ptr");
    let mut m = BTreeMap::new();
    m.insert(1u8, "a");
    let entry = m.remove_entry(&1);
    println!("remove_entry(&1) = {:?}（key+value 一起移出）", entry);
    let rc = std::rc::Rc::new(5u8);
    let p: *const u8 = std::rc::Rc::as_ptr(&rc); // 直接获得底层裸指针
    println!("Rc::as_ptr 指向地址处数据 = {}", unsafe { *p });
}
