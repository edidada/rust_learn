// rustc 1.97.1 演示 —— patch：LLVM 优化 miscompilation 修复（记叙，无新 API）
fn main() {
    println!("rustc 1.97.1 演示（patch 版）");

    println!("\n1. LLVM 优化错误编译修复");
    println!("修复手段：");
    println!("1) 回退 LLVM submodule 升级，纳入 LLVM 侧的修复");
    println!("2) 回退一个已知触发该 bug 的 rustc 侧改动（谨慎起见，非严格必要）");
    // 无新 API；简单语义自检示意
    let v: Vec<u32> = (0..5).map(|x| x * 2).collect();
    println!("语义回归自检：{:?}", v);
}
