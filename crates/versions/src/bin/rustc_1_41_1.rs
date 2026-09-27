// rustc 1.41.1 演示 —— patch 版 bugfix：static 类型检查 / Copy 生命周期约束 / Layout::repeat 误编译
// 说明：本版无新增语言/库特性，以下节对应笔记中的 Patch Fixes，用现行稳定写法演示“修复保证的正确行为”。

// 1.41.1 修复背景：static 项的类型此前可能漏检，现在编译器总是检查。
static ANSWER: u32 = 40 + 2; // 类型与初始化表达式始终一致，1.41.1 起必查
static GREET: &str = "hello"; // &str 是 Copy；引用类型的 static 也照常编译通过

// 1.41.1 修复背景：Copy impl 的生命周期约束（T: 'a 类界）此前可能漏检，
// 例如实现 impl<C: Copy> Trait for Cow<'_, C> 时 C 需要满足生命周期界，现在总是检查。
// 这里用等价的稳定形态演示：带生命周期引参的 Copy 结构体。
#[derive(Clone, Copy, Debug)]
struct Wrapped<'a> {
    text: &'a str, // Copy<'a> 要求 'a 满足约束，编译期强制检查
}

fn main() {
    println!("rustc 1.41.1 演示（patch：仅修复，无新增稳定 API）");

    println!("\n1. 总是检查 static 项的类型");
    println!("static ANSWER: u32 = {}", ANSWER);
    println!("static GREET: &str = {:?}", GREET);
    // 反例（编译期即报错，无法在演示中运行）：
    // static BAD: u32 = "not a u32";  // 1.41.1 之前可能完全未经检查就编译，现在保证报错

    println!("\n2. 总是检查 Copy impl 的生命周期约束");
    let w = Wrapped { text: "copy me" };
    let w2 = w; // Wrapped<'a>: Copy，移动后原值仍可用——约束检查由编译器保证
    println!("Copy 后两份都在：{:?} / {:?}", w, w2);

    println!("\n3. 调用 Layout::repeat 的误编译已修复");
    // 该版本修复的是编译器 codegen 层 bug：此前某些调用 Layout::repeat 的代码
    // 会生成错误机器码（miscompilation），导致重复布局算出错误结果；API 行为无变化。
    // 历史/现行形态：Layout::repeat(元素 Layout, 重复次数) 返回 (重复后的总 Layout, 元素步长)。
    println!("机制：Layout::repeat(layout, n) 按 n 份布局拼接（含 padding）并返回 (Layout, stride)。");
    println!("1.41.1 修复后：调用者不再产生错误机器码，分配器网络的布局计算恢复正确。");
    println!("（这是修复项，无运行时可观察的新行为，故不加实际 repeat 调用以免猜签名）");

    println!("\n4. 32 位 Apple 目标的发布说明");
    // 1.41.0 官方曾宣布是最后一个为 tier 1/2 的 32 位 Apple 目标（如 i386-apple-darwin 等）
    // 出二进制的版本；但 patch 发布未料到会存在——1.41.1 仍包含这些目标的发布二进制。
    println!("1.41.1 仍为 tier 1/2 的 32 位 Apple 目标提供发布二进制（补丁目标照常可用）。");
}
