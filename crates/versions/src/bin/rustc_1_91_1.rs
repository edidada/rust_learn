// rustc 1.91.1 演示 —— patch：illumos 文件锁、wasm_import_module 跨 crate 修复（记叙，无新 API）
fn main() {
    println!("rustc 1.91.1 演示（patch 版）");

    println!("\n1. illumos 文件锁支持");
    // 本 patch 为 illumos 实现 File::lock/try_lock 等平台后端，
    // 使 Cargo 在 illumos 上能正确锁定 build 目录；
    // 与 rustc 本体语言特性无关，仅记录。
    println!("illumos 上 File::lock 家族可用 → Cargo build dir 锁修复");

    println!("\n2. wasm_import_module 跨 crate");
    // #[wasm_import_module] 用于 extern 块指定导入模块；
    // 1.91.0 中跨 crate 引用时该属性信息丢失，导致 wasm 链接报错，1.91.1 修复。
    println!("跨 crate 的 #[wasm_import_module] 链接修复（wasm 目标场景）");
}
