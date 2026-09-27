// rustc 1.57.0 演示 —— try_reserve、map_while、数组 as_slice、const panic
fn main() {
    println!("rustc 1.57.0 演示");

    // ============================================================
    println!("\n1. const 求值中允许 panic + TryFrom/TryInto 在 prelude");
    // (a) 1.57 起 const fn 内可 panic（编译期失败即报错，代替的是运行时 panic 语义）。
    // 示例：const 里越界取值会直接编译失败（此处仅展示机制说明，不真触发）。
    // (b) 2021 edition 起 TryFrom/TryInto 已在 prelude，无需 use std::convert::*。
    let n: u8 = u8::try_from(255i32).unwrap_or(0);
    println!("  u8::try_from(255) unwrap = {}", n);

    // ============================================================
    println!("\n2. Vec::try_reserve —— 容量预留失败不 abort");
    // 1.57 稳定 try_reserve 系列：内存不足时返回 TryReserveError 而不是 abort。
    // 对比旧行为：vec.reserve(超大) 内存不足会直接 abort 进程。
    let mut v: Vec<i32> = Vec::new();
    match v.try_reserve(1024) {
        Ok(_) => println!("  try_reserve(1024) 成功，capacity = {}", v.capacity()),
        Err(e) => println!("  try_reserve 失败 = {:?}", e),
    }
    // HashMap/String/VecDeque/HashSet 均有对应 *_reserve/_reserve_exact。

    // ============================================================
    println!("\n3. Iterator::map_while");
    // 1.57 稳定：逐元素 map，遇到谓词失败即提前结束。
    // 对比旧写法：map(...).take_while(...) 双重迭代器。
    let got: Vec<i32> = [1, 2, 3, -1, 5].into_iter().map_while(|x| {
        if x > 0 { Some(x) } else { None }
    }).collect();
    println!("  map_while(x>0) = {:?}", got);

    // ============================================================
    println!("\n4. [T; N]::as_slice / as_mut_slice");
    // 1.57 稳定：把数组一步拿 &[T]/&mut [T]。
    // 对比旧写法：&arr[..]。
    let arr = [1u8, 2, 3, 4];
    println!("  arr.as_slice() = {:?}", arr.as_slice());
    let mut arr2 = arr;
    arr2.as_mut_slice()[1] = 9;
    println!("  as_mut_slice 改后 = {:?}", arr2.as_slice());

    // ============================================================
    println!("\n5. 表达式里直接用花括号宏");
    // 1.57 起：m!{..}.method() 与 m!{..}? 合法（此前要 (m!{}).method()）。
    let v = vec![1u8, 2, 3];
    println!("  vec![1,2,3].len() = {}", v.len()); // vec! 即宏表达式成员访问的现成例子

    // ============================================================
    println!("\n6. #[must_use] 覆盖面扩大");
    println!("  - std 大量函数加 #[must_use]：忽略返回值告警。");
    println!("  - 典型：collect/iter 链被丢弃时会提示 - 注意 unbox/unextend 用法。");
}
