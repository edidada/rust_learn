// rustc 1.52.1 演示 —— 补丁版本：默认关闭增量编译的说明
// 本版无语言/库变化，只有编译器行为调整；按规范用 println 讲清机制。
fn main() {
    println!("rustc 1.52.1 演示（补丁版）");

    println!("\n1. 默认关闭增量编译");
    println!("  - 1.52.0 开启的增量编译校验触发大范围破坏；本补丁默认关闭增量编译。");
    println!("  - 仅当设置 RUSTC_FORCE_INCREMENTAL=1 时才会再次启用。");
    println!("  - 受影响：debug / check 构建（增量）；release 构建通常不受影响。");
    println!("  - 1.52.0 引入的校验能查出旧版本同样存在的 bug（未修复时可能误编译）。");
}
