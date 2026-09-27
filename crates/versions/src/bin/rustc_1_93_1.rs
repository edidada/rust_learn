// rustc 1.93.1 演示 —— patch：编译器 ICE 修复、clippy 误报修复、wasip2 FD 泄漏回滚（记叙，无新 API）
fn main() {
    println!("rustc 1.93.1 演示（patch 版）");

    println!("\n1. 关键字恢复 ICE 修复（影响 rustfmt）");
    println!("修复前：rustfmt 遇到关键字上下文会 ICE；1.93.1 不再尝试把关键字恢复成普通标识符");

    println!("\n2. clippy::panicking_unwrap 误报修复");
    // 对 s.field.unwrap() 这种经由 Deref 的字段访问曾误报"可能 panic"；
    // 本仓库演示环境不运行 clippy，仅记录
    let s = String::from("abc");
    let b = (&s, 1usize);
    let _ = b.1.checked_add(1);
    println!("字段访问 + checked 运算：无误报场景");

    println!("\n3. wasm32-wasip2 文件描述符泄漏回滚");
    println!("回滚某次 wasm 相关 CI 依赖更新，修 wasip2 目标 FD 泄漏");
}
