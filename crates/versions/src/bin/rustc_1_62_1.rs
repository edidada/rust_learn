// rustc 1.62.1 演示 —— 补丁版：回归修复与 SGX 漏洞缓解
// 本版无语言/库特性变化；按规范用 println 讲清。
fn main() {
    println!("rustc 1.62.1 演示（补丁版）");

    println!("\n1. 编译器回归与 SGX 漏洞缓解");
    println!("  - 修复 impl Trait 返回类型相关的不健全函数强转。");
    println!("  - 修复 async fn 生命周期的增量编译 bug。");
    println!("  - Windows 同步读写退回同步 fallback。");
    println!("  - x86_64-fortanix-unknown-sgx 缓解 MMIO 旧数据漏洞 INTEL-SA-00615。");
}
