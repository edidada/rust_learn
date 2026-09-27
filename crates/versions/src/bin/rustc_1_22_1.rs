// rustc 1.22.1 演示 —— patch 版：仅 Cargo 更新（修复 macOS 10.13 问题）
// 该版本 introduces:
// 1.22.1 是随 1.22.0 同日发布的补丁版，仅更新 Cargo 工具本身：
//   - 修复 macOS 10.13 “High Sierra” 上 Cargo 遇到的问题
// 语言与标准库无任何变化，因此本演示只用 println 讲解版本定位。

fn main() {
    println!("rustc 1.22.1 演示");
    println!("\n1. patch 版定位");
    println!("1.22.1 = 1.22.0 + Cargo 修复（macOS 10.13 High Sierra 兼容问题）");
    println!("rustc 语言/std 行为与 1.22.0 完全一致：x += &8 与 Option 上的 ? 均可用");
    let mut x = 2;
    x += &8;
    println!("顺手验证 1.22 语法仍在：x = {}", x);
}
