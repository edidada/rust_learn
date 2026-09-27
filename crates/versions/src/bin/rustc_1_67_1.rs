// rustc 1.67.1 演示 —— patch 版本：thin archives 修复 + clippy uninlined_format_args 降级
// 本版本是构建工具/编译器内部修复，无法在普通代码运行演示，仅文字说明。
fn main() {
    println!("rustc 1.67.1 演示");
    println!("\n1. Changes");
    println!("  - 修复与 thin archives 的互操作问题（构建/链接侧）");
    println!("  - 修复编译器构建过程中的一个内部错误");
    println!("  - clippy::uninlined_format_args 降级为 pedantic：");
    println!("    以前 format!(\"{{}}\", x) 默认建议改写为 format!(\"{{x}}\")，");
    println!("    1.67.1 起该建议只在 pedantic 级别下出现，减少日常噪音。");
}
