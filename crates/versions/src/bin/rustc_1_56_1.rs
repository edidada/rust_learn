// rustc 1.56.1 演示 —— 补丁版：Trojan Source（CVE-2021-42574）防护 lint
// 本版无语言/库变化，只有编译器 lint 增强；按规范用 println 讲清机制。
fn main() {
    println!("rustc 1.56.1 演示（补丁版）");

    println!("\n1. 双向覆盖 Unicode 码点检测（Trojan Source）");
    println!("  - 新 lint 检测源码中的 U+202E 等双向覆盖码点（CVE-2021-42574）。");
    println!("  - 这些字符能在编辑器显示层隐藏真实控制流（视觉欺骗攻击）。");
    println!("  - 正常写代码不需处理；若库源被注入这些字符会得到编译告警。");
    println!("  - 本文件刻意不包含任何该类字符。");
}
