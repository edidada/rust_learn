// rustc 1.101.0 演示 —— 未发布版记叙（页面仅 2 条 PR；工具链 1.98，不调用任何 1.101 新 API）
fn main() {
    println!("rustc 1.101.0 演示（未发布，仅记叙）");

    println!("\n1. RawWakerVTable 8 字节对齐保证");
    println!("Waker/VTable 的内存布局增加对齐承诺，利于 FFI 与 ABI 稳定");

    println!("\n2. SyncView 稳定");
    println!("接续 1.98 Atomic view 家族（from_mut/get_mut_slice）：SyncView 提供同步访问视图");
    println!("1.98 近似物：std::sync::atomic::Atomic*::from_mut 系（本工具链可用）");
}
