// rustc 1.77.1 演示 —— patch 版本：撤销 Windows 默认剥离 debuginfo（文字说明）
fn main() {
    println!("rustc 1.77.1 演示");
    println!("\n1. Revert: Windows 默认剥离 debuginfo");
    println!("  - 1.77.0 曾在未请求 debuginfo 时默认剥离所有 debuginfo，");
    println!("    在 Windows 上引发回归；1.77.1 撤销该默认行为，恢复原状。");
    println!("  - 其它平台不受影响；内部修复文档页标题锚点渲染。");
}
