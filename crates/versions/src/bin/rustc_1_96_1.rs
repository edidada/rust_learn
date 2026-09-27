// rustc 1.96.1 演示 —— patch：Cargo 超时/重试、libssh2 CVE 补丁、MIR 优化错误编译修复（记叙）
fn main() {
    println!("rustc 1.96.1 演示（patch 版）");

    println!("\n1. Cargo 超时/重试行为修复");
    println!("网络请求 timeout 与 retry 语义修正（registry 拉取场景）");

    println!("\n2. libssh2 CVE 补丁（Cargo 侧）");
    println!("CVE-2025-15661 / CVE-2026-55199 / CVE-2026-55200 —— 依赖库安全修复");

    println!("\n3. rustc MIR 优化错误编译修复");
    // miscompilation 意为"生成机器码与源语义不符"；本 patch 无新 API，仅修后端正确性
    let x: u32 = 5;
    let y = x.wrapping_mul(1);
    println!("简单语义回归验证：wrapping_mul(1) = {y}（应等于 5）");
}
