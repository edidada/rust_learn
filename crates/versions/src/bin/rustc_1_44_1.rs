// rustc 1.44.1 演示 —— patch 版 bugfix：rustfmt cfg_attr 嵌套属性 / Apple 可执行文件名 / macOS backtrace / Clippy lint 分布
// 说明：本版无新增语言/库特性，以下节对应笔记中的 Patch Fixes，描述修复内容。

fn main() {
    println!("rustc 1.44.1 演示（patch：仅修复，无新增稳定 API）");

    println!("\n1. rustfmt 再次接受 cfg_attr 中的 rustfmt_skip");
    // 历史：1.44.0 曾暂时不再接受 `#[cfg_attr(feature = "x", rustfmt::skip)]`
    // 这样的嵌套属性写法，导致部分代码被 rustfmt 误格式化或报错；本版恢复接受。
    // 该行为属 rustfmt 工具层面，运行期不可观察。
    println!("机制：允许在 cfg_attr 内嵌套 rustfmt::skip，被排除的项不被 rustfmt 重排。");
    println!("写法示例（不在此演示中运行）：#[cfg_attr(rustfmt, rustfmt_skip)] / #[rustfmt::skip]");

    println!("\n2. Apple 平台不再对可执行文件名做哈希");
    // 历史：此前 Apple 目标上的可执行文件名被加哈希后缀，调试器/backtrace 无法对应到符号；
    // 1.44.1 起不再哈希，backtrace 能正确解析。
    println!("机制：编译器对可执行文件名不再追加哈希，怀疑是 backtrace 解析不准的版本可升级。");

    println!("\n3. 修复 macOS 上查找 backtrace 的崩溃");
    // 背景：在 macOS 上当构建产物（dylib 等）变化时读取 backtrace 可能崩溃，本版修复。
    // 运行时可用 std::backtrace 尚未稳定，先用 println 说明行为变化即可。
    println!("机制：在 macOS 上发生 panic/错误时打印 backtrace 不再触发崩溃；");
    println!("即便 backtrace 不完整，也能稳定输出（本版是工具链 bugfix）。");

    println!("\n4. Clippy 将 lint 等级正确应用至不同文件");
    // 背景：此前一个 crate 内不同文件的 #[allow]/#[warn] 等 lint 属性可能被
    // Clippy 错误地跨文件应用，导致误报/漏报；本版修复。
    println!("机制：Clippy 与 rustc 的 lint 等级按文件隔离——一个文件里的");
    println!("#[allow(...)] 不会泄漏到另一个文件。");
}
