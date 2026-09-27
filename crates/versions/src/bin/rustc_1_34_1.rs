// rustc 1.34.1 演示 —— patch 版：Clippy 误报修复
// 该版本 introduces:
// 1. 修复 Clippy redundant_closure lint 误报。
// 2. 修复 Clippy missing_const_for_fn lint 误报。
// 3. 修复 Clippy 对部分宏的 panic。
// 无语言/库变化。

fn main() {
    println!("rustc 1.34.1 演示");
    println!("\n1. patch 版定位");
    println!("修复 Clippy 三处误报/panic：redundant_closure、missing_const_for_fn、宏检查 panic");
    println!("语言本体与 1.34.0 一致，以下复检 TryInto 可用：");
    let t: u64 = 7u8.try_into().unwrap();
    println!("7u8 -> u64 = {}", t);
}
