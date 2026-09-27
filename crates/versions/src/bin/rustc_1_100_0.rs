// rustc 1.100.0 演示 —— nightly 版记叙（页面尚未定稿；工具链 1.98，不调用任何 1.100 新 API）
use std::ops::ControlFlow;

fn main() {
    println!("rustc 1.100.0 演示（nightly 未发布，仅记叙）");

    println!("\n1. never type 稳定里程碑");
    println!("!（never type）自 2016 年起一直是 unstable；1.100 稳定它 —— 最终可写 fn bomb() -> !");
    // 历史形态（1.98 中可用的是有限的 never 用法）：
    let cf: ControlFlow<(), ()> = ControlFlow::Break(());
    println!("1.98 可用近似物：ControlFlow/Infallible；真正的 fn bomb() -> ! {} 需 1.100", "稳定后");

    println!("\n2. 库侧稳定项（记叙）");
    println!("Allocator trait 稳定：自定义分配器 trait（曾经 10 年 unstable）");
    println!("bool::toggle 稳定：self 上的 ! 取反");
    println!("Box 上的 impl 泛化；整数 min/max intrinsics");

    println!("\n3. 平台侧（记叙）");
    println!("wasm32-wasip3 升 Tier 2；新增 powerpc64-sony-ps3 Tier 3");
    println!("x86 SSE 目标用 SSE 寄存器做 ABI（FnABI 变化）");
}
