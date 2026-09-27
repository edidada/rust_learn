// rustc 1.58.1 演示 —— 补丁版：remove_dir_all 竞态条件修复
// 本版无语言/库特性变化，只有安全修复与工具修复；按规范用 println 讲清。
fn main() {
    println!("rustc 1.58.1 演示（补丁版）");

    println!("\n1. std::fs::remove_dir_all 竞态修复（CVE-2022-21658）");
    println!("  - 旧实现存在符号链接竞态：并发替换目录内容时可能删掉非目标路径。");
    println!("  - 修复后 remove_dir_all 不再跟随攻击者放入的符号链接。");
    println!("  - 建议尽快升级；本 patch 不改变任何稳定 API 语义。");
}
