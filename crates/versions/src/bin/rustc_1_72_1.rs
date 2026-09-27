// rustc 1.72.1 演示 —— patch 版本：LLVM codegen 调整、编译时间回归修复等（文字说明）
fn main() {
    println!("rustc 1.72.1 演示");
    println!("\n1. Changes（编译器/rustdoc 侧修复）");
    println!("  - 调整 codegen 改动以改善 LLVM 代码生成");
    println!("  - rustdoc: 修复带生命周期对象的 self 类型参数");
    println!("  - 修复 1.72.0 引入的编译时间回归");
    println!("  - 修复 ICE 回归：#115215、#115559");
}
