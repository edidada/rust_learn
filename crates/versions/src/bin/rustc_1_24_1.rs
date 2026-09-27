// rustc 1.24.1 演示 —— patch 版：FFI unwind 行为回调
// 该版本 introduces:
// 1.24.0 曾规定 "unwinding 不会穿越 FFI 边界而是 abort"，
// 1.24.1 取消了这个过于激进的变化：不再在通过 FFI 展开时直接 abort。
// 其他修复均与 Windows 工具链及文档工具相关，无语言/库变化。

fn main() {
    println!("rustc 1.24.1 演示");
    println!("\n1. patch 版定位");
    println!("1.24.1 修复：unwinding 穿越 FFI 时的 abort 行为（1.24.0 过严）、Windows UTF-16 链接器参数等");
    println!();
    // 顺手演示 1.24 的 const fn 能力仍在
    let buffer: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
    println!("沿用 1.24 能力：size_of::<usize>() 数组长度 = {}", buffer.len());
}
