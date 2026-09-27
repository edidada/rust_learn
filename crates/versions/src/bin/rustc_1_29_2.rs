// rustc 1.29.2 演示 —— patch 版：LLVM 别名 bug 规避
// 该版本 introduces:
// 1. 规避 LLVM aliasing 相关 bug 造成的错误编译（miscompilation）。
// 2. 恢复 windows-gnu 目标的 rls-preview 组件。
// 无语言/库变化。

fn main() {
    println!("rustc 1.29.2 演示");
    println!("\n1. patch 版定位");
    println!("规避 LLVM aliasing bug 的 miscompilation；恢复 windows-gnu 的 rls-preview");
    println!("代码行为与 1.29.1 一致，仅编译器内部变化，无可演示 API");
}
